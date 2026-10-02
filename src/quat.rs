use core::fmt;
use core::ops::{Add, BitAnd, BitOr, BitXor, Not, Sub};
use crate::error::CpuError;

/// Strongly-typed 2-bit Quat representation (0..=3).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Quat {
    Q0 = 0b00,
    Q1 = 0b01,
    Q2 = 0b10,
    Q3 = 0b11,
}

impl Quat {
    /// Creates a Quat from two binary bits (b1: high bit, b0: low bit).
    pub const fn from_bits(b1: u8, b0: u8) -> Result<Self, CpuError> {
        if b1 > 1 || b0 > 1 {
            return Err(CpuError::InvalidValue);
        }
        Ok(Self::from_u8_unchecked((b1 << 1) | b0))
    }

    /// Converts the Quat into two binary bits (b1: high bit, b0: low bit).
    pub const fn to_bits(self) -> (u8, u8) {
        let val = self as u8;
        ((val >> 1) & 1, val & 1)
    }

    /// Returns the underlying `u8` value (0..=3).
    pub const fn value(self) -> u8 {
        self as u8
    }

    /// Internal helper for fast and safe conversion using lower 2 bits.
    pub const fn from_u8_truncated(val: u8) -> Self {
        match val & 0b11 {
            0 => Quat::Q0,
            1 => Quat::Q1,
            2 => Quat::Q2,
            _ => Quat::Q3,
        }
    }

    /// Fast unchecked conversion for internal guaranteed bounds.
    pub const fn from_u8_unchecked(val: u8) -> Self {
        match val {
            0 => Quat::Q0,
            1 => Quat::Q1,
            2 => Quat::Q2,
            _ => Quat::Q3,
        }
    }
}

impl TryFrom<u8> for Quat {
    type Error = CpuError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Quat::Q0),
            1 => Ok(Quat::Q1),
            2 => Ok(Quat::Q2),
            3 => Ok(Quat::Q3),
            _ => Err(CpuError::InvalidValue),
        }
    }
}

impl From<Quat> for u8 {
    fn from(quat: Quat) -> Self {
        quat as u8
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Quat::Q0 => write!(f, "00"),
            Quat::Q1 => write!(f, "01"),
            Quat::Q2 => write!(f, "10"),
            Quat::Q3 => write!(f, "11"),
        }
    }
}

impl BitAnd for Quat {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Quat::from_u8_unchecked(self.value() & rhs.value())
    }
}

impl BitOr for Quat {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Quat::from_u8_unchecked(self.value() | rhs.value())
    }
}

impl BitXor for Quat {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Quat::from_u8_unchecked(self.value() ^ rhs.value())
    }
}

impl Not for Quat {
    type Output = Self;

    fn not(self) -> Self::Output {
        Quat::from_u8_unchecked(!self.value() & 0b11)
    }
}

impl Add for Quat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Quat::from_u8_unchecked((self.value() + rhs.value()) % 4)
    }
}

impl Sub for Quat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = (self.value() as i8) - (rhs.value() as i8);
        let res = if diff < 0 { diff + 4 } else { diff };
        Quat::from_u8_unchecked(res as u8)
    }
}