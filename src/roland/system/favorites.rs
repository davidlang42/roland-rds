use std::fmt::Debug;
use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{StructuredJson, Json, StructuredJsonError};
use crate::roland::types::enums::PatchCategory;
use crate::roland::types::numeric::{OneIndexedU16, OneIndexedU8};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Favorites {
    #[validate]
    one_touch_piano_current_number: OneIndexedU8,
    #[validate]
    unused_one_touch_piano_current_number: [OneIndexedU8; 2],
    #[validate]
    one_touch_e_piano_current_number: OneIndexedU8,
    #[validate]
    unused_one_touch_e_piano_current_number: [OneIndexedU8; 2],
    #[validate]
    bank_a: Bank,
    #[validate]
    bank_b: Bank,
    #[validate]
    bank_c: Bank,
    #[validate]
    bank_d: Bank,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<6>::zero")]
    unused: Bits<6>
}

impl Favorites {
    pub fn all_banks(&self) -> Vec<&Bank> {
        vec![
            &self.bank_a,
            &self.bank_b,
            &self.bank_c,
            &self.bank_d
        ]
    }
}

impl Bytes<76> for Favorites {
    fn to_bytes(&self) -> Result<Box<[u8; Self::BYTE_SIZE]>, BytesError> {
        BitStream::write_fixed(|bits| {
            bits.set_u8::<7>(self.one_touch_piano_current_number.into(), 0, 127)?;
            for value in self.unused_one_touch_piano_current_number {
                bits.set_u8::<7>(value.into(), 0, 127)?;
            }
            bits.set_u8::<7>(self.one_touch_e_piano_current_number.into(), 0, 127)?;
            for value in self.unused_one_touch_e_piano_current_number {
                bits.set_u8::<7>(value.into(), 0, 127)?;
            }
            for bank in self.all_banks() {
                bits.set_bits(&bank.to_bits()?);
            }
            Ok(bits.set_bits(&self.unused))
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let one_touch_piano_current_number = bs.get_u8::<7>(0, 127)?.into();
            let mut unused_one_touch_piano_current_number = [OneIndexedU8::default(); 2];
            for i in 0..unused_one_touch_piano_current_number.len() {
                unused_one_touch_piano_current_number[i] = bs.get_u8::<7>(0, 127)?.into();
            }
            let one_touch_e_piano_current_number = bs.get_u8::<7>(0, 127)?.into();
            let mut unused_one_touch_e_piano_current_number = [OneIndexedU8::default(); 2];
            for i in 0..unused_one_touch_e_piano_current_number.len() {
                unused_one_touch_e_piano_current_number[i] = bs.get_u8::<7>(0, 127)?.into();
            }
            let bank_a = Bank::from_bits(bs.get_bits())?;
            let bank_b = Bank::from_bits(bs.get_bits())?;
            let bank_c = Bank::from_bits(bs.get_bits())?;
            let bank_d = Bank::from_bits(bs.get_bits())?;
            Ok(Self {
                one_touch_piano_current_number,
                unused_one_touch_piano_current_number,
                one_touch_e_piano_current_number,
                unused_one_touch_e_piano_current_number,
                bank_a,
                bank_b,
                bank_c,
                bank_d,
                unused: bs.get_bits()
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Bank {
    #[validate]
    favorites: [Favorite; Self::USED_FAVORITES],
    #[validate]
    unused_favorites: [Favorite; Self::FAVORITES_PER_BANK - Self::USED_FAVORITES]
}

impl Bank {
    const USED_FAVORITES: usize = 6;
    const FAVORITES_PER_BANK: usize = 10;
    const BITS_SIZE: usize = Favorite::BITS_SIZE * Self::FAVORITES_PER_BANK;

    fn to_bits(&self) -> Result<Bits<{Self::BITS_SIZE}>, BytesError> {
        BitStream::write_fixed_bits(|bits| {
            for favorite in &self.favorites {
                bits.set_bits(&favorite.to_bits()?);
            }
            for unused_favorite in &self.unused_favorites {
                bits.set_bits(&unused_favorite.to_bits()?);
            }
            Ok(())
        })
    }

    fn from_bits(bits: Bits<{Self::BITS_SIZE}>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed_bits(bits, |data| {
            let mut favorites = Vec::new();
            for _ in 0..Self::USED_FAVORITES {
                favorites.push(Favorite::from_bits(data.get_bits())?);
            }
            let mut unused_favorites = Vec::new();
            for _ in 0..Self::FAVORITES_PER_BANK - Self::USED_FAVORITES {
                unused_favorites.push(Favorite::from_bits(data.get_bits())?);
            }
            Ok(Self {
                favorites: favorites.try_into().unwrap(),
                unused_favorites: unused_favorites.try_into().unwrap()
            })
        })
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Favorite {
    category: PatchCategory,
    #[validate]
    live_set_number: OneIndexedU16
}

impl Favorite {
    const BITS_SIZE: usize = 14;

    fn to_bits(&self) -> Result<Bits<{Self::BITS_SIZE}>, BytesError> {
        BitStream::write_fixed_bits(|bits| {
            bits.set_u8::<2>(self.category.into(), 0, 3)?;
            bits.set_u16::<12>(self.live_set_number.into(), 0, 299)?;
            Ok(())
        })
    }

    fn from_bits(bits: Bits<{Self::BITS_SIZE}>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed_bits(bits, |data| {
            Ok(Self {
                category: data.get_u8::<2>(0, 3)?.into(),
                live_set_number: data.get_u16::<12>(0, 299)?.into()
            })
        })
    }
}
