use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};

use super::super::{max, in_range_u16};

#[derive(Serialize, Deserialize, Debug)]
pub struct Favorites {
    one_touch_piano_current_number: [u8; 3], // each max 127
    one_touch_e_piano_current_number: [u8; 3], // each max 127
    banks: [Bank; Self::BANKS],
    unused: Bits<6>
}

impl Favorites {
    const BANKS: usize = 4;
}

impl Bytes<76> for Favorites {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bits| {
            for value in self.one_touch_piano_current_number {
                bits.set_u8::<7>(value);
            }
            for value in self.one_touch_e_piano_current_number {
                bits.set_u8::<7>(value);
            }
            for bank in &self.banks {
                bits.set_bits(&bank.to_bits());
            }
            bits.set_bits(&self.unused);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let mut one_touch_piano_current_number = [0; 3];
            for i in 0..one_touch_piano_current_number.len() {
                one_touch_piano_current_number[i] = data.get_u8::<7>();
            }
            let mut one_touch_e_piano_current_number = [0; 3];
            for i in 0..one_touch_e_piano_current_number.len() {
                one_touch_e_piano_current_number[i] = data.get_u8::<7>();
            }
            let mut banks = Vec::new();
            for _ in 0..Self::BANKS {
                banks.push(Bank::from_bits(data.get_bits())?);
            }
            Ok(Self {
                one_touch_piano_current_number,
                one_touch_e_piano_current_number,
                banks: banks.try_into().unwrap(),
                unused: data.get_bits()
            })
        })
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json())
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank([Favorite; Self::FAVORITES_PER_BANK]);

impl Bank {
    const FAVORITES_PER_BANK: usize = 10;
    const BITS_SIZE: usize = Favorite::BITS_SIZE * Self::FAVORITES_PER_BANK;

    fn to_bits(&self) -> Bits<{Self::BITS_SIZE}> {
        BitStream::write_fixed_bits(|bits| {
            for favorite in &self.0 {
                bits.set_bits(&favorite.to_bits());
            }
        })
    }

    fn from_bits(bits: Bits<{Self::BITS_SIZE}>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed_bits(bits, |data| {
            let mut favorites = Vec::new();
            for _ in 0..Self::FAVORITES_PER_BANK {
                favorites.push(Favorite::from_bits(data.get_bits())?);
            }
            Ok(Self(favorites.try_into().unwrap()))
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Favorite {
    category: u8, // max 3, (One Touch Piano, One Touch E.Piano, Preset, User)
    live_set_number: u16 // max 299 (1-300)
}

impl Favorite {
    const BITS_SIZE: usize = 14;

    fn to_bits(&self) -> Bits<{Self::BITS_SIZE}> {
        BitStream::write_fixed_bits(|bits| {
            bits.set_u8::<2>(max(self.category, 3));
            bits.set_u16::<12>(in_range_u16(self.live_set_number, 0, 299));
        })
    }

    fn from_bits(bits: Bits<{Self::BITS_SIZE}>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed_bits(bits, |data| {
            Ok(Self {
                category: data.get_u8::<2>(),
                live_set_number: data.get_u16::<12>()
            })
        })
    }
}
