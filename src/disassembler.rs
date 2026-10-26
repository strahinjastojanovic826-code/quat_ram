use crate::ram::QuatRam;
use crate::quat::Quat;

pub fn disassemble(ram: &QuatRam, limit: usize) {
    for i in (0..limit).step_by(2) {
        if let Ok(instr) = ram.read_quat(i) {
            match instr {
                Quat::Q0 => println!("{}: NOP", i),
                Quat::Q1 => println!("{}: INSTRUCTION Q1 (ADD/REG)", i),
                Quat::Q2 => println!("{}: INSTRUCTION Q2 (SUB/REG)", i),
                Quat::Q3 => println!("{}: INSTRUCTION Q3 (JUMP)", i),
            }
        }
    }
}