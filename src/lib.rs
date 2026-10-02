//! # Quatt CPU & RAM Library
//!
//! A library for simulating a 2-bit (Quat) based CPU and RAM architecture.
//! Each Quat represents a base-4 digit stored using 2 binary bits (00, 01, 10, 11).

mod cpu;
mod error;
mod quat;
mod ram;
mod ffi;

pub use cpu::QuatCpu;
pub use error::CpuError;
pub use quat::Quat;
pub use ram::{QuatRam, StaticQuatRam};
use crate::ffi::*;

#[cfg(test)]
mod tests {

    use crate::ffi::*;
    use super::*;
    
    #[test]
    fn test_quat_conversion() {
        let q = Quat::from_bits(1, 0).unwrap();
        assert_eq!(q, Quat::Q2);
        assert_eq!(q.to_bits(), (1, 0));
        assert_eq!(q.value(), 2);
    }

    #[test]
    fn test_cpu_add_and_carry() {
        let mut cpu = QuatCpu::new();
        cpu.load(0, 1, 1).unwrap(); // reg[0] = Q3 (3)
        cpu.load(1, 0, 1).unwrap(); // reg[1] = Q1 (1)

        let carry = cpu.add(0, 1, 2).unwrap(); // 3 + 1 = 4 -> 0 mod 4, carry = true
        assert_eq!(cpu.registers[2], Quat::Q0);
        assert!(carry);
    }

    #[test]
    fn test_ram_packing() {
        let mut ram = QuatRam::new(10);
        ram.write_quat(0, 3).unwrap();
        ram.write_quat(1, 1).unwrap();
        ram.write_quat(2, 2).unwrap();
        ram.write_quat(3, 0).unwrap();

        assert_eq!(ram.read_quat(0).unwrap(), Quat::Q3);
        assert_eq!(ram.read_quat(1).unwrap(), Quat::Q1);
        assert_eq!(ram.read_quat(2).unwrap(), Quat::Q2);
        assert_eq!(ram.read_quat(3).unwrap(), Quat::Q0);

        // Verify overwriting a quat does not corrupt neighboring quats in the same byte
        ram.write_quat(1, 2).unwrap();
        assert_eq!(ram.read_quat(0).unwrap(), Quat::Q3);
        assert_eq!(ram.read_quat(1).unwrap(), Quat::Q2);
        assert_eq!(ram.read_quat(2).unwrap(), Quat::Q2);
    }

    #[test]
    fn test_quat_bitwise_ops_and_display() {
        let q1 = Quat::Q3; // 11
        let q2 = Quat::Q1; // 01

        assert_eq!(q1 & q2, Quat::Q1);
        assert_eq!(q1 | q2, Quat::Q3);
        assert_eq!(q1 ^ q2, Quat::Q2);
        assert_eq!(!q2, Quat::Q2);

        assert_eq!(format!("{}", Quat::Q2), "10");
    }

    #[test]
    fn test_cpu_logic_ops() {
        let mut cpu = QuatCpu::new();
        cpu.load(0, 1, 1).unwrap(); // reg[0] = Q3 (11)
        cpu.load(1, 0, 1).unwrap(); // reg[1] = Q1 (01)

        cpu.and_regs(0, 1, 2).unwrap();
        assert_eq!(cpu.registers[2], Quat::Q1);

        cpu.not_reg(1, 3).unwrap();
        assert_eq!(cpu.registers[3], Quat::Q2);
    }

    #[test]
    fn test_static_ram() {
        let mut s_ram = StaticQuatRam::<4>::new(8).unwrap(); // 4 bytes for 8 quats
        s_ram.write_quat(5, Quat::Q3).unwrap();
        assert_eq!(s_ram.read_quat(5).unwrap(), Quat::Q3);
    }

    #[test]
    fn test_quat_add_sub_operators() {
        let q1 = Quat::Q2;
        let q2 = Quat::Q1;

        // Test Operator + (Add)
        assert_eq!(q1 + q2, Quat::Q3);
        // Test Overflow (3 + 1 = 4 -> 0 mod 4)
        assert_eq!(Quat::Q3 + Quat::Q1, Quat::Q0);

        // Test Operator - (Sub)
        assert_eq!(q1 - q2, Quat::Q1);
        // Test Underflow (0 - 1 = -1 -> 3 mod 4)
        assert_eq!(Quat::Q0 - Quat::Q1, Quat::Q3);
    }

    #[test]
    fn test_cpu_sub_and_borrow() {
        let mut cpu = QuatCpu::new();
        // Load Q1 into reg[0] and Q3 into reg[1]
        cpu.load(0, 0, 1).unwrap();
        cpu.load(1, 1, 1).unwrap();

        // 1 - 3 = -2 -> 2 mod 4, with borrow = true
        let borrow = cpu.sub(0, 1, 2).unwrap();
        assert_eq!(cpu.registers[2], Quat::Q2);
        assert!(borrow, "Borrow should be true on underflow");
    }

    #[test]
    fn test_stress_million_quats_ram() {
        let count: usize = 1_000_000;
        let mut ram = QuatRam::new(count);

        // Fill 1,000,000 quats with alternating pattern
        for i in 0..count {
            let val = match i % 4 {
                0 => Quat::Q0,
                1 => Quat::Q1,
                2 => Quat::Q2,
                _ => Quat::Q3,
            };
            ram.write_quat(i, val)
                .expect("Failed to write quat during stress test");
        }

        assert_eq!(ram.capacity(), count);

        // Verify boundary and random elements
        assert_eq!(ram.read_quat(0).unwrap(), Quat::Q0);
        assert_eq!(ram.read_quat(500_001).unwrap(), Quat::Q1);
        assert_eq!(ram.read_quat(999_999).unwrap(), Quat::Q3);
    }

    #[test]
    fn test_cpu_ram_execution_cycle() {
        let mut ram = QuatRam::new(4);
        let mut cpu = QuatCpu::new();

        // Write instructions to RAM
        ram.write_quat(0, Quat::Q1).unwrap();
        ram.write_quat(1, Quat::Q3).unwrap();

        // Set initial values in registers
        cpu.registers[0] = Quat::Q1;
        cpu.registers[1] = Quat::Q2;

        // Cycle 1: Executes Q1 (ADD R0 + R1 -> R0)
        cpu.step(&ram).unwrap();
        assert_eq!(cpu.registers[0], Quat::Q3);
        assert_eq!(cpu.pc, 1);
        assert_eq!(cpu.zero_flag, false);

        // Cycle 2: Executes Q3 (RESET PC)
        cpu.step(&ram).unwrap();
        assert_eq!(cpu.pc, 0);
    }

    // ==========================================
    // NEWLY ADDED PRODUCTION TESTS
    // ==========================================

    #[test]
    fn test_out_of_bounds_errors() {
        let mut ram = QuatRam::new(4);
        assert_eq!(ram.read_quat(4), Err(CpuError::InvalidAddress));
        assert_eq!(ram.write_quat(10, Quat::Q1), Err(CpuError::InvalidAddress));

        let mut cpu = QuatCpu::new();
        assert_eq!(cpu.load(4, 0, 1), Err(CpuError::InvalidRegister));
        assert_eq!(cpu.add(0, 5, 1), Err(CpuError::InvalidRegister));
    }

    #[test]
    fn test_ffi_interface() {
        unsafe {
            // RAM FFI test
            let ram_ptr = quat_ram_new(8);
            assert!(!ram_ptr.is_null());

            let res_write = quat_ram_write(ram_ptr, 2, CQuat::Q2);
            assert_eq!(res_write, 0);

            let mut out_quat = CQuat::Q0;
            let res_read = quat_ram_read(ram_ptr, 2, &mut out_quat);
            assert_eq!(res_read, 0);
            assert_eq!(out_quat, CQuat::Q2);

            // CPU FFI test
            let cpu_ptr = quat_cpu_new();
            assert!(!cpu_ptr.is_null());

            let res_step = quat_cpu_step(cpu_ptr, ram_ptr);
            assert_eq!(res_step, 0);

            quat_ram_free(ram_ptr);
            quat_cpu_free(cpu_ptr);
        }
    }
}