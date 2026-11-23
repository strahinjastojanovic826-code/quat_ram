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

    /// Serializes the packed quaternary RAM buffer into a raw byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Restores a `QuatRam` from a packed byte slice and target `capacity_in_quats`.
    pub fn from_bytes(capacity_in_quats: usize, bytes: &[u8]) -> Result<Self, CpuError> {
        let bytes_needed = (capacity_in_quats + 3) >> 2;
        if bytes.len() < bytes_needed {
            return Err(CpuError::InvalidAddress);
        }
        Ok(Self {
            data: bytes[..bytes_needed].to_vec(),
            capacity_in_quats,
        })
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
    
    /// Serializes the packed stack RAM buffer into a fixed byte array.
    pub fn to_bytes(&self) -> [u8; N] {
        self.data
    }

    /// Restores a `StaticQuatRam<N>` from a packed byte array and `capacity_in_quats`.
    pub fn from_bytes(capacity_in_quats: usize, bytes: [u8; N]) -> Result<Self, CpuError> {
        if capacity_in_quats > N * 4 {
            return Err(CpuError::InvalidAddress);
        }
        Ok(Self {
            data: bytes,
            capacity_in_quats,
        })
    }
}

pub trait Ram {
    fn capacity(&self) -> usize;
    fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError>;
    fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError>;

    fn clear(&mut self) -> Result<(), CpuError> {
        for addr in 0..self.capacity() {
            self.write_quat(addr, Quat::Q0)?;
        }
        Ok(())
    }

    fn write_bulk(&mut self, start_addr: usize, quats: &[Quat]) -> Result<(), CpuError> {
        if start_addr.saturating_add(quats.len()) > self.capacity() {
            return Err(CpuError::InvalidAddress);
        }
        for (offset, &q) in quats.iter().enumerate() {
            self.write_quat(start_addr + offset, q)?;
        }
        Ok(())
    }

    fn read_bulk(&self, start_addr: usize, len: usize) -> Result<Vec<Quat>, CpuError> {
        if start_addr.saturating_add(len) > self.capacity() {
            return Err(CpuError::InvalidAddress);
        }
        let mut out = Vec::with_capacity(len);
        for offset in 0..len {
            out.push(self.read_quat(start_addr + offset)?);
        }
        Ok(out)
    }
}

impl Ram for QuatRam {
    fn capacity(&self) -> usize {
        self.capacity_in_quats
    }
    fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError> {
        self.read_quat(quat_addr)
    }
    fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError> {
        self.write_quat(quat_addr, value)
    }
    fn clear(&mut self) -> Result<(), CpuError> {
        QuatRam::clear(self);
        Ok(())
    }
}

impl<const N: usize> Ram for StaticQuatRam<N> {
    fn capacity(&self) -> usize {
        self.capacity_in_quats
    }
    fn read_quat(&self, quat_addr: usize) -> Result<Quat, CpuError> {
        self.read_quat(quat_addr)
    }
    fn write_quat(&mut self, quat_addr: usize, value: impl TryInto<Quat>) -> Result<(), CpuError> {
        self.write_quat(quat_addr, value)
    }
}