use bitflags::bitflags;
use std::{
    fmt::{Debug, Display, LowerHex},
    ops::{Add, AddAssign, BitAnd},
};

bitflags! {
    pub struct ArithmeticOpFlags: u8 {
        const C = 1 << 0;
        const AC = 1 << 1;
        const OVERFLOW = 1 << 2;
        const BORROW = 1 << 3;
    }
}

/// A struct representing a (optionally signed) byte.
///
/// ### Signed bytes
///
/// For negative signed bytes:
/// The inner value will go from 128 (-1) to 255 (-128).
/// For positive signed bytes:
/// The inner value will go from 0 (0) to 127 (127).
///
/// ### Unsigned bytes
/// Unsigned bytes will have the full (255) range of a byte.
///
/// All project-relevant arithmetic operations implemented in this struct will
/// be mindful of the above limits.
#[derive(Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub struct Byte {
    inner_value: u8,
    signed: bool,
    flags: ArithmeticOpFlags,
}

impl Debug for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner_value)
        // f.debug_struct("Byte").field("inner_value", &self.inner_value).field("signed", &self.signed).field("flags", &self.flags).finish()
    }
}

impl Display for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner_value)
    }
}

impl LowerHex for Byte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.inner_value, f)
    }
}

impl From<u8> for Byte {
    fn from(val: u8) -> Self {
        Byte::new(val, false)
    }
}

impl From<i8> for Byte {
    fn from(val: i8) -> Self {
        let mut byte = val as u8;

        if byte > 127 {
            byte = byte - 127;
        }

        Byte::new(byte, true)
    }
}

impl PartialEq<Byte> for Byte {
    fn eq(&self, other: &Byte) -> bool {
        self.inner_value == other.inner_value && self.signed == other.signed
    }
}

impl BitAnd<u8> for Byte {
    type Output = Self;

    fn bitand(self, rhs: u8) -> Self::Output {
        Byte::from(self.inner_value & rhs)
    }
}

impl BitAnd<i8> for Byte {
    type Output = Self;

    fn bitand(self, rhs: i8) -> Self::Output {
        self & Byte::from(rhs)
    }
}

impl BitAnd<Byte> for Byte {
    type Output = Self;

    fn bitand(self, rhs: Byte) -> Self::Output {
        Byte::from(self.inner_value & rhs.inner_value)
    }
}

impl Add<u8> for Byte {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        self.add(Byte::from(rhs))
    }
}

impl Add<i8> for Byte {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        self.add(Byte::from(rhs))
    }
}

impl Add<Byte> for Byte {
    type Output = Self;

    fn add(self, rhs: Byte) -> Self::Output {
        let result: Byte;
 
        if (self.signed || rhs.signed) && (self.inner_value > 127 || rhs.inner_value > 127) {
            result = Byte::from(self.to_signed() + rhs.to_signed())
        } else {
            result = Byte::from(self.inner_value + rhs.inner_value)
        }

        result
    }
}

impl AddAssign<u8> for Byte {
    fn add_assign(&mut self, rhs: u8) {
        *self = self.add(rhs);
    }
}

impl AddAssign<i8> for Byte {
    fn add_assign(&mut self, rhs: i8) {
        *self = self.add(rhs);
    }
}

impl AddAssign<Byte> for Byte {
    fn add_assign(&mut self, rhs: Byte) {
        *self = self.add(rhs);
    }
}

impl Byte {
    pub fn new(val: u8, signed: bool) -> Self {
        Byte { inner_value: val, signed, flags: ArithmeticOpFlags::empty() }
    }

    pub const fn empty() -> Self {
        Byte { inner_value: 0, signed: false, flags: ArithmeticOpFlags::empty() }
    }

    pub const fn empty_signed() -> Self {
        Byte { inner_value: 0, signed: true, flags: ArithmeticOpFlags::empty() }
    }

    pub fn is_signed(&self) -> bool {
        self.signed
    }

    pub fn to_signed(&self) -> i8 {
        let signed_value: i8;

        if self.inner_value > 127 {
            signed_value = 0i8 - (self.inner_value - 127) as i8;
        } else {
            signed_value = self.inner_value as i8;
        }

        signed_value
    }

    pub fn get_inner_value(&self) -> u8 {
        self.inner_value
    }
}
