use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CpuError {
    /// Requested register index does not exist (valid range: 0..=3)
    InvalidRegister,
    /// Requested RAM address is out of bounds
    InvalidAddress,
    /// Value exceeds 2-bit quat range (must be 0..=3 or bit values 0/1)
    InvalidValue,
}

impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRegister => write!(f, "Requested register index does not exist (valid range: 0..=3)"),
            Self::InvalidAddress => write!(f, "Requested RAM address is out of bounds"),
            Self::InvalidValue => write!(f, "Value exceeds 2-bit quat range (must be 0..=3 or bit values 0/1)"),
        }
    }
}

impl std::error::Error for CpuError {}