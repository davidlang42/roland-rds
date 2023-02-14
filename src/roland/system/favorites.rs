use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{StructuredJson, Json, StructuredJsonError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Favorites {
    one_touch_piano_current_number: [u8; 3], // each max 127
    one_touch_e_piano_current_number: [u8; 3], // each max 127
    banks: [Bank; Self::BANKS],
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused: Bits<6>
}

impl Favorites {
    const BANKS: usize = 4;
}

impl Bytes<76> for Favorites {
    fn to_bytes(&self) -> Result<Box<[u8; 76]>, BytesError> {
        BitStream::write_fixed(|bits| {
            for value in self.one_touch_piano_current_number {
                bits.set_u8::<7>(value, 0, 127)?;
            }
            for value in self.one_touch_e_piano_current_number {
                bits.set_u8::<7>(value, 0, 127)?;
            }
            for bank in &self.banks {
                bits.set_bits(&bank.to_bits()?);
            }
            Ok(bits.set_bits(&self.unused))
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let mut one_touch_piano_current_number = [0; 3];
            for i in 0..one_touch_piano_current_number.len() {
                one_touch_piano_current_number[i] = data.get_u8::<7>(0, 127)?;
            }
            let mut one_touch_e_piano_current_number = [0; 3];
            for i in 0..one_touch_e_piano_current_number.len() {
                one_touch_e_piano_current_number[i] = data.get_u8::<7>(0, 127)?;
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
}

impl Json for Favorites {
    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Result<Self, StructuredJsonError> {
        Self::from_json(structured_json.to_single_json()?).map_err(|e| e.into())
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn from_json(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank([Favorite; Self::FAVORITES_PER_BANK]);

impl Bank {
    const FAVORITES_PER_BANK: usize = 10;
    const BITS_SIZE: usize = Favorite::BITS_SIZE * Self::FAVORITES_PER_BANK;

    fn to_bits(&self) -> Result<Bits<{Self::BITS_SIZE}>, BytesError> {
        BitStream::write_fixed_bits(|bits| {
            for favorite in &self.0 {
                bits.set_bits(&favorite.to_bits()?);
            }
            Ok(())
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

    fn to_bits(&self) -> Result<Bits<{Self::BITS_SIZE}>, BytesError> {
        BitStream::write_fixed_bits(|bits| {
            bits.set_u8::<2>(self.category, 0, 3)?;
            bits.set_u16::<12>(self.live_set_number, 0, 299)?;
            Ok(())
        })
    }

    fn from_bits(bits: Bits<{Self::BITS_SIZE}>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed_bits(bits, |data| {
            Ok(Self {
                category: data.get_u8::<2>(0, 3)?,
                live_set_number: data.get_u16::<12>(0, 299)?
            })
        })
    }
}
