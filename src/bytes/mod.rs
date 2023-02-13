use std::fmt::Debug;

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
    InvalidCharacter(char)
}

pub trait Bytes<const N: usize> {
    const BYTE_SIZE: usize = N;

    fn to_bytes(&self) -> Box<[u8; N]>;
    fn from_bytes(bytes: Box<[u8; N]>) -> Result<Self, BytesError> where Self: Sized;
}
