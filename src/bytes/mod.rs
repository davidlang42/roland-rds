use std::fmt::{Debug, Display};
use std::error::Error;

mod bits;
mod bit_stream;

pub use bits::{Bit, Bits};
pub use bit_stream::BitStream;

#[derive(Debug)]
pub enum BytesError {
    IncorrectCheckSum {
        expected: Vec<u8>,
        found: Vec<u8>
    },
    InvalidCharacter(char),
    ValueOutOfRangeU8 {
        value: u8,
        min: u8,
        max: u8
    },
    ValueOutOfRangeU16 {
        value: u16,
        min: u16,
        max: u16
    },
    InvalidTone {
        msb: u8,
        lsb: u8,
        pc: u8
    }
}

impl Error for BytesError {}

impl Display for BytesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Bytes<const N: usize> {
    const BYTE_SIZE: usize = N;

    fn to_bytes(&self) -> Result<Box<[u8; N]>, BytesError>;
    fn from_bytes(bytes: Box<[u8; N]>) -> Result<Self, BytesError> where Self: Sized;

    fn array_from_bytes<const A: usize>(data: &mut BitStream) -> Result<Box<[Self; A]>, BytesError> where Self: Sized + Debug {
        let mut parsed = Vec::new();
        for _ in 0..A {
            parsed.push(Self::from_bytes(data.get_bytes())?);
        }
        Ok(parsed.try_into().unwrap())
    }
}
