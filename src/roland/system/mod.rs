use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use crate::json::serialize_chars_as_string;
use self::common::Common;
use self::favorites::Favorites;
use self::v_link::VLink;

use super::validate;

mod favorites;
mod common;
mod v_link;

#[derive(Serialize, Deserialize)]
pub struct System {
    unused1: Bits<16>, // 2 bytes
    common: Common, // 8 bytes
    compressor: Bits<192>, // 103 bits, 24 bytes?
    v_link: VLink, // 4 bytes
    unused2: Bits<32>, // 4 bytes
    favorites: Favorites, // 76 bytes
    switch_assign: Bits<208>, // 150 bits, 20 bytes?
    #[serde(with = "serialize_chars_as_string")]
    hardware_version: [char; 16]
}

impl Bytes<160> for System {
    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> {
        let mut data = BitStream::read(bytes);
        let unused1 = data.get_bits();
        let common = Common::from_bytes(Box::new(data.get_bytes()))?;
        let compressor = data.get_bits();
        let v_link = VLink::from_bytes(Box::new(data.get_bytes()))?;
        let unused2 = data.get_bits();
        let favorites = Favorites::from_bytes(Box::new(data.get_bytes()))?;
        let switch_assign = data.get_bits();
        let mut hardware_version = [char::default(); 16];
        for i in 0..hardware_version.len() {
            hardware_version[i] = validate(data.get_u8::<8>() as char)?;
        }
        Ok(Self {
            unused1,
            common,
            compressor,
            v_link,
            unused2,
            favorites,
            switch_assign,
            hardware_version
        })
    }

    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bytes = self.unused1.to_bytes();
        for byte in *self.common.to_bytes() {
            bytes.push(byte);
        }
        for byte in self.compressor.to_bytes() {
            bytes.push(byte);
        }
        for byte in *self.v_link.to_bytes() {
            bytes.push(byte);
        }
        for byte in self.unused2.to_bytes() {
            bytes.push(byte);
        }
        for byte in *self.favorites.to_bytes() {
            bytes.push(byte);
        }
        for byte in self.switch_assign.to_bytes() {
            bytes.push(byte);
        }
        for ch in self.hardware_version {
            bytes.push(ch as u8);
        }
        bytes.try_into().unwrap()
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
