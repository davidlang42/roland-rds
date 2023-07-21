use schemars::JsonSchema;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError};
use crate::json::serialize_chars_as_string;
use self::common::Common;
use self::compressor::Compressor;
use self::favorites::Favorites;
use self::switch_assign::SwitchAssign;
use self::v_link::VLink;

mod favorites;
mod common;
mod v_link;
mod switch_assign;
mod compressor;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct System {
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding1: Bits<16>, // 2 bytes padding
    common: Common, // 10 bytes
    unsure2: Bits<16>, // 2 bytes Common checksum?
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding2: Bits<16>, // 2 bytes padding
    compressor: Compressor, // 14 bytes
    unsure3: Bits<16>, // 2 bytes Compressor checksum?
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding3: Bits<16>, // 2 bytes padding
    v_link: VLink, // 4 bytes
    unsure4: Bits<16>, // 2 bytes VLink checksum?
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding4: Bits<16>, // 2 bytes padding
    favorites: Favorites, // 76 bytes
    unsure5: Bits<16>, // 2 bytes Favorites checksum?
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding5: Bits<16>, // 2 bytes padding
    switch_assign: SwitchAssign, // 20 bytes
    unsure6: Bits<16>, // 2 bytes SwitchAssign checksum?
    #[serde(deserialize_with = "serialize_chars_as_string::deserialize")]
    #[serde(serialize_with = "serialize_chars_as_string::serialize")]
    #[schemars(with = "serialize_chars_as_string::StringSchema::<16>")]
    hardware_version: [char; 16] // 16 bytes
}

impl Bytes<160> for System {
    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> {
        BitStream::read_fixed(bytes, |data| {
            let padding1 = data.get_bits();
            let common = Common::from_bytes(data.get_bytes())?;
            let unsure2 = data.get_bits();
            let padding2 = data.get_bits();
            let compressor = Compressor::from_bytes(data.get_bytes())?;
            let unsure3 = data.get_bits();
            let padding3 = data.get_bits();
            let v_link = VLink::from_bytes(data.get_bytes())?;
            let unsure4 = data.get_bits();
            let padding4 = data.get_bits();
            let favorites = Favorites::from_bytes(data.get_bytes())?;
            let unsure5 = data.get_bits();
            let padding5 = data.get_bits();
            let switch_assign = SwitchAssign::from_bytes(data.get_bytes())?;
            let unsure6 = data.get_bits();
            let mut hardware_version = [char::default(); 16];
            for i in 0..hardware_version.len() {
                hardware_version[i] = data.get_char::<8>()?;
            }
            Ok(Self {
                padding1,
                common,
                unsure2,
                padding2,
                compressor,
                unsure3,
                padding3,
                v_link,
                unsure4,
                padding4,
                favorites,
                unsure5,
                padding5,
                switch_assign,
                unsure6,
                hardware_version
            })
        })
    }

    fn to_bytes(&self) -> Result<Box<[u8; 160]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_bits(&self.padding1);
            bs.set_bytes(self.common.to_bytes()?);
            bs.set_bits(&self.unsure2);
            bs.set_bits(&self.padding2);
            bs.set_bytes(self.compressor.to_bytes()?);
            bs.set_bits(&self.unsure3);
            bs.set_bits(&self.padding3);
            bs.set_bytes(self.v_link.to_bytes()?);
            bs.set_bits(&self.unsure4);
            bs.set_bits(&self.padding4);
            bs.set_bytes(self.favorites.to_bytes()?);
            bs.set_bits(&self.unsure5);
            bs.set_bits(&self.padding5);
            bs.set_bytes(self.switch_assign.to_bytes()?);
            bs.set_bits(&self.unsure6);
            for ch in self.hardware_version {
                bs.set_char::<8>(ch)?;
            }
            Ok(())
        })
    }
}

impl Json for System {
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
