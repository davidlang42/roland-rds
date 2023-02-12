use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use crate::roland::in_range_u16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Mfx {
    enable: bool,
    effect_type: Bits<92>, //TODO decode effect type (needs new test RDS file)
    parameters: [u16; 32], // 12768-52768 (-20000 - +20000)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused: Bits<3>
}

impl Bytes<76> for Mfx {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bs| {
            bs.set_bool(self.enable);
            bs.set_bits(&self.effect_type);
            for i in 0..self.parameters.len() {
                bs.set_u16::<16>(in_range_u16(self.parameters[i], 12768, 52768));
            }
            bs.set_bits(&self.unused);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let enable = bs.get_bool();
            let effect_type = bs.get_bits();
            let mut parameters = [0; 32];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>();
            }
            Ok(Self {
                enable,
                effect_type,
                parameters,
                unused: bs.get_bits()
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