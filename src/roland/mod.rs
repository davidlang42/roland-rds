use std::fmt::Debug;

use crate::bits::BitStream;
use crate::bytes::{Bytes, BytesError};

pub mod rd300nx;
pub mod live_set;
pub mod layers;
pub mod tones;
pub mod system;

//TODO dont serialize unused bits if all zero (across entire project)
//TODO handle types better than all u8 (across entire project)
//TODO factor out common sets of settings between layers and live set common/etc, or system and live set
//TODO make microtune a map of non-default values (are there any other arrays which are really maps?)
//TODO look for other sections of json which are overly verbose and contain basically default data and figure out what to do with them
//TODO complete or action future decoding of each sub-section as issues and point back to the 700NX midi-implementation doc

fn validate(ch: char) -> Result<char, BytesError> { //TODO should we validate on the way in as well as out? probably need better error messages for invalid which include which # live set we are in, which field we are looking at
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

fn in_range_u16(value: u16, min: u16, max: u16) -> u16 {
    if value < min || value > max {
        panic!("Tried to write out of range value: {} ({} - {})", value, min, max);
    }
    value
}

fn parse_many<const B: usize, T: Bytes<B> + Debug, const N: usize>(data: &mut BitStream) -> Result<Box<[T; N]>, BytesError> {
    let mut parsed = Vec::new();
    for _ in 0..N {
        parsed.push(T::from_bytes(Box::new(data.get_bytes()))?);
    }
    Ok(parsed.try_into().unwrap())
}