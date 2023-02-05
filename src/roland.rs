use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, ParseError, StructuredJson};
use crate::json::serialize_chars_as_string;
use crate::json::serialize_array_as_vec;

#[derive(Serialize, Deserialize)]
pub struct RD300NX {
    #[serde(with = "serialize_array_as_vec")]
    pub user_sets: Box<[LiveSet; Self::USER_SETS]>,
    pub bank_a: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    pub bank_b: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    pub bank_c: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    pub bank_d: Box<[LiveSet; Self::FAVOURITES_PER_BANK]>,
    pub current: LiveSet,
    footer: Footer
    // checksum: 2 bytes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LiveSet {
    #[serde(with = "serialize_chars_as_string")]
    name: [char; 16], // 14 bytes
    other: Bits<17160>, // 2145 bytes
    // checksum: 1 byte
}

impl Bytes<183762> for RD300NX {
    fn from_bytes(bytes: [u8; Self::BYTE_SIZE]) -> Result<Self, ParseError> {
        let mut data = BitStream::read(bytes);
        let user_sets = parse_many(&mut data)?;
        let bank_a = parse_many(&mut data)?;
        let bank_b = parse_many(&mut data)?;
        let bank_c = parse_many(&mut data)?;
        let bank_d = parse_many(&mut data)?;
        let current = LiveSet::from_bytes(data.get_bytes())?;
        let footer = Footer::from_bytes(data.get_bytes())?;
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

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::NestedCollection(vec![
            ("user_sets".to_string(), StructuredJson::from_collection(self.user_sets.as_slice(), |ls| ls.name_string())),
            ("bank_a".to_string(), StructuredJson::from_collection(self.bank_a.as_slice(), |ls| ls.name_string())),
            ("bank_b".to_string(), StructuredJson::from_collection(self.bank_b.as_slice(), |ls| ls.name_string())),
            ("bank_c".to_string(), StructuredJson::from_collection(self.bank_c.as_slice(), |ls| ls.name_string())),
            ("bank_d".to_string(), StructuredJson::from_collection(self.bank_d.as_slice(), |ls| ls.name_string())),
            ("current".to_string(), self.current.to_structured_json()),
            ("footer".to_string(), self.footer.to_structured_json())
        ])
    }

    fn from_structured_json(mut structured_json: StructuredJson) -> Self {
        let user_sets = structured_json.extract("user_sets").to_array();
        let bank_a = structured_json.extract("bank_a").to_array();
        let bank_b = structured_json.extract("bank_b").to_array();
        let bank_c = structured_json.extract("bank_c").to_array();
        let bank_d = structured_json.extract("bank_d").to_array();
        let current = structured_json.extract("current").to();
        let footer = structured_json.extract("footer").to();
        structured_json.done();
        Self {
            user_sets,
            bank_a,
            bank_b,
            bank_c,
            bank_d,
            current,
            footer
        }
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
    fn from_bytes(bytes: [u8; Self::BYTE_SIZE]) -> Result<Self, ParseError> {
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

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(serde_json::to_string(&self).expect("Error serializing JSON"))//TODO duplicated code
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        let text = structured_json.to_single_json();
        serde_json::from_str(&text).expect("Error deserializing JSON") //TODO duplicated code
    }
}

#[derive(Serialize, Deserialize)]
pub struct Footer {
    other: Bits<1152>,
    #[serde(with = "serialize_chars_as_string")]
    hardware_version: [char; 16]
}

impl Bytes<160> for Footer {
    fn from_bytes(bytes: [u8; Self::BYTE_SIZE]) -> Result<Self, ParseError> {
        let mut data = BitStream::read(bytes);
        let other = data.get_bits();
        let mut hardware_version = [char::default(); 16];
        for i in 0..hardware_version.len() {
            hardware_version[i] = validate(data.get_u8::<8>() as char)?;
        }
        Ok(Self {
            other,
            hardware_version
        })
    }

    fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        let mut bytes = self.other.to_bytes();
        for ch in self.hardware_version {
            bytes.push(ch as u8);
        }
        bytes.try_into().unwrap()
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(serde_json::to_string(&self).expect("Error serializing JSON"))//TODO duplicated code
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        let text = structured_json.to_single_json();
        serde_json::from_str(&text).expect("Error deserializing JSON") //TODO duplicated code
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

fn parse_many<const B: usize, T: Bytes<B> + Debug, const N: usize>(data: &mut BitStream) -> Result<Box<[T; N]>, ParseError> {
    let mut parsed = Vec::new();
    for _ in 0..N {
        parsed.push(T::from_bytes(data.get_bytes())?);
    }
    Ok(Box::new(parsed.try_into().unwrap()))
}