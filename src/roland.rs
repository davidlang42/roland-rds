use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, ParseError};

#[derive(Serialize, Deserialize)]
pub struct RD300NX {
    //TODO go back to using fixed length arrays
    // pub user_sets: Box<[LiveSet; Self::USER_SETS]>,
    // pub bank_a: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    // pub bank_b: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    // pub bank_c: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    // pub bank_d: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    pub user_sets: Vec<LiveSet>,
    pub bank_a: Vec<LiveSet>,
    pub bank_b: Vec<LiveSet>,
    pub bank_c: Vec<LiveSet>,
    pub bank_d: Vec<LiveSet>,
    pub current: LiveSet,
    footer: Footer //TODO contains hardware version as 8-bit chars rather than 7-bit
    // checksum: 2 bytes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveSet {
    //TODO serialize as string
    name: [char; 16], // 14 bytes
    other: Bits<17160>, // 2145 bytes
    // checksum: 1 byte
}

impl Bytes<183762> for RD300NX {
    fn parse(bytes: [u8; Self::BYTE_SIZE]) -> Result<Self, ParseError> {
        let mut data = BitStream::read(bytes);
        let user_sets = parse_many(&mut data, Self::USER_SETS)?;
        let bank_a = parse_many(&mut data, Self::FAVOURITES_PER_BANK)?;
        let bank_b = parse_many(&mut data, Self::FAVOURITES_PER_BANK)?;
        let bank_c = parse_many(&mut data, Self::FAVOURITES_PER_BANK)?;
        let bank_d = parse_many(&mut data, Self::FAVOURITES_PER_BANK)?;
        let current = LiveSet::parse(data.get_bytes())?;
        let footer = Footer::parse(data.get_bytes())?;
        let found_check_sum = [
            data.get_u8::<8>(),
            data.get_u8::<8>(),
        ];
        if !data.eof() {
            panic!("Failed to read all {} bytes", Self::BYTE_SIZE);
        }
        let rds = Self {
            user_sets,
            bank_a,
            bank_b,
            bank_c,
            bank_d,
            current,
            footer
        };
        let bytes = rds.to_bytes();
        let expected_check_sum: [u8; 2] = bytes[(bytes.len()-2)..bytes.len()].try_into().unwrap();
        if found_check_sum != expected_check_sum {
            return Err(ParseError::IncorrectCheckSum {
                expected: expected_check_sum.into_iter().collect(),
                found: found_check_sum.into_iter().collect()
            });
        }
        Ok(rds)
    }

    fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        let mut bytes = Vec::new();
        for live_set in self.all_live_sets() {
            for byte in live_set.to_bytes() {
                bytes.push(byte);
            }
        }
        for byte in self.footer.to_bytes() {
            bytes.push(byte);
        }
        let check_sum = Self::check_sum(&bytes);
        for byte in check_sum.to_be_bytes() {
            bytes.push(byte);
        }
        bytes.try_into().unwrap()
    }
}

impl RD300NX {
    const USER_SETS: usize = 60;
    const FAVOURITES_PER_BANK: usize = 6;

    fn check_sum(bytes_without_checksum: &Vec<u8>) -> u16 {
        let mut sum: u16 = 0;
        for byte in bytes_without_checksum {
            sum = sum.wrapping_add(*byte as u16);
        }
        sum
    }

    pub fn all_live_sets(&self) -> Vec<&LiveSet> {
        let mut all: Vec<&LiveSet> = self.user_sets.iter().chain(self.bank_a.iter()).chain(self.bank_b.iter()).chain(self.bank_c.iter()).chain(self.bank_d.iter()).collect();
        all.push(&self.current);
        all
    }
}

impl LiveSet {
    pub fn name_string(&self) -> String {
        self.name.iter().collect()
    }

    fn check_sum(bytes_without_checksum: &Vec<u8>) -> u8 {
        let mut sum: u8 = 0;
        for byte in bytes_without_checksum {
            sum = sum.wrapping_add(*byte);
        }
        u8::MAX - sum + 1
    }
}

impl Bytes<2160> for LiveSet {
    fn parse(bytes: [u8; Self::BYTE_SIZE]) -> Result<Self, ParseError> {
        let mut data = BitStream::read(bytes);
        let mut name = [char::default(); 16];
        for i in 0..name.len() {
            name[i] = validate(data.get_char())?;
        }
        let other = data.get_bits();
        let found_check_sum = data.get_u8::<8>();
        let live_set = Self {
            name,
            other
        };
        let bytes = live_set.to_bytes();
        let expected_check_sum = bytes[bytes.len() - 1];
        if found_check_sum != expected_check_sum {
            return Err(ParseError::IncorrectCheckSum {
                expected: vec![expected_check_sum],
                found: vec![found_check_sum]
            });
        }
        Ok(live_set)
    }

    fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        let mut bytes = Bits::<7>::compress(self.name).to_bytes();
        bytes.append(&mut self.other.to_bytes());
        let check_sum = Self::check_sum(&bytes);
        bytes.push(check_sum);
        bytes.try_into().unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Footer(Bits<1280>);

impl Bytes<160> for Footer {
    fn parse(bytes: [u8; Self::BYTE_SIZE]) -> Result<Self, ParseError> {
        let mut data = BitStream::read(bytes);
        let bits = data.get_bits();
        Ok(Self(bits.into()))
    }

    fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        self.0.to_bytes().try_into().unwrap()
    }
}

fn validate(ch: char) -> Result<char, ParseError> {
    // Roland keyboards use chars 32 ' ' through 126 '~' inclusive
    let ascii = ch as u8;
    if ascii < 32 || ascii > 126 {
        Err(ParseError::InvalidCharacter(ch))
    } else {
        Ok(ch)
    }
}

fn parse_many<const B: usize, T: Bytes<B> + Debug>(data: &mut BitStream, n: usize) -> Result<Vec<T>, ParseError> {
    let mut parsed = Vec::new();
    for _ in 0..n {
        parsed.push(T::parse(data.get_bytes())?);
    }
    Ok(parsed)
}