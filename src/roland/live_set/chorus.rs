use std::fmt::Debug;
use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError};
use crate::roland::types::enums::OutputSelect;
use crate::roland::types::effects::chorus::ChorusType;
use crate::roland::types::numeric::Parameter;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Chorus {
    #[validate]
    pub chorus_type: ChorusType,
    #[validate(range(max = 127))]
    depth: u8,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<2>::zero")]
    unused1: Bits<2>,
    output_select: OutputSelect,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<1>::zero")]
    unused: Bits<1>
}

impl Bytes<42> for Chorus {
    fn to_bytes(&self) -> Result<Box<[u8; 42]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_u8::<4>(self.chorus_type.number(), 0, 3)?;
            bs.set_u8::<7>(self.depth, 0, 127)?;
            bs.set_bits(&self.unused1);
            bs.set_u8::<2>(self.output_select.into(), 0, 2)?;
            for p in self.chorus_type.parameters().into_iter() {
                bs.set_u16::<16>(p.into(), 12768, 52768)?;
            }
            bs.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let type_number = bs.get_u8::<4>(0, 3)?;
            let depth = bs.get_u8::<7>(0, 127)?;
            let unused1 = bs.get_bits();
            let output_select = bs.get_u8::<2>(0, 2)?.into();
            let mut parameters = [Parameter::default(); 20];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>(12768, 52768)?.into();
            }
            Ok(Self {
                chorus_type: ChorusType::from(type_number, parameters),
                depth,
                unused1,
                output_select,
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

impl Chorus {
    pub fn tone_remain_warning(a: &Self, b: &Self, a_max_chorus_level: u8, b_max_chorus_level: u8) -> Option<String> {
        let a_off = a_max_chorus_level == 0 || a.chorus_type.is_off();
        let b_off = b_max_chorus_level == 0 || b.chorus_type.is_off();
        if a_off && !b_off {
            Some(format!("Chorus ({}) turns ON", b.chorus_type.name()))
        } else if !a_off && b_off {
            Some(format!("Chorus ({}) turns OFF", a.chorus_type.name()))
        } else if a_off && b_off {
            None // other changes to Chorus are irrelevant if Chorus is off 
        } else if a.chorus_type.number() != b.chorus_type.number() {
            Some(format!("Chorus ({}) changes to {:?}", a.chorus_type.name(), b.chorus_type.name()))
        } else if a.depth != b.depth {
            Some(format!("Chorus ({}) depth changes from {} to {}", a.chorus_type.name(), a.depth, b.depth))
        } else if a.output_select != b.output_select {
            Some(format!("Chorus ({}) output changes from {:?} to {:?}", a.chorus_type.name(), a.output_select, b.output_select))
        } else if a.chorus_type.parameters() != b.chorus_type.parameters() {
            Some(format!("Chorus ({}) parameters change", a.chorus_type.name()))
        } else {
            None
        }
    }
}
