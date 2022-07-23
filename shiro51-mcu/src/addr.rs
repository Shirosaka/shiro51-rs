use std::fmt::{Binary, LowerHex, UpperHex};
use std::ops::*;

// ensure that BitField is available when using any address type
pub use bit_field::BitField;
use shiro51_util::error::{ErrorType, RuntimeError};

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct BitAddr(u8);

impl BitAddr {
    #[inline]
    pub fn try_new(addr: u8) -> Result<BitAddr, RuntimeError> {
        let bit_addr = BitAddr(if addr <= 127 { addr / 8 + 32 } else { addr - (1 << (addr % 8)) });

        if !bit_addr.is_bit_addressable() {
            return Err(RuntimeError::new(ErrorType::InvalidBitAddr));
        }

        Ok(bit_addr)
    }

    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn as_addr8(&self) -> Addr8 {
        Addr8(self.0)
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn bit(&self) -> u8 {
        1 << (self.0 % 8)
    }

    #[inline]
    pub fn is_bit_addressable(&self) -> bool {
        (self.0 >= 0x20 && self.0 < 0x30) || (self.0 >= 0x80 && self.0 % 8 == 0)
    }
}

impl std::fmt::Display for BitAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Addr: {}.{}", self.0, self.bit())
    }
}

impl BitField for BitAddr {
    const BIT_LENGTH: usize = u8::BIT_LENGTH;

    #[inline]
    fn get_bit(&self, bit: usize) -> bool {
        self.0.get_bit(bit)
    }

    #[inline]
    fn get_bits<T: RangeBounds<usize>>(&self, range: T) -> Self {
        BitAddr(self.0.get_bits(range))
    }

    #[inline]
    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
        self.0.set_bit(bit, value);
        self
    }

    #[inline]
    fn set_bits<T: RangeBounds<usize>>(&mut self, range: T, value: Self) -> &mut Self {
        self.0.set_bits(range, value.as_u8());
        self
    }
}

impl LowerHex for BitAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Addr8(u8);

impl Addr8 {
    #[inline]
    pub fn new(addr: u8) -> Self {
        Self(addr)
    }

    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn as_u8(&self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub fn is_bit_addressable(&self) -> bool {
        (self.0 >= 0x20 && self.0 < 0x30) || (self.0 >= 0x80 && self.0 % 8 == 0)
    }
}

impl LowerHex for Addr8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for Addr8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl Binary for Addr8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl From<Addr8> for u8 {
    #[inline]
    fn from(addr: Addr8) -> Self {
        addr.as_u8()
    }
}

impl From<u8> for Addr8 {
    #[inline]
    fn from(addr: u8) -> Self {
        Addr8(addr)
    }
}

impl BitField for Addr8 {
    const BIT_LENGTH: usize = u8::BIT_LENGTH;

    #[inline]
    fn get_bit(&self, bit: usize) -> bool {
        self.0.get_bit(bit)
    }

    #[inline]
    fn get_bits<T: std::ops::RangeBounds<usize>>(&self, range: T) -> Self {
        Addr8(self.0.get_bits(range))
    }

    #[inline]
    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
        self.0.set_bit(bit, value);
        self
    }

    #[inline]
    fn set_bits<T: std::ops::RangeBounds<usize>>(&mut self, range: T, value: Self) -> &mut Self {
        self.0.set_bits(range, value.as_u8());
        self
    }
}

impl Add for Addr8 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Addr8(self.0 + rhs.0)
    }
}

impl Add<u8> for Addr8 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: u8) -> Self::Output {
        Addr8(self.0 + rhs)
    }
}

impl AddAssign for Addr8 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl AddAssign<u8> for Addr8 {
    #[inline]
    fn add_assign(&mut self, rhs: u8) {
        self.0 += rhs
    }
}

impl Sub for Addr8 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Addr8(self.0 - rhs.0)
    }
}

impl Sub<u8> for Addr8 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: u8) -> Self::Output {
        Addr8(self.0 - rhs)
    }
}

impl SubAssign for Addr8 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl SubAssign<u8> for Addr8 {
    #[inline]
    fn sub_assign(&mut self, rhs: u8) {
        self.0 -= rhs
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Addr16(u16);

impl Addr16 {
    #[inline]
    pub fn new(addr: u16) -> Self {
        Self(addr)
    }

    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn as_u8(&self) -> u8 {
        self.0 as u8
    }

    #[inline]
    pub const fn as_u16(&self) -> u16 {
        self.0
    }

    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline]
    pub const fn is_null(&self) -> bool {
        self.0 == 0
    }
}

impl From<Addr16> for u16 {
    fn from(addr: Addr16) -> Self {
        addr.as_u16()
    }
}

impl LowerHex for Addr16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl Binary for Addr16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl Add for Addr16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Addr16(self.0 + rhs.0)
    }
}

impl Add<u16> for Addr16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        Addr16(self.0 + rhs)
    }
}

impl AddAssign for Addr16 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl AddAssign<u16> for Addr16 {
    fn add_assign(&mut self, rhs: u16) {
        self.0 += rhs
    }
}

impl AddAssign<usize> for Addr16 {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs as u16
    }
}

impl AddAssign<i32> for Addr16 {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs as u16
    }
}

impl BitAnd<u16> for Addr16 {
    type Output = Self;

    fn bitand(self, rhs: u16) -> Self::Output {
        Addr16(self.0 & rhs)
    }
}

impl BitAndAssign<u16> for Addr16 {
    fn bitand_assign(&mut self, rhs: u16) {
        self.0 &= rhs
    }
}

impl BitOr<u16> for Addr16 {
    type Output = Self;

    fn bitor(self, rhs: u16) -> Self::Output {
        Addr16(self.0 | rhs)
    }
}

impl BitOrAssign<u16> for Addr16 {
    fn bitor_assign(&mut self, rhs: u16) {
        self.0 |= rhs
    }
}

impl Shl<u16> for Addr16 {
    type Output = Self;

    fn shl(self, rhs: u16) -> Self::Output {
        Addr16(self.0 << rhs)
    }
}

impl Shr<u16> for Addr16 {
    type Output = Self;

    fn shr(self, rhs: u16) -> Self::Output {
        Addr16(self.0 >> rhs)
    }
}

impl BitField for Addr16 {
    const BIT_LENGTH: usize = u16::BIT_LENGTH;

    fn get_bit(&self, bit: usize) -> bool {
        self.0.get_bit(bit)
    }

    fn get_bits<T: std::ops::RangeBounds<usize>>(&self, range: T) -> Self {
        Addr16(self.0.get_bits(range))
    }

    fn set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
        self.0.set_bit(bit, value);
        self
    }

    fn set_bits<T: std::ops::RangeBounds<usize>>(&mut self, range: T, value: Self) -> &mut Self {
        self.0.set_bits(range, value.as_u16());
        self
    }
}

pub const fn addr16(addr1: Addr8, addr2: Addr8) -> Addr16 {
    Addr16(((addr1.as_u8() as u16) << 8) | addr2.as_u8() as u16)
}
