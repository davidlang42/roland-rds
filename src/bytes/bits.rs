use std::{fmt::Display, str::FromStr};
use serde::{Serialize, Deserialize};

use crate::json::serialize_fromstr_display;

#[derive(Debug, Copy, Clone)]
pub struct Bit(bool);

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Bit {
    pub const ONE: Bit = Bit(true);
    pub const ZERO: Bit = Bit(false);

    pub fn on(&self) -> bool {
        self.0
    }

    pub fn to_char(&self) -> char {
        if self.0 {
            '1'
        } else {
            '0'
        }
    }
}

#[derive(Debug)]
pub struct Bits<const N: usize>(pub [Bit; N]);

impl<'de, const N: usize> Deserialize<'de> for Bits<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        serialize_fromstr_display::deserialize(deserializer)
    }
}

impl<const N: usize> Serialize for Bits<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serialize_fromstr_display::serialize(self, serializer)
    }
}

impl<const N: usize> Display for Bits<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, bit) in self.0.iter().enumerate() {
            if i % Self::BITS_PER_BYTE == 0 && i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

pub enum BitsError {
    IncompleteByteBeforeEnd(String),
    InvalidDigit(char),
    WrongNumberOfBits {
        expected: usize,
        found: usize
    }
}

impl Display for BitsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompleteByteBeforeEnd(s) => write!(f, "incomplete byte before end ({})", s),
            Self::InvalidDigit(c) => write!(f, "invalid bit digit ({})", c),
            Self::WrongNumberOfBits { expected, found } => write!(f, "wrong number of bits (expected {}, found {})", expected, found)
        }
    }
}

impl<const N: usize> FromStr for Bits<N> {
    type Err = BitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<&str> = s.split(" ").collect();
        let mut bits = Vec::new();
        for i in 0..bytes.len() {
            if i != bytes.len() - 1 && bytes[i].len() != Self::BITS_PER_BYTE {
                return Err(BitsError::IncompleteByteBeforeEnd(bytes[i].to_string()));
            }
            for c in bytes[i].chars() {
                let bit = match c {
                    '0' => Bit::ZERO,
                    '1' => Bit::ONE,
                    _ => return Err(BitsError::InvalidDigit(c))
                };
                bits.push(bit);
            }
        }
        if bits.len() != N {
            Err(BitsError::WrongNumberOfBits { expected: N, found: bits.len() })
        } else {
            Ok(Bits(bits.try_into().unwrap()))
        }
    }
}

impl<const N: usize> Bits<N> {
    const BITS_PER_BYTE: usize = 8;

    pub fn is_zero(&self) -> bool {
        for bit in self.0 {
            if bit.on() {
                return false;
            }
        }
        true
    }

    pub fn zero() -> Self {
        Self([Bit::ZERO; N])
    }

    pub fn is_unit(&self) -> bool {
        for i in 0..(self.0.len() - 1) {
            if self.0[i].on() {
                return false;
            }
        }
        self.0[self.0.len() - 1].on()
    }

    pub fn unit() -> Self {
        let mut bits = [Bit::ZERO; N];
        bits[N - 1] = Bit::ONE;
        Self(bits)
    }
    
    pub fn to_u8(&self) -> u8 {
        if N > 8 {
            panic!("Bits size ({}) is too big for a u8 value", N);
        }
        let mut num = 0;
        let mut bit_value = 2u8.pow((self.0.len() - 1) as u32);
        for bit in self.0 {
            if bit.on() {
                num += bit_value;
            }
            bit_value /= 2;
        }
        num
    }

    pub fn to_u16(&self) -> u16 {
        if N > 16 {
            panic!("Bits size ({}) is too big for a u16 value", N);
        }
        let mut num = 0;
        let mut bit_value = 2u16.pow((self.0.len() - 1) as u32);
        for bit in self.0 {
            if bit.on() {
                num += bit_value;
            }
            bit_value /= 2;
        }
        num
    }

    pub fn from_u8(mut byte: u8) -> Bits<N> {
        if N > 8 {
            panic!("Bits size ({}) is too big for a u8 value", N);
        }
        if N != 8 && byte >= 2u8.pow(N as u32) {
            panic!("Bits size ({}) is too small for u8 value {}", N, byte);
        }
        let mut bits = [Bit::ZERO; N];
        let mut bit_value = 2u8.pow((bits.len() - 1) as u32);
        for bit_index in 0..bits.len() {
            if byte >= bit_value {
                byte -= bit_value;
                bits[bit_index] = Bit::ONE;
            }
            bit_value /= 2;
        }
        Bits(bits)
    }

    pub fn from_u16(mut two_bytes: u16) -> Bits<N> {
        if N > 16 {
            panic!("Bits size ({}) is too big for a u16 value", N);
        }
        if N != 16 && two_bytes >= 2u16.pow(N as u32) {
            panic!("Bits size ({}) is too small for u16 value {}", N, two_bytes);
        }
        let mut bits = [Bit::ZERO; N];
        let mut bit_value = 2u16.pow((bits.len() - 1) as u32);
        for bit_index in 0..bits.len() {
            if two_bytes >= bit_value {
                two_bytes -= bit_value;
                bits[bit_index] = Bit::ONE;
            }
            bit_value /= 2;
        }
        Bits(bits)
    }
}
