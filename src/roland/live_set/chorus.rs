use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chorus {
    chorus_type: u8, // max 3 (OFF, CHORUS, DELAY, GM2 CHORUS)
    depth: u8, // max 127
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused1: Bits<2>,
    output_select: u8, // max 2 (MAIN, REV, MAIN+REV)
    parameters: [u16; 20], // each 12768-52768 (-20000 - +20000)
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused: Bits<1>
}

impl Bytes<42> for Chorus {
    fn to_bytes(&self) -> Result<Box<[u8; 42]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_u8::<4>(self.chorus_type, 0, 3)?;
            bs.set_u8::<7>(self.depth, 0, 127)?;
            bs.set_bits(&self.unused1);
            bs.set_u8::<2>(self.output_select, 0, 2)?;
            for i in 0..self.parameters.len() {
                bs.set_u16::<16>(self.parameters[i], 12768, 52768)?;
            }
            bs.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let chorus_type = bs.get_u8::<4>(0, 3)?;
            let depth = bs.get_u8::<7>(0, 127)?;
            let unused1 = bs.get_bits();
            let output_select = bs.get_u8::<2>(0, 2)?;
            let mut parameters = [0; 20];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>(12768, 52768)?;
            }
            Ok(Self {
                chorus_type,
                depth,
                unused1,
                output_select,
                parameters,
                unused: bs.get_bits()
            })
        })
    }
}

impl Json for Chorus {
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