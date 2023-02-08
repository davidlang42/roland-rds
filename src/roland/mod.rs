use std::fmt::Debug;

use crate::bits::BitStream;
use crate::bytes::{Bytes, BytesError};

pub mod rd300nx;
pub mod live_set;
pub mod layers;

fn validate(ch: char) -> Result<char, BytesError> {
    // Roland keyboards use chars 32 ' ' through 126 '~' inclusive
    let ascii = ch as u8;
    if ascii < 32 || ascii > 126 {
        Err(BytesError::InvalidCharacter(ch))
    } else {
        Ok(ch)
    }
}

fn max(value: u8, max: u8) -> u8 {
    if value > max {
        panic!("Tried to write out of range value: {} (max {})", value, max);
    }
    value
}

fn in_range(value: u8, min: u8, max: u8) -> u8 {
    if value < min || value > max {
        panic!("Tried to write out of range value: {} ({} - {})", value, min, max);
    }
    value
}

fn parse_many<const B: usize, T: Bytes<B> + Debug, const N: usize>(data: &mut BitStream) -> Result<Box<[T; N]>, BytesError> {
    let mut parsed = Vec::new();
    for _ in 0..N {
        parsed.push(T::from_bytes(data.get_bytes())?);
    }
    Ok(Box::new(parsed.try_into().unwrap()))
}