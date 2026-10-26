use crate::error::CpuError;
use crate::quat::Quat;

/// CPU emulation featuring 4 Quat registers and basic arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuatCpu {
    pub registers: [Quat; 4],
    pub pc: usize,
    pub zero_flag: bool,
    pub overflow_flag: bool,
}

#[derive(Debug, Clone)]
pub struct CpuTrace {
    pub pc: usize,
    pub registers: [Quat; 4],
    pub zero_flag: bool,
    pub overflow_flag: bool,
}

impl QuatCpu {
    /// Initializes CPU with all 4 registers set to Quat::Q0.
    pub fn new() -> Self {
        Self {
            registers: [Quat::Q0; 4],
            pc: 0,
            zero_flag: false,
            overflow_flag: false,
        }
    }

    /// Helper method to convert two bits into a raw `u8` quat value (0..=3).
    pub fn bits_to_quat(&self, b1: u8, b0: u8) -> Result<Quat, CpuError> {
        Quat::from_bits(b1, b0)
    }

    pub fn reset(&mut self) {
        self.registers = [Quat::Q0; 4];
        self.pc = 0;
        self.zero_flag = false;
        self.overflow_flag = false;
    }

    /// Helper method to split a `u8` quat value into high and low bits.
    pub fn quat_to_bits(&self, quat: Quat) -> (u8, u8) {
        quat.to_bits()
    }

    /// Loads two bits into the specified register.
    pub fn load(&mut self, reg: usize, b1: u8, b0: u8) -> Result<(), CpuError> {
        if reg >= self.registers.len() {
            return Err(CpuError::InvalidRegister);
        }
        let quat = Quat::from_bits(b1, b0)?;
        self.registers[reg] = quat;
        Ok(())
    }

    /// Adds values from `src1` and `src2` modulo 4, and stores the result in `dest`.
    /// Returns `Ok(carry)` where `carry` is true if overflow occurred (sum >= 4).
    pub fn add(&mut self, src1: usize, src2: usize, dest: usize) -> Result<bool, CpuError> {
        if src1 >= 4 || src2 >= 4 || dest >= 4 {
            return Err(CpuError::InvalidRegister);
        }

        let sum = self.registers[src1].value() + self.registers[src2].value();
        let carry = sum >= 4;
        self.registers[dest] = Quat::from_u8_unchecked(sum % 4);
        Ok(carry)
    }

    pub fn sub(&mut self, src1: usize, src2: usize, dest: usize) -> Result<bool, CpuError> {
        if src1 >= 4 || src2 >= 4 || dest >= 4 {
            return Err(CpuError::InvalidRegister);
        }

        let val1 = self.registers[src1].value() as i8;
        let val2 = self.registers[src2].value() as i8;
        let diff = val1 - val2;

        let (res_val, borrow) = if diff < 0 {
            ((diff + 4) as u8, true)
        } else {
            (diff as u8, false)
        };

        self.registers[dest] = Quat::from_u8_unchecked(res_val);
        Ok(borrow)
    }

    pub fn trace(&self) -> CpuTrace {
        CpuTrace {
            pc: self.pc,
            registers: self.registers,
            zero_flag: self.zero_flag,
            overflow_flag: self.overflow_flag,
        }
    }

    /// Performs bitwise AND between `src1` and `src2`, saving to `dest`.
    pub fn and_regs(&mut self, src1: usize, src2: usize, dest: usize) -> Result<(), CpuError> {
        if src1 >= 4 || src2 >= 4 || dest >= 4 {
            return Err(CpuError::InvalidRegister);
        }
        self.registers[dest] = self.registers[src1] & self.registers[src2];
        Ok(())
    }

    /// Performs bitwise OR between `src1` and `src2`, saving to `dest`.
    pub fn or_regs(&mut self, src1: usize, src2: usize, dest: usize) -> Result<(), CpuError> {
        if src1 >= 4 || src2 >= 4 || dest >= 4 {
            return Err(CpuError::InvalidRegister);
        }
        self.registers[dest] = self.registers[src1] | self.registers[src2];
        Ok(())
    }

    /// Performs bitwise XOR between `src1` and `src2`, saving to `dest`.
    pub fn xor_regs(&mut self, src1: usize, src2: usize, dest: usize) -> Result<(), CpuError> {
        if src1 >= 4 || src2 >= 4 || dest >= 4 {
            return Err(CpuError::InvalidRegister);
        }
        self.registers[dest] = self.registers[src1] ^ self.registers[src2];
        Ok(())
    }

    /// Inverts (NOT) the value in `src` and saves it to `dest`.
    pub fn not_reg(&mut self, src: usize, dest: usize) -> Result<(), CpuError> {
        if src >= 4 || dest >= 4 {
            return Err(CpuError::InvalidRegister);
        }
        self.registers[dest] = !self.registers[src];
        Ok(())
    }

    pub fn jump(&mut self, target_addr: usize) {
        self.pc = target_addr;
    }

    /// Jumps to target_addr if Zero Flag is set.
    pub fn jump_if_zero(&mut self, target_addr: usize) -> bool {
        if self.zero_flag {
            self.pc = target_addr;
            true
        } else {
            false
        }
    }

    pub fn step(&mut self, ram: &crate::ram::QuatRam) -> Result<(), CpuError> {
        let instr_quat = ram.read_quat(self.pc)?;

        match instr_quat {
            Quat::Q0 => {
                // NOP: skip
            }
            Quat::Q1 => {
                let overflow = self.add(0, 1, 0)?;
                self.overflow_flag = overflow;
                self.zero_flag = self.registers[0] == Quat::Q0;
            }
            Quat::Q2 => {
                let borrow = self.sub(0, 1, 0)?;
                self.overflow_flag = borrow;
                self.zero_flag = self.registers[0] == Quat::Q0;
            }
            Quat::Q3 => {
    // Primer: Skok ako nula zastavica (zero_flag) NIJE postavljena
    let target_addr = 5; // Primer adrese ili učitano iz memorije/registra
    if !self.zero_flag {
        self.pc = target_addr;
    }
    return Ok(());
}
        }

        self.pc += 1;
        Ok(())
    }
}

impl Default for QuatCpu {
    fn default() -> Self {
        Self::new()
    }
}