use std::fmt::Debug;
use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError, serialize_default_terminated_array};
use crate::json::validation::valid_boxed_elements;
use crate::roland::types::enums::{OutputSelect, ChorusType};
use crate::roland::types::numeric::Parameter;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Chorus {
    pub chorus_type: ChorusType,
    #[validate(range(max = 127))]
    depth: u8,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<2>::zero")]
    unused1: Bits<2>,
    output_select: OutputSelect,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 20>")]
    #[validate(custom = "valid_boxed_elements")]
    parameters: Box<[Parameter; 20]>,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<1>::zero")]
    unused: Bits<1>
}

impl Bytes<42> for Chorus {
    fn to_bytes(&self) -> Result<Box<[u8; 42]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_u8::<4>(self.chorus_type.into(), 0, 3)?;
            bs.set_u8::<7>(self.depth, 0, 127)?;
            bs.set_bits(&self.unused1);
            bs.set_u8::<2>(self.output_select.into(), 0, 2)?;
            for i in 0..self.parameters.len() {
                bs.set_u16::<16>(self.parameters[i].into(), 12768, 52768)?;
            }
            bs.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let chorus_type = bs.get_u8::<4>(0, 3)?.into();
            let depth = bs.get_u8::<7>(0, 127)?;
            let unused1 = bs.get_bits();
            let output_select = bs.get_u8::<2>(0, 2)?.into();
            let mut parameters = [Parameter::default(); 20];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>(12768, 52768)?.into();
            }
            Ok(Self {
                chorus_type,
                depth,
                unused1,
                output_select,
                parameters: Box::new(parameters),
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
        let a_off = a_max_chorus_level == 0 || a.chorus_type == ChorusType::Off;
        let b_off = b_max_chorus_level == 0 || b.chorus_type == ChorusType::Off;
        if a_off && !b_off {
            Some(format!("Chorus ({:?}) turns ON", b.chorus_type))
        } else if !a_off && b_off {
            Some(format!("Chorus ({:?}) turns OFF", a.chorus_type))
        } else if a_off && b_off {
            None // other changes to Chorus are irrelevant if Chorus is off 
        } else if a.chorus_type != b.chorus_type {
            Some(format!("Chorus ({:?}) changes to {:?}", a.chorus_type, b.chorus_type))
        } else if a.depth != b.depth {
            Some(format!("Chorus ({:?}) depth changes from {} to {}", a.chorus_type, a.depth, b.depth))
        } else if a.output_select != b.output_select {
            Some(format!("Chorus ({:?}) output changes from {:?} to {:?}", a.chorus_type, a.output_select, b.output_select))
        } else if a.parameters != b.parameters {
            Some(format!("Chorus ({:?}) parameters change", a.chorus_type))
        } else {
            None
        }
    }
}