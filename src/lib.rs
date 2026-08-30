//! # Quatt CPU & RAM Library
//! Biblioteka za emulaciju CPU-a i RAM-a koji 2 binarna bita konvertuje u 1 kvat (0..3).

/// Moguće greške pri radu sa Quatt sistemom
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuError {
    /// Traženi registar ne postoji (validni su 0..3)
    InvalidRegister,
    /// Tražena adresa u RAM-u je van opsega
    InvalidAddress,
    /// Vrednost bita mora biti 0 ili 1, odnosno kvat 0 do 3
    InvalidValue,
}

/// Emulacija CPU-a sa 4 kvat registra
#[derive(Debug, Clone, Copy)]
pub struct QuatCpu {
    pub registers: [u8; 4],
}

impl QuatCpu {
    pub fn new() -> Self {
        Self { registers: [0; 4] }
    }

    pub fn bits_to_quat(b1: u8, b0: u8) -> Result<u8, CpuError> {
        if b1 > 1 || b0 > 1 {
            return Err(CpuError::InvalidValue);
        }
        Ok((b1 << 1) | b0)
    }

    pub fn quat_to_bits(quat: u8) -> (u8, u8) {
        let q = quat & 0b11;
        ((q >> 1) & 1, q & 1)
    }

    pub fn load(&mut self, reg: usize, b1: u8, b0: u8) -> Result<(), CpuError> {
        if reg >= self.registers.len() {
            return Err(CpuError::InvalidRegister);
        }
        let quat = Self::bits_to_quat(b1, b0)?;
        self.registers[reg] = quat;
        Ok(())
    }

    pub fn add(&mut self, src1: usize, src2: usize, dest: usize) -> Result<(), CpuError> {
        if src1 >= 4 || src2 >= 4 || dest >= 4 {
            return Err(CpuError::InvalidRegister);
        }
        self.registers[dest] = (self.registers[src1] + self.registers[src2]) % 4;
        Ok(())
    }
}

impl Default for QuatCpu {
    fn default() -> Self {
        Self::new()
    }
}

/// Emulacija RAM memorije bazirane na kvatovima (2 bita = 1 kvat)
pub struct QuatRam {
    data: Vec<u8>,
    capacity_in_quats: usize,
}

impl QuatRam {
    /// Alocira RAM memoriju za zadati broj kvatova
    pub fn new(capacity_in_quats: usize) -> Self {
        let bytes_needed = (capacity_in_quats + 3) / 4;
        Self {
            data: vec![0; bytes_needed],
            capacity_in_quats,
        }
    }

    /// Upisuje 1 kvat (0..3) na zadatu kvat-adresu
    pub fn write_quat(&mut self, quat_addr: usize, value: u8) -> Result<(), CpuError> {
        if value > 3 {
            return Err(CpuError::InvalidValue);
        }
        if quat_addr >= self.capacity_in_quats {
            return Err(CpuError::InvalidAddress);
        }

        let byte_index = quat_addr / 4;
        let quat_offset = quat_addr % 4;
        let bit_shift = quat_offset * 2;

        self.data[byte_index] &= !(0b11 << bit_shift);
        self.data[byte_index] |= (value & 0b11) << bit_shift;

        Ok(())
    }

    /// Čita 1 kvat (0..3) sa zadate kvat-adrese
    pub fn read_quat(&self, quat_addr: usize) -> Result<u8, CpuError> {
        if quat_addr >= self.capacity_in_quats {
            return Err(CpuError::InvalidAddress);
        }

        let byte_index = quat_addr / 4;
        let quat_offset = quat_addr % 4;
        let bit_shift = quat_offset * 2;

        Ok((self.data[byte_index] >> bit_shift) & 0b11)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ispravan_rad_cpu() {
        let mut cpu = QuatCpu::new();
        assert!(cpu.load(0, 1, 1).is_ok());
        assert_eq!(cpu.registers[0], 3);
    }

    #[test]
    fn test_sprecavanje_gresaka_cpu() {
        let mut cpu = QuatCpu::new();
        assert_eq!(cpu.load(0, 2, 0), Err(CpuError::InvalidValue));
        assert_eq!(cpu.load(9, 1, 0), Err(CpuError::InvalidRegister));
    }

    #[test]
    fn test_quat_ram() {
        let mut ram = QuatRam::new(8);

        assert!(ram.write_quat(0, 3).is_ok());
        assert!(ram.write_quat(1, 1).is_ok());
        assert!(ram.write_quat(5, 2).is_ok());

        assert_eq!(ram.read_quat(0).unwrap(), 3);
        assert_eq!(ram.read_quat(1).unwrap(), 1);
        assert_eq!(ram.read_quat(5).unwrap(), 2);
        assert_eq!(ram.read_quat(2).unwrap(), 0);

        // Provera greške van opsega RAM-a
        assert_eq!(ram.read_quat(10), Err(CpuError::InvalidAddress));
    }
}