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
    fn strip_comment(line: &str) -> &str {
        match line.find(';') {
            Some(idx) => &line[..idx],
            None => line,
        }
    }

    /// Assembles lines with optional labels (`LABEL:`) and inline `;` comments.
    pub fn assemble_with_labels(&self, source: &str) -> Result<Vec<Quat>, CpuError> {
        let mut quats: Vec<Quat> = Vec::new();
        for raw_line in source.lines() {
            let code_part = Self::strip_comment(raw_line).trim();
            if code_part.is_empty() {
                continue;
            }
            let without_label = match code_part.split_once(':') {
                Some((_label, rest)) => rest.trim(),
                None => code_part,
            };
            if without_label.is_empty() {
                continue;
            }
            let mnemonic = without_label
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_uppercase();
            match mnemonic.as_str() {
                "NOP" => quats.push(Quat::Q0),
                "ADD" => quats.push(Quat::Q1),
                "SUB" => quats.push(Quat::Q2),
                "JUMP" | "JNZ" => quats.push(Quat::Q3),
                _ => return Err(CpuError::InvalidValue),
            }
        }
        Ok(quats)
    }
}