use std::ops::{Range, RangeInclusive};

use log::debug;

pub trait BitOps {
    fn get_bit(self, bit: u8) -> u8;
    fn set_bit(&mut self, bit: u8);
    fn clear_bit(&mut self, bit: u8);
    fn get_bits_exclusive(self, bits: Range<u8>) -> u8;
    fn get_bits_inclusive(self, bits: RangeInclusive<u8>) -> u8;
    fn is_bit_addressable(self) -> bool;
    fn is_bit_set(self, bit: u8) -> bool;
    fn to_signed(self) -> i32;
}

impl BitOps for u8 {
    fn get_bit(self, bit: u8) -> u8 {
        debug!("BitOps::get_bit({:#04x}, {})", self, bit);
        let data = (self >> bit) & 0x1;
        data
    }

    fn set_bit(&mut self, bit: u8) {
        debug!("BitOps::set_bit({:#04x}, {})", self, bit);
        *self |= 1 << bit;
        debug!("Byte: {:#04x}", self);
    }

    fn clear_bit(&mut self, bit: u8) {
        debug!("BitOps::clear_bit({:#04x}, {})", self, bit);
        *self &= !(1 << bit);
        debug!("Byte: {:#04x}", self);
    }

    fn get_bits_exclusive(self, bits: Range<u8>) -> u8 {
        debug!("BitOps::get_bits_exclusive({:#04x}, {:?})", self, bits);
        let mut res = 0u8;

        for bit in bits {
            res |= self.get_bit(bit) << bit;
        }

        res
    }

    fn get_bits_inclusive(self, bits: RangeInclusive<u8>) -> u8 {
        debug!("BitOps::get_bits_inclusive({:#04x}, {:?})", self, bits);
        let mut res = 0u8;

        for bit in bits {
            res |= Self::get_bit(self, bit) << bit;
        }

        res
    }

    fn is_bit_addressable(self) -> bool {
        debug!("BitOps::is_bit_addressable({:#04x})", self);
        (self >= 32 && self <= 47) || (self >= 128 && self % 8 == 0)
    }

    fn is_bit_set(self, bit: u8) -> bool {
        debug!("BitOps::is_bit_set({:#04x}, {})", self, bit);
        self.get_bit(bit) == 1
    }

    fn to_signed(self) -> i32 {
        let data = self as i32;
        if data >= 0 && data <= 127 {
            data as i32
        } else {
            (data - 256) as i32
        }
    }
}

#[cfg(test)]
mod bitops_tests {
    use super::BitOps;

    #[test]
    fn get_bit() {
        let byte: u8 = 0x8a;

        assert_eq!(byte.get_bit(1), 1);
        assert_eq!(byte.get_bit(3), 1);
        assert_eq!(byte.get_bit(7), 1);
    }

    #[test]
    fn set_bit() {
        let mut byte = 0u8;

        byte.set_bit(1);
        assert_eq!(byte, 2);
    }
}
