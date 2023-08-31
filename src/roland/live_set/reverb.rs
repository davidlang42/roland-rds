use std::fmt::Debug;

use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError};
use crate::roland::types::effects::reverb::ReverbType;
use crate::roland::types::numeric::Parameter;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Reverb {
    #[validate]
    pub reverb_type: ReverbType,
    #[validate(range(max = 127))]
    depth: u8,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<2>::zero")]
    unused1: Bits<2>,
    // [Parameter; 20]
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<3>::zero")]
    unused2: Bits<3>
}

impl Bytes<42> for Reverb {
    fn to_bytes(&self) -> Result<Box<[u8; 42]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_u8::<4>(self.reverb_type.number(), 0, 6)?;
            bs.set_u8::<7>(self.depth, 0, 127)?;
            bs.set_bits(&self.unused1);
            for p in self.reverb_type.parameters().into_iter() {
                bs.set_u16::<16>(p.into(), 12768, 52768)?;
            }
            bs.set_bits(&self.unused2);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let type_number = bs.get_u8::<4>(0, 6)?;
            let depth = bs.get_u8::<7>(0, 127)?;
            let unused1 = bs.get_bits();
            let mut parameters = [Parameter::default(); 20];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>(12768, 52768)?.into();
            }
            Ok(Self {
                reverb_type: ReverbType::from(type_number, parameters),
                depth,
                unused1,
                unused2: bs.get_bits()
            })
        })
    }
}

impl Json for Reverb {
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

impl Reverb {
    pub fn tone_remain_warning(a: &Self, b: &Self, a_max_reverb_level: u8, b_max_reverb_level: u8) -> Option<String> {
        let a_off = a_max_reverb_level == 0 || a.reverb_type.is_off();
        let b_off = b_max_reverb_level == 0 || b.reverb_type.is_off();
        if a_off && !b_off {
            Some(format!("Reverb ({}) turns ON", b.reverb_type.name()))
        } else if !a_off && b_off {
            Some(format!("Reverb ({}) turns OFF", a.reverb_type.name()))
        } else if a_off && b_off {
            None // other changes to Reverb are irrelevant if Reverb is off 
        } else if a.reverb_type.number() != b.reverb_type.number() {
            Some(format!("Reverb ({}) changes to {}", a.reverb_type.name(), b.reverb_type.name()))
        } else if a.depth != b.depth {
            Some(format!("Reverb ({}) depth changes from {} to {}", a.reverb_type.name(), a.depth, b.depth))
        } else if a.reverb_type.parameters() != b.reverb_type.parameters() {
            Some(format!("Reverb ({}) parameters change", a.reverb_type.name()))
        } else {
            None
        }
    }
}