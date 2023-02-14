use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson};
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

#[derive(Serialize, Deserialize)]
pub struct System {
    //TODO (SYSTEM) check & write system checksums
    //TODO (SYSTEM) dont serialize padding #[serde(skip_serializing_if="Bits::is_unit", default="Bits::unit")]
    //TODO (SYSTEM) split up structured json, use "sys_common" for name of common
    unused1: Bits<16>, // 2 bytes padding ("00000000 00000001")
    common: Common, // 10 bytes
    unused2: Bits<32>, // 2 bytes Common checksum, 2 bytes padding
    compressor: Compressor, // 14 bytes
    unused3: Bits<32>, // 2 bytes Compressor checksum, 2 bytes padding
    v_link: VLink, // 4 bytes
    unused4: Bits<32>, // 2 bytes VLink checksum, 2 bytes padding 
    favorites: Favorites, // 76 bytes
    unused5: Bits<32>, // 2 bytes Favorites checksum, 2 bytes padding
    switch_assign: SwitchAssign, // 20 bytes
    unused6: Bits<16>, // 2 bytes SwitchAssign checksum
    #[serde(with = "serialize_chars_as_string")]
    hardware_version: [char; 16] // 16 bytes
}

impl Bytes<160> for System {
    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> {
        BitStream::read_fixed(bytes, |data| {
            let unused1 = data.get_bits();
            let common = Common::from_bytes(data.get_bytes())?;
            let unused2 = data.get_bits();
            let compressor = Compressor::from_bytes(data.get_bytes())?;
            let unused3 = data.get_bits();
            let v_link = VLink::from_bytes(data.get_bytes())?;
            let unused4 = data.get_bits();
            let favorites = Favorites::from_bytes(data.get_bytes())?;
            let unused5 = data.get_bits();
            let switch_assign = SwitchAssign::from_bytes(data.get_bytes())?;
            let unused6 = data.get_bits();
            let mut hardware_version = [char::default(); 16];
            for i in 0..hardware_version.len() {
                hardware_version[i] = data.get_char::<8>()?;
            }
            Ok(Self {
                unused1,
                common,
                unused2,
                compressor,
                unused3,
                v_link,
                unused4,
                favorites,
                unused5,
                switch_assign,
                unused6,
                hardware_version
            })
        })
    }

    fn to_bytes(&self) -> Result<Box<[u8; 160]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_bits(&self.unused1);
            bs.set_bytes(self.common.to_bytes()?);
            bs.set_bits(&self.unused2);
            bs.set_bytes(self.compressor.to_bytes()?);
            bs.set_bits(&self.unused3);
            bs.set_bytes(self.v_link.to_bytes()?);
            bs.set_bits(&self.unused4);
            bs.set_bytes(self.favorites.to_bytes()?);
            bs.set_bits(&self.unused5);
            bs.set_bytes(self.switch_assign.to_bytes()?);
            bs.set_bits(&self.unused6);
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

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json())
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}
