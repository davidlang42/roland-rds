use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson};
use crate::roland::in_range_u16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mfx {
    enable: bool,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused1: Bits<8>,
    effect_type: u8,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::unit")]
    padding1: Bits<8>,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::unit")]
    padding2: Bits<14>,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::unit")]
    padding3: Bits<14>,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::unit")]
    padding4: Bits<14>,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused2: Bits<26>,
    parameters: [u16; 32], // 12768-52768 (-20000 - +20000)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused3: Bits<3>
}

impl Bytes<76> for Mfx {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bs| {
            bs.set_bool(self.enable);
            bs.set_bits(&self.unused1);
            bs.set_u8::<8>(self.effect_type);
            bs.set_bits(&self.padding1);
            bs.set_bits(&self.padding2);
            bs.set_bits(&self.padding3);
            bs.set_bits(&self.padding4);
            bs.set_bits(&self.unused2);
            for i in 0..self.parameters.len() {
                bs.set_u16::<16>(in_range_u16(self.parameters[i], 12768, 52768));
            }
            bs.set_bits(&self.unused3);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let enable = bs.get_bool();
            let unused1 = bs.get_bits();
            let effect_type = bs.get_u8::<8>();
            let padding1 = bs.get_bits();
            let padding2 = bs.get_bits();
            let padding3 = bs.get_bits();
            let padding4 = bs.get_bits();
            let unused2 = bs.get_bits();
            let mut parameters = [0; 32];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>();
            }
            Ok(Self {
                enable,
                unused1,
                effect_type,
                padding1,
                padding2,
                padding3,
                padding4,
                unused2,
                parameters,
                unused3: bs.get_bits()
            })
        })
    }
}

impl Json for Mfx {
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