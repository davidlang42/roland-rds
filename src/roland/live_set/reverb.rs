use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson};
use crate::roland::in_range_u16;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reverb {
    reverb_type: u8, // max 6 (OFF, REVERB, ROOM, HALL, PLATE, GM2 REVERB, CATHEDRAL)
    depth: u8, // max 127
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused1: Bits<2>,
    parameters: [u16; 20], // each 12768-52768 (-20000 - +20000)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused2: Bits<3>
}

impl Bytes<42> for Reverb {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bs| {
            bs.set_u8::<4>(self.reverb_type);
            bs.set_u8::<7>(self.depth);
            bs.set_bits(&self.unused1);
            for i in 0..self.parameters.len() {
                bs.set_u16::<16>(in_range_u16(self.parameters[i], 12768, 52768));
            }
            bs.set_bits(&self.unused2);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let reverb_type = bs.get_u8::<4>();
            let depth = bs.get_u8::<7>();
            let unused1 = bs.get_bits();
            let mut parameters = [0; 20];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>();
            }
            Ok(Self {
                reverb_type,
                depth,
                unused1,
                parameters,
                unused2: bs.get_bits()
            })
        })
    }
}

impl Json for Reverb {
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