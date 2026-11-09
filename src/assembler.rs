use crate::quat::Quat;
use crate::error::CpuError;

pub struct Assembler;

impl Assembler {
    /// Parses a simple text assembly script into a sequence of Quats for RAM loading.
    pub fn assemble(source: &str) -> Result<Vec<Quat>, CpuError> {
        let mut quats = Vec::new();
        
        for line in source.lines() {
            let line = line.trim();
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            let upper_line = line.to_uppercase();
            match upper_line.as_str() {
                "NOP" => quats.push(Quat::Q0),
                "ADD" => quats.push(Quat::Q1),
                "SUB" => quats.push(Quat::Q2),
                "JUMP" => quats.push(Quat::Q3),
                _ => return Err(CpuError::InvalidValue),
            }
        }

        Ok(quats)
    }
}