#[derive(Debug)]
pub enum ParseError {
    IncorrectCheckSum {
        expected: Vec<u8>,
        found: Vec<u8>
    },
    InvalidCharacter(char)
}

pub trait Bytes<const N: usize> {
    const BYTE_SIZE: usize = N;

    fn to_bytes(&self) -> [u8; N];
    fn from_bytes(bytes: [u8; N]) -> Result<Self, ParseError> where Self: Sized;
}