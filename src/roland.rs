use crate::bits::{Bits, BitStream};

// 183762 bytes
pub struct RD300NX {
    pub live_sets: [LiveSet; Self::LIVE_SETS], // 183600 bytes
    footer: Bits<1280> // 160 bytes
    // checksum: 2 bytes
}

// 2160 bytes
#[derive(Debug)]
pub struct LiveSet {
    name: [char; 16], // 14 bytes
    other: Bits<17160>, // 2145 bytes
    // checksum: 1 byte
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEndOfStream(usize),
    IncorrectCheckSum {
        expected: Vec<u8>,
        found: Vec<u8>
    },
    UnnessesaryTrailingBits(String)
}

impl RD300NX {
    const LIVE_SETS: usize = 85;

    pub fn parse(mut data: BitStream) -> Result<Self, ParseError> {
        let mut live_sets = Vec::new();
        for _ in 0..Self::LIVE_SETS {
            live_sets.push(LiveSet::parse(&mut data)?);
        }
        let footer = data.get_bits().unwrap();
        let found_check_sum = [
            data.get_u8::<8>().ok_or(ParseError::UnexpectedEndOfStream(data.offset()))?,
            data.get_u8::<8>().ok_or(ParseError::UnexpectedEndOfStream(data.offset()))?
        ];
        let rds = Self {
            live_sets: live_sets.try_into().unwrap(),
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
        if !data.eof() {
            return Err(ParseError::UnnessesaryTrailingBits(format!("{}", data)));
        }
        Ok(rds)
    }

    pub fn to_bytes(&self) -> Vec<u8> {//TODO make this a Bytes<N> trait
        let mut bytes = Vec::new();
        for i in 0..Self::LIVE_SETS {
            bytes.append(&mut self.live_sets[i].to_bytes());
        }
        for byte in self.footer.to_bytes() {
            bytes.push(byte);
        }
        let check_sum = Self::check_sum(&bytes);
        for byte in check_sum.to_be_bytes() {
            bytes.push(byte);
        }
        bytes
    }

    fn check_sum(bytes_without_checksum: &Vec<u8>) -> u16 {
        let mut sum: u16 = 0;
        for byte in bytes_without_checksum {
            sum += *byte as u16; //intentional overflow
        }
        sum
    }
}

impl LiveSet {
    pub fn name_string(&self) -> String {
        self.name.iter().collect()
    }

    pub fn parse(data: &mut BitStream) -> Result<Self, ParseError> {
        let mut name = [char::default(); 16];
        for i in 0..name.len() {
            name[i] = data.get_char().ok_or(ParseError::UnexpectedEndOfStream(data.offset()))?;
        }
        let other = data.get_bits().ok_or(ParseError::UnexpectedEndOfStream(data.offset()))?;
        Ok(Self {
            name,
            other
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Bits::<7>::compress(self.name).to_bytes();
        bytes.append(&mut self.other.to_bytes());
        let check_sum = Self::check_sum(&bytes);
        bytes.push(check_sum);
        bytes
    }

    fn check_sum(bytes_without_checksum: &Vec<u8>) -> u8 {
        let mut sum: u8 = 0;
        for byte in bytes_without_checksum {
            sum += *byte as u8; //intentional overflow
        }
        sum
    }
}