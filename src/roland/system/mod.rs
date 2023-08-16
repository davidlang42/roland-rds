use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError, serialize_chars_as_string};
use crate::json::validation::valid_chars;
use super::sum_to_zero;
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

#[derive(Serialize, Deserialize, JsonSchema, Validate)]
pub struct System {
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding1: Bits<16>, // 2 bytes padding
    #[validate]
    pub common: Common, // 10 bytes
    checksum1: SystemCheckSum, // 2 bytes checksum
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding2: Bits<16>, // 2 bytes padding
    //#[validate]
    compressor: Compressor, // 14 bytes
    checksum2: SystemCheckSum, // 2 bytes checksum
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding3: Bits<16>, // 2 bytes padding
    //#[validate]
    v_link: VLink, // 4 bytes
    checksum3: SystemCheckSum, // 2 bytes checksum
    // 2 bytes VLink checksum?
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding4: Bits<16>, // 2 bytes padding
    #[validate]
    favorites: Favorites, // 76 bytes
    checksum4: SystemCheckSum, // 2 bytes checksum
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<16>::unit")]
    padding5: Bits<16>, // 2 bytes padding
    //#[validate]
    switch_assign: SwitchAssign, // 20 bytes
    checksum5: SystemCheckSum, // 2 bytes checksum
    #[serde(deserialize_with = "serialize_chars_as_string::deserialize")]
    #[serde(serialize_with = "serialize_chars_as_string::serialize")]
    #[schemars(with = "serialize_chars_as_string::StringSchema::<16>")]
    #[validate(custom = "valid_chars")]
    hardware_version: [char; 16] // 16 bytes
}

impl Bytes<160> for System {
    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> {
        BitStream::read_fixed(bytes, |data| {
            let padding1: Bits<16> = data.get_bits();
            let common = Common::from_bytes(data.get_bytes())?;
            let checksum1 = SystemCheckSum::read(data)?;
            let padding2 = data.get_bits();
            let compressor = Compressor::from_bytes(data.get_bytes())?;
            let checksum2 = SystemCheckSum::read(data)?;
            let padding3 = data.get_bits();
            let v_link = VLink::from_bytes(data.get_bytes())?;
            let checksum3 = SystemCheckSum::read(data)?;
            let padding4 = data.get_bits();
            let favorites = Favorites::from_bytes(data.get_bytes())?;
            let checksum4 = SystemCheckSum::read(data)?;
            let padding5 = data.get_bits();
            let switch_assign = SwitchAssign::from_bytes(data.get_bytes())?;
            let checksum5 = SystemCheckSum::read(data)?;
            let mut hardware_version = [char::default(); 16];
            for i in 0..hardware_version.len() {
                hardware_version[i] = data.get_char::<8>()?;
            }
            Ok(Self {
                padding1,
                common,
                checksum1,
                padding2,
                compressor,
                checksum2,
                padding3,
                v_link,
                checksum3,
                padding4,
                favorites,
                checksum4,
                padding5,
                switch_assign,
                checksum5,
                hardware_version
            })
        })
    }

    fn to_bytes(&self) -> Result<Box<[u8; 160]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_bits(&self.padding1);
            bs.set_bytes(self.common.to_bytes()?);
            self.checksum1.write(bs);
            bs.set_bits(&self.padding2);
            bs.set_bytes(self.compressor.to_bytes()?);
            self.checksum2.write(bs);
            bs.set_bits(&self.padding3);
            bs.set_bytes(self.v_link.to_bytes()?);
            self.checksum3.write(bs);
            bs.set_bits(&self.padding4);
            bs.set_bytes(self.favorites.to_bytes()?);
            self.checksum4.write(bs);
            bs.set_bits(&self.padding5);
            bs.set_bytes(self.switch_assign.to_bytes()?);
            self.checksum5.write(bs);
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

/*
System section checksums are a bit screwy.

For each System section (eg. Common, VLink, etc.) there is a 2 byte checksum. On the keyboard, you can only "Write" (save) one System section at a time.
The checksum for the section is set when that section is "Written" (saved), BUT its value depends on the bytes of all System bytes before it, *including other sections*.
This means that a checksum can be (and more often than not is) out of date. This will happen whenever a System section is "Written" (saved) without re-writing all sections following it.
For this reason, the checksum must actually be stored (at least partially).

What I know about the check sum mathematically:
- the sum of the padding bytes, section bytes and both checksum bytes is zero
- how the 2 checksum bytes is chosen exactly is a mystery (because there are many combinations of bytes which would cause the sum to be zero)
- the way the 2 checksums bytes are chosen depends on System section preceeding the current one (except presumably for the first section)
- it appears (likely, not confirmed) that changing the preceeding System sections affects the choice of the 2nd byte of checksum
*/
#[derive(Serialize, Deserialize, JsonSchema)]
struct SystemCheckSum(u8); // store the 2nd byte only, as the 1st can be calculated as the checksum

impl SystemCheckSum {
    fn read(data: &mut BitStream) -> Result<Self, BytesError> {
        let mut sum = data.sum_previous_bytes();
        let first = data.get_full_u8();
        let second = data.get_full_u8();
        sum = sum.wrapping_add(second as u16);
        let expected_first = sum_to_zero(sum);
        if first == expected_first {
            Ok(Self(second))
        } else {
            Err(BytesError::IncorrectCheckSum {
                expected: vec![expected_first],
                found: vec![first]
            })
        }
    }
    
    fn write(&self, data: &mut BitStream) {
        let sum = data.sum_previous_bytes().wrapping_add(self.0 as u16);
        let first = sum_to_zero(sum);
        data.set_full_u8(first);
        data.set_full_u8(self.0);
    }
}