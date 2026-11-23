use crate::error::CpuError;
use crate::quat::Quat;
use crate::ram::Ram;

/// Decoded quaternary instruction representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuatInstruction {
    Nop,
    Add { src1: usize, src2: usize, dest: usize },
    Sub { src1: usize, src2: usize, dest: usize },
    JumpIfNotZero { target_addr: usize },
    Halt,
}

impl QuatInstruction {
    pub fn decode(opcode: Quat) -> Self {
        match opcode {
            Quat::Q0 => Self::Nop,
            Quat::Q1 => Self::Add { src1: 0, src2: 1, dest: 0 },
            Quat::Q2 => Self::Sub { src1: 0, src2: 1, dest: 0 },
            Quat::Q3 => Self::JumpIfNotZero { target_addr: 5 },
        }
    }
}

/// CPU emulation featuring 4 Quat registers and basic arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuatCpu {
    pub registers: [Quat; 4],
    pub pc: usize,
    pub zero_flag: bool,
    pub overflow_flag: bool,
    pub halted: bool,
}

#[derive(Debug, Clone)]
pub struct CpuTrace {
    pub pc: usize,
    pub registers: [Quat; 4],
    pub zero_flag: bool,
    pub overflow_flag: bool,
}

impl core::fmt::Display for CpuTrace {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "PC={} | R0={} R1={} R2={} R3={} | Z={} O={}",
            self.pc,
            self.registers[0].value(),
            self.registers[1].value(),
            self.registers[2].value(),
            self.registers[3].value(),
            u8::from(self.zero_flag),
            u8::from(self.overflow_flag),
        )
    }
}

/// Lightweight debugger holding breakpoints and recent CPU execution traces.
#[derive(Debug, Clone, Default)]
pub struct CpuDebugger {
    pub breakpoints: Vec<usize>,
    pub history: Vec<CpuTrace>,
    pub max_history: usize,
}

impl CpuDebugger {
    pub fn new(max_history: usize) -> Self {
        Self {
            breakpoints: Vec::new(),
            history: Vec::new(),
            max_history,
        }
    }

    pub fn add_breakpoint(&mut self, addr: usize) {
        if !self.breakpoints.contains(&addr) {
            self.breakpoints.push(addr);
        }
    }

    pub fn remove_breakpoint(&mut self, addr: usize) {
        self.breakpoints.retain(|&b| b != addr);
    }

    pub fn has_breakpoint(&self, pc: usize) -> bool {
        self.breakpoints.contains(&pc)
    }

    pub fn record(&mut self, trace: CpuTrace) {
        if self.max_history == 0 {
            return;
        }
        if self.history.len() == self.max_history {
            self.history.remove(0);
        }
        self.history.push(trace);
    }
}

impl QuatCpu {
    /// Initializes CPU with all 4 registers set to Quat::Q0.
    pub fn new() -> Self {
        Self {
            registers: [Quat::Q0; 4],
            pc: 0,
            zero_flag: false,
            overflow_flag: false,
            halted: false,
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

    /// Executes a single step if `self.pc` is not on a breakpoint, recording the trace.
    pub fn step_with_debugger<R: crate::ram::Ram>(
        &mut self,
        ram: &R,
        debugger: &mut CpuDebugger,
        ignore_breakpoint_at_pc: bool,
    ) -> Result<bool, CpuError> {
        if !ignore_breakpoint_at_pc && debugger.has_breakpoint(self.pc) {
            debugger.record(self.trace());
            return Ok(false);
        }
        self.step(ram)?;
        debugger.record(self.trace());
        Ok(true)
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

    /// Compares two registers and updates `zero_flag` (`true` if equal).
    pub fn cmp_regs(&mut self, reg1: usize, reg2: usize) -> Result<bool, CpuError> {
        if reg1 >= self.registers.len() || reg2 >= self.registers.len() {
            return Err(CpuError::InvalidRegister);
        }
        let is_equal = self.registers[reg1].eq_quat(self.registers[reg2]);
        self.zero_flag = is_equal;
        Ok(is_equal)
    }

    /// Rotates a register value left in quaternary space and updates flags.
    pub fn rotl_reg(&mut self, src: usize, dest: usize, shift: u32) -> Result<(), CpuError> {
        if src >= self.registers.len() || dest >= self.registers.len() {
            return Err(CpuError::InvalidRegister);
        }
        let rotated = self.registers[src].rotl_quat(shift);
        self.registers[dest] = rotated;
        self.update_flags(dest, false)
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

    pub fn load_immediate(&mut self, reg: usize, val: Quat) -> Result<(), CpuError> {
    if reg >= self.registers.len() {
        return Err(CpuError::InvalidRegister);
    }
    self.registers[reg] = val;
    Ok(())
}

    pub fn halt(&mut self) {
        self.halted = true;
    }

    pub fn decode_next<R: Ram>(&self, ram: &R) -> Result<QuatInstruction, CpuError> {
        if self.halted {
            return Err(CpuError::Halted);
        }
        let instr_quat = ram.read_quat(self.pc)?;
        Ok(QuatInstruction::decode(instr_quat))
    }

    fn update_flags(&mut self, reg_idx: usize, overflow: bool) -> Result<(), CpuError> {
        if reg_idx >= self.registers.len() {
            return Err(CpuError::InvalidRegister);
        }
        self.overflow_flag = overflow;
        self.zero_flag = self.registers[reg_idx] == Quat::Q0;
        Ok(())
    }

    pub fn execute(&mut self, instr: QuatInstruction) -> Result<(), CpuError> {
        if self.halted {
            return Err(CpuError::Halted);
        }
        match instr {
            QuatInstruction::Nop => {
                self.pc += 1;
            }
            QuatInstruction::Add { src1, src2, dest } => {
                let overflow = self.add(src1, src2, dest)?;
                self.overflow_flag = overflow;
                self.zero_flag = self.registers[dest] == Quat::Q0;
                self.pc += 1;
            }
            QuatInstruction::Sub { src1, src2, dest } => {
                let borrow = self.sub(src1, src2, dest)?;
                self.overflow_flag = borrow;
                self.zero_flag = self.registers[dest] == Quat::Q0;
                self.pc += 1;
            }
            QuatInstruction::JumpIfNotZero { target_addr } => {
                if !self.zero_flag {
                    self.pc = target_addr;
                } else {
                    self.pc += 1;
                }
            }
            QuatInstruction::Halt => {
                self.halted = true;
            }
        }
        Ok(())
    }

    pub fn step<R: Ram>(&mut self, ram: &R) -> Result<(), CpuError> {
        let instr = self.decode_next(ram)?;
        self.execute(instr)
    }

    /// Serializes CPU state into a compact byte buffer (little-endian).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(4 + core::mem::size_of::<usize>() + 3);
        for r in self.registers {
            buf.push(r.value());
        }
        buf.extend_from_slice(&self.pc.to_le_bytes());
        buf.push(u8::from(self.zero_flag));
        buf.push(u8::from(self.overflow_flag));
        buf.push(u8::from(self.halted));
        buf
    }

    /// Restores CPU state from a byte slice produced by `to_bytes`.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CpuError> {
        let pc_len = core::mem::size_of::<usize>();
        let expected_len = 4 + pc_len + 3;
        if bytes.len() != expected_len {
            return Err(CpuError::InvalidValue);
        }

        let mut registers = [Quat::Q0; 4];
        for (i, slot) in registers.iter_mut().enumerate() {
            *slot = Quat::try_from(bytes[i])?;
        }

        let mut pc_bytes = [0u8; core::mem::size_of::<usize>()];
        pc_bytes.copy_from_slice(&bytes[4..4 + pc_len]);
        let pc = usize::from_le_bytes(pc_bytes);

        let zero_flag = match bytes[4 + pc_len] {
            0 => false,
            1 => true,
            _ => return Err(CpuError::InvalidValue),
        };
        let overflow_flag = match bytes[4 + pc_len + 1] {
            0 => false,
            1 => true,
            _ => return Err(CpuError::InvalidValue),
        };
        let halted = match bytes[4 + pc_len + 2] {
            0 => false,
            1 => true,
            _ => return Err(CpuError::InvalidValue),
        };

        Ok(Self {
            registers,
            pc,
            zero_flag,
            overflow_flag,
            halted,
        })
    }
}

impl Default for QuatCpu {
    fn default() -> Self {
        Self::new()
    }
}