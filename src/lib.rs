//! # Quatt CPU & RAM Library
//!
//! A library for simulating a 2-bit (Quat) based CPU and RAM architecture.
//! Each Quat represents a base-4 digit stored using 2 binary bits (00, 01, 10, 11).

pub mod cpu;
pub mod error;
pub mod quat;
pub mod ram;
pub mod ffi;
pub mod disassembler;
pub mod assembler;
pub mod multicore;

pub use cpu::QuatCpu;
pub use error::CpuError;
pub use quat::Quat;
pub use ram::{QuatRam, StaticQuatRam};

use crate::multicore::MultiCoreSystem;
    use crate::assembler::Assembler;
    use crate::ffi::*;


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use std::mem;

    #[test]
    fn test_quat_conversion() {
        let iterations = 1_000_000;
        let start = Instant::now();
        for _ in 0..iterations {
            let q = Quat::from_bits(1, 0).unwrap();
            let _ = q.to_bits();
            let _ = q.value();
        }
        let duration = start.elapsed();
        
        println!("\n[METRICS: Quat Conversion]");
        println!("  Iterations:     {}", iterations);
        println!("  Total time:     {:?}", duration);
        println!("  Throughput:     {:.2} conversions/sec", iterations as f64 / duration.as_secs_f64());

        let q = Quat::from_bits(1, 0).unwrap();
        assert_eq!(q, Quat::Q2);
        assert_eq!(q.to_bits(), (1, 0));
        assert_eq!(q.value(), 2);
    }

    #[test]
    fn test_cpu_add_and_carry() {
        let mut cpu = QuatCpu::new();
        cpu.load(0, 1, 1).unwrap(); 
        cpu.load(1, 0, 1).unwrap(); 

        let iterations = 500_000;
        let start = Instant::now();
        let mut carry_count = 0;
        for _ in 0..iterations {
            if cpu.add(0, 1, 2).unwrap() {
                carry_count += 1;
            }
        }
        let duration = start.elapsed();

        println!("\n[METRICS: CPU Add & Carry Evaluation]");
        println!("  Operations:     {}", iterations);
        println!("  Carry triggers: {}", carry_count);
        println!("  Throughput:     {:.2} ops/sec", iterations as f64 / duration.as_secs_f64());

        let carry = cpu.add(0, 1, 2).unwrap(); 
        assert_eq!(cpu.registers[2], Quat::Q0);
        assert!(carry);
    }

    #[test]
    fn test_ram_packing() {
        let mut ram = QuatRam::new(10);
        
        // Measure bit-packing memory efficiency
        let raw_capacity = ram.capacity();
        let actual_bytes_allocated = mem::size_of_val(&ram) + (raw_capacity / 4);
        let uncompressed_size = raw_capacity * mem::size_of::<Quat>();
        let compression_ratio = 1.0 - (actual_bytes_allocated as f64 / uncompressed_size as f64);

        println!("\n[METRICS: RAM Bit-Packing & Compression]");
        println!("  Logical Quats:        {}", raw_capacity);
        println!("  Uncompressed size:    {} bytes", uncompressed_size);
        println!("  Packed memory size:   ~{} bytes", actual_bytes_allocated);
        println!("  Memory savings ratio: {:.1}%", compression_ratio * 100.0);

        ram.write_quat(0, 3).unwrap();
        ram.write_quat(1, 1).unwrap();
        ram.write_quat(2, 2).unwrap();
        ram.write_quat(3, 0).unwrap();

        assert_eq!(ram.read_quat(0).unwrap(), Quat::Q3);
        assert_eq!(ram.read_quat(1).unwrap(), Quat::Q1);
        assert_eq!(ram.read_quat(2).unwrap(), Quat::Q2);
        assert_eq!(ram.read_quat(3).unwrap(), Quat::Q0);

        // Verify isolation safety (neighboring quats protection)
        ram.write_quat(1, 2).unwrap();
        assert_eq!(ram.read_quat(0).unwrap(), Quat::Q3);
        assert_eq!(ram.read_quat(1).unwrap(), Quat::Q2);
        assert_eq!(ram.read_quat(2).unwrap(), Quat::Q2);
    }

    #[test]
    fn test_quat_bitwise_ops_and_display() {
        let q1 = Quat::Q3; 
        let q2 = Quat::Q1; 

        let iterations = 2_000_000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = q1 & q2;
            let _ = q1 | q2;
            let _ = q1 ^ q2;
            let _ = !q2;
        }
        let duration = start.elapsed();

        println!("\n[METRICS: Quat Bitwise Operations]");
        println!("  Batches executed: {}", iterations);
        println!("  Total ops count:  {} (4 ops per batch)", iterations * 4);
        println!("  Throughput:       {:.2} ops/sec", (iterations * 4) as f64 / duration.as_secs_f64());

        assert_eq!(q1 & q2, Quat::Q1);
        assert_eq!(q1 | q2, Quat::Q3);
        assert_eq!(q1 ^ q2, Quat::Q2);
        assert_eq!(!q2, Quat::Q2);
        assert_eq!(format!("{}", Quat::Q2), "10");
    }

    #[test]
    fn test_cpu_logic_ops() {
        let mut cpu = QuatCpu::new();
        cpu.load(0, 1, 1).unwrap(); 
        cpu.load(1, 0, 1).unwrap(); 

        let iterations = 500_000;
        let start = Instant::now();
        for _ in 0..iterations {
            cpu.and_regs(0, 1, 2).unwrap();
            cpu.not_reg(1, 3).unwrap();
        }
        let duration = start.elapsed();

        println!("\n[METRICS: CPU Logic Instructions]");
        println!("  Cycles evaluated: {}", iterations);
        println!("  Instruction rate: {:.2} inst/sec", (iterations * 2) as f64 / duration.as_secs_f64());

        cpu.and_regs(0, 1, 2).unwrap();
        assert_eq!(cpu.registers[2], Quat::Q1);
        cpu.not_reg(1, 3).unwrap();
        assert_eq!(cpu.registers[3], Quat::Q2);
    }

    #[test]
    fn test_static_ram() {
        let mut s_ram = StaticQuatRam::<4>::new(8).unwrap(); 
        
        let start = Instant::now();
        let iterations = 100_000;
        for _ in 0..iterations {
            s_ram.write_quat(5, Quat::Q3).unwrap();
            let _ = s_ram.read_quat(5).unwrap();
        }
        let duration = start.elapsed();

        println!("\n[METRICS: Static Stack-Allocated RAM]");
        println!("  Stack buffer size: {} bytes", mem::size_of_val(&s_ram));
        println!("  Access cycles:     {}", iterations * 2);
        println!("  Access throughput: {:.2} operations/sec", (iterations * 2) as f64 / duration.as_secs_f64());

        assert_eq!(s_ram.read_quat(5).unwrap(), Quat::Q3);
    }

    #[test]
    fn test_quat_add_sub_operators() {
        let q1 = Quat::Q2;
        let q2 = Quat::Q1;

        let iterations = 1_000_000;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = q1 + q2;
            let _ = Quat::Q3 + Quat::Q1;
            let _ = q1 - q2;
            let _ = Quat::Q0 - Quat::Q1;
        }
        let duration = start.elapsed();

        println!("\n[METRICS: Quat Operator Overloading (Mod-4 Arithmetic)]");
        println!("  Arithmetic rounds: {}", iterations);
        println!("  Throughput:        {:.2} calc/sec", (iterations * 4) as f64 / duration.as_secs_f64());

        assert_eq!(q1 + q2, Quat::Q3);
        assert_eq!(Quat::Q3 + Quat::Q1, Quat::Q0);
        assert_eq!(q1 - q2, Quat::Q1);
        assert_eq!(Quat::Q0 - Quat::Q1, Quat::Q3);
    }

    #[test]
    fn test_cpu_sub_and_borrow() {
        let mut cpu = QuatCpu::new();
        cpu.load(0, 0, 1).unwrap();
        cpu.load(1, 1, 1).unwrap();

        let iterations = 500_000;
        let start = Instant::now();
        let mut borrow_count = 0;
        for _ in 0..iterations {
            if cpu.sub(0, 1, 2).unwrap() {
                borrow_count += 1;
            }
        }
        let duration = start.elapsed();

        println!("\n[METRICS: CPU Subtraction & Borrow Flag]");
        println!("  Operations evaluated: {}", iterations);
        println!("  Borrow triggers:      {}", borrow_count);
        println!("  Throughput:           {:.2} ops/sec", iterations as f64 / duration.as_secs_f64());

        let borrow = cpu.sub(0, 1, 2).unwrap();
        assert_eq!(cpu.registers[2], Quat::Q2);
        assert!(borrow, "Borrow should be true on underflow");
    }

    #[test]
    fn test_stress_million_quats_ram() {
        let count: usize = 1_000_000;
        
        let start_alloc = Instant::now();
        let mut ram = QuatRam::new(count);
        let alloc_duration = start_alloc.elapsed();

        let start_write = Instant::now();
        for i in 0..count {
            let val = match i % 4 {
                0 => Quat::Q0,
                1 => Quat::Q1,
                2 => Quat::Q2,
                _ => Quat::Q3,
            };
            ram.write_quat(i, val).unwrap();
        }
        let write_duration = start_write.elapsed();

        let start_read = Instant::now();
        for i in (0..count).step_by(10) {
            let _ = ram.read_quat(i).unwrap();
        }
        let read_duration = start_read.elapsed();

        let compressed_bytes = count / 4;
        println!("\n[METRICS: 1M Quats RAM Stress & Throughput]");
        println!("  Total capacity:     {} quats", count);
        println!("  Memory allocated:   {} bytes ({:.2} MB)", compressed_bytes, compressed_bytes as f64 / 1_048_576.0);
        println!("  Allocation speed:   {:?}", alloc_duration);
        println!("  Sequential write:   {:.2} M quats/sec", count as f64 / write_duration.as_secs_f64());
        println!("  Sampled read speed: {:.2} reads/sec", (count / 10) as f64 / read_duration.as_secs_f64());

        assert_eq!(ram.capacity(), count);
        assert_eq!(ram.read_quat(0).unwrap(), Quat::Q0);
        assert_eq!(ram.read_quat(500_001).unwrap(), Quat::Q1);
        assert_eq!(ram.read_quat(999_999).unwrap(), Quat::Q3);
    }

    #[test]
    fn test_cpu_ram_execution_cycle() {
        let mut ram = QuatRam::new(4);
        let mut cpu = QuatCpu::new();

        ram.write_quat(0, Quat::Q1).unwrap();
        ram.write_quat(1, Quat::Q3).unwrap();

        cpu.registers[0] = Quat::Q1;
        cpu.registers[1] = Quat::Q2;

        let iterations = 100_000;
        let start = Instant::now();
        for _ in 0..iterations {
            cpu.pc = 0;
            let _ = cpu.step(&ram);
            let _ = cpu.step(&ram);
        }
        let duration = start.elapsed();

        println!("\n[METRICS: CPU Execution Cycle & IPS]");
        println!("  Full cycles run:  {}", iterations);
        println!("  Total steps:      {}", iterations * 2);
        println!("  Cycle frequency:  {:.2} cycles/sec", iterations as f64 / duration.as_secs_f64());
        println!("  Avg step latency: {:?}", duration / (iterations * 2));

        cpu.pc = 0;
        cpu.step(&ram).unwrap();
        assert_eq!(cpu.registers[0], Quat::Q3);
        assert_eq!(cpu.pc, 1);

        cpu.step(&ram).unwrap();
        assert_eq!(cpu.pc, 5);
    }

    #[test]
    fn test_out_of_bounds_errors() {
        let iterations = 500_000;
        let mut ram = QuatRam::new(4);
        let mut cpu = QuatCpu::new();

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = ram.read_quat(4);
            let _ = cpu.load(4, 0, 1);
        }
        let duration = start.elapsed();

        println!("\n[METRICS: Error Guard & Bounds Checking Overhead]");
        println!("  Checks performed: {}", iterations * 2);
        println!("  Guard overhead:   {:.2} checks/sec", (iterations * 2) as f64 / duration.as_secs_f64());

        assert_eq!(ram.read_quat(4), Err(CpuError::InvalidAddress));
        assert_eq!(ram.write_quat(10, Quat::Q1), Err(CpuError::InvalidAddress));
        assert_eq!(cpu.load(4, 0, 1), Err(CpuError::InvalidRegister));
        assert_eq!(cpu.add(0, 5, 1), Err(CpuError::InvalidRegister));
    }

    #[test]
    fn test_ffi_interface() {
        let iterations = 50_000;
        let start = Instant::now();
        
        unsafe {
            for _ in 0..iterations {
                let ram_ptr = quat_ram_new(8);
                let _ = quat_ram_write(ram_ptr, 2, CQuat::Q2);
                let mut out_quat = CQuat::Q0;
                let _ = quat_ram_read(ram_ptr, 2, &mut out_quat);
                let cpu_ptr = quat_cpu_new();
                let _ = quat_cpu_step(cpu_ptr, ram_ptr);
                quat_ram_free(ram_ptr);
                quat_cpu_free(cpu_ptr);
            }
        }
        let duration = start.elapsed();

        println!("\n[METRICS: FFI Interop Overhead]");
        println!("  FFI cycles:     {}", iterations);
        println!("  FFI throughput: {:.2} cycles/sec", iterations as f64 / duration.as_secs_f64());

        unsafe {
            let ram_ptr = quat_ram_new(8);
            assert!(!ram_ptr.is_null());
            let res_write = quat_ram_write(ram_ptr, 2, CQuat::Q2);
            assert_eq!(res_write, 0);

            let mut out_quat = CQuat::Q0;
            let res_read = quat_ram_read(ram_ptr, 2, &mut out_quat);
            assert_eq!(res_read, 0);
            assert_eq!(out_quat, CQuat::Q2);

            let cpu_ptr = quat_cpu_new();
            assert!(!cpu_ptr.is_null());
            let res_step = quat_cpu_step(cpu_ptr, ram_ptr);
            assert_eq!(res_step, 0);

            quat_ram_free(ram_ptr);
            quat_cpu_free(cpu_ptr);
        }
    }
 
    #[test]
fn test_assembler_parsing() {
    let source = "
        ; Test script comment
        NOP
        ADD
        SUB
        JUMP
    ";
    let quats = Assembler::assemble(source).unwrap();
    assert_eq!(quats.len(), 4);
    assert_eq!(quats[0], Quat::Q0);
    assert_eq!(quats[1], Quat::Q1);
    assert_eq!(quats[2], Quat::Q2);
    assert_eq!(quats[3], Quat::Q3);
}

#[test]
fn test_cpu_load_immediate() {
    let mut cpu = QuatCpu::new();
    cpu.load_immediate(0, Quat::Q3).unwrap();
    assert_eq!(cpu.registers[0], Quat::Q3);
}

#[test]
fn test_multicore_simulation() {
    let result = MultiCoreSystem::run_shared_ram_simulation();
    assert!(result.is_ok());
}

}