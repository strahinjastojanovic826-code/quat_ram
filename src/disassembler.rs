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

/// Disassembles up to `limit` quats from `ram` into a formatted string.
pub fn disassemble_to_string<R: crate::ram::Ram>(ram: &R, limit: usize) -> String {
    let mut output = String::new();
    let max_addr = limit.min(ram.capacity());
    for i in 0..max_addr {
        if let Ok(instr) = ram.read_quat(i) {
            let line = match instr {
                Quat::Q0 => format!("{i}: NOP\n"),
                Quat::Q1 => format!("{i}: ADD (R0 + R1 -> R0)\n"),
                Quat::Q2 => format!("{i}: SUB (R0 - R1 -> R0)\n"),
                Quat::Q3 => format!("{i}: JNZ (CONDITIONAL JUMP)\n"),
            };
            output.push_str(&line);
        }
    }
    output
}