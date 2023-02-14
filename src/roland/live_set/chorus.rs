use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError};
use crate::roland::types::Parameter;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chorus {
    chorus_type: u8, // max 3 (OFF, CHORUS, DELAY, GM2 CHORUS)
    depth: u8, // max 127
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused1: Bits<2>,
    output_select: u8, // max 2 (MAIN, REV, MAIN+REV)
    parameters: [Parameter; 20],
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
                bs.set_u16::<16>(self.parameters[i].into(), 12768, 52768)?;
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
            let mut parameters = [Parameter::default(); 20];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>(12768, 52768)?.into();
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