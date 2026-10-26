use crate::error::CpuError;
use crate::quat::Quat;

/// Emulated RAM memory packed with 2-bit quats (4 quats per byte).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuatRam {
    data: Vec<u8>,
    capacity_in_quats: usize,
}

impl QuatRam {
    /// Allocates RAM memory to accommodate the specified number of quats.
    pub fn new(capacity_in_quats: usize) -> Self {
        let bytes_needed = (capacity_in_quats + 3) >> 2;
        Self {
            data: vec![0; bytes_needed],
            capacity_in_quats,
        }
    }

    /// Returns the total capacity of RAM in quats.
    pub fn capacity(&self) -> usize {
        self.capacity_in_quats
    }

    pub fn clear(&mut self) {
        for byte in self.data.iter_mut() {
            *byte = 0;
        }
    }

    pub fn memset(&mut self, start_addr: usize, len: usize, value: Quat) -> Result<(), CpuError> {
        if start_addr + len > self.capacity_in_quats {
            return Err(CpuError::InvalidAddress);
        }
        for i in 0..len {
            self.write_quat(start_addr + i, value)?;
        }
        Ok(())
    }

    /// Writes a quat value (accepts both `u8` or `Quat`) to the specified quat address.
    pub fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError> {
        if quat_addr >= self.capacity_in_quats {
            return Err(CpuError::InvalidAddress);
        }

        let quat: Quat = value.try_into().map_err(|_| CpuError::InvalidValue)?;
        let byte_index = quat_addr >> 2;
        let bit_shift = (quat_addr & 3) << 1;

        let mask = !(0b11u8 << bit_shift);
        self.data[byte_index] = (self.data[byte_index] & mask) | (quat.value() << bit_shift);
        Ok(())
    }

    /// Reads a quat value from the specified quat address.
    pub fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError> {
        if quat_addr >= self.capacity_in_quats {
            return Err(CpuError::InvalidAddress);
        }

        let byte_index = quat_addr >> 2;
        let bit_shift = (quat_addr & 3) << 1;

        let quat_val = (self.data[byte_index] >> bit_shift) & 0b11;
        Ok(Quat::from_u8_unchecked(quat_val))
    }
}

/// Stack-allocated RAM memory packed with 2-bit quats, useful for fixed-size environments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticQuatRam<const N: usize> {
    data: [u8; N],
    capacity_in_quats: usize,
}

impl<const N: usize> StaticQuatRam<N> {
    /// Creates a new static RAM buffer. `capacity_in_quats` cannot exceed `N * 4`.
    pub fn new(capacity_in_quats: usize) -> Result<Self, CpuError> {
        if capacity_in_quats > N * 4 {
            return Err(CpuError::InvalidAddress);
        }
        Ok(Self {
            data: [0; N],
            capacity_in_quats,
        })
    }

    pub fn read_raw_byte(&self, byte_index: usize) -> Result<u8, CpuError> {
        self.data.get(byte_index).copied().ok_or(CpuError::InvalidAddress)
    }

    pub fn write_raw_byte(&mut self, byte_index: usize, byte: u8) -> Result<(), CpuError> {
        if let Some(slot) = self.data.get_mut(byte_index) {
            *slot = byte;
            Ok(())
        } else {
            Err(CpuError::InvalidAddress)
        }
    }

    pub fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError> {
        if quat_addr >= self.capacity_in_quats {
            return Err(CpuError::InvalidAddress);
        }

        let quat: Quat = value.try_into().map_err(|_| CpuError::InvalidValue)?;
        let byte_index = quat_addr >> 2;
        let bit_shift = (quat_addr & 3) << 1;

        let mask = !(0b11u8 << bit_shift);
        self.data[byte_index] = (self.data[byte_index] & mask) | (quat.value() << bit_shift);
        Ok(())
    }

    pub fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError> {
        if quat_addr >= self.capacity_in_quats {
            return Err(CpuError::InvalidAddress);
        }

        let byte_index = quat_addr >> 2;
        let bit_shift = (quat_addr & 3) << 1;

        let quat_val = (self.data[byte_index] >> bit_shift) & 0b11;
        Ok(Quat::from_u8_unchecked(quat_val))
    }
}

pub trait Ram {
    fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError>;
    fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError>;
}

impl Ram for QuatRam {
    fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError> {
        self.read_quat(quat_addr)
    }

    fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError> {
        self.write_quat(quat_addr, value)
    }
}

impl<const N: usize> Ram for StaticQuatRam<N> {
    fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError> {
        self.read_quat(quat_addr)
    }

    fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError> {
        self.write_quat(quat_addr, value)
    }
}