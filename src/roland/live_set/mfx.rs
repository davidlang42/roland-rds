use std::fmt::Debug;

use schemars::JsonSchema;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError};
use crate::roland::types::effects::mfx::MfxType;
use crate::roland::types::numeric::Parameter;

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct Mfx {
    enable: bool,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<8>::zero")]
    unused1: Bits<8>, //TODO I think this is actually the MFX control choice (which depends on type, so should probably be part of mfx_type)
    #[validate]
    mfx_type: MfxType,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<8>::unit")]
    padding1: Bits<8>,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<14>::unit")]
    padding2: Bits<14>,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<14>::unit")]
    padding3: Bits<14>,
    #[serde(skip_serializing_if="Bits::is_unit", default="Bits::<14>::unit")]
    padding4: Bits<14>,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<26>::zero")]
    unused2: Bits<26>,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<3>::zero")]
    unused3: Bits<3>
}

impl Bytes<76> for Mfx {
    fn to_bytes(&self) -> Result<Box<[u8; Self::BYTE_SIZE]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_bool(self.enable);
            bs.set_bits(&self.unused1);
            bs.set_u8::<8>(self.mfx_type.number(), 0, 255)?;
            bs.set_bits(&self.padding1);
            bs.set_bits(&self.padding2);
            bs.set_bits(&self.padding3);
            bs.set_bits(&self.padding4);
            bs.set_bits(&self.unused2);
            for p in self.mfx_type.parameters() {
                bs.set_u16::<16>(p.into(), 12768, 52768)?;
            }
            bs.set_bits(&self.unused3);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            let enable = bs.get_bool();
            let unused1 = bs.get_bits();
            let type_number = bs.get_u8::<8>(0, 255)?.into();
            let padding1 = bs.get_bits();
            let padding2 = bs.get_bits();
            let padding3 = bs.get_bits();
            let padding4 = bs.get_bits();
            let unused2 = bs.get_bits();
            let mut parameters = [Parameter::default(); 32];
            for i in 0..parameters.len() {
                parameters[i] = bs.get_u16::<16>(12768, 52768)?.into();
            }
            Ok(Self {
                enable,
                unused1,
                mfx_type: MfxType::from(type_number, parameters),
                padding1,
                padding2,
                padding3,
                padding4,
                unused2,
                unused3: bs.get_bits()
            })
        })
    }
}

impl Json for Mfx {
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

impl Mfx {
    fn active(&self) -> bool {
        self.enable && !self.mfx_type.is_off()
    }
}

impl Mfx {
    pub fn tone_remain_warning(a: &Self, b: &Self, a_layer0_active: &bool) -> Option<String> {
        if !a_layer0_active {
            None // if layer0 wasn't on to begin with then mfx can't affect any tone which needs remaining
        } else if a.active() && !b.active() {
            Some(format!("Mfx ({}) turns OFF", a.mfx_type.name()))
        } else if !a.active() && b.active() {
            Some(format!("Mfx ({}) turns ON", b.mfx_type.name()))
        } else if !a.active() && !b.active() {
            None // other changes to Mfx are irrelevant if Mfx is off 
        } else if a.mfx_type.number() != b.mfx_type.number() {
            Some(format!("Mfx ({}) changes to {}", a.mfx_type.name(), b.mfx_type.name()))
        } else if a.mfx_type.parameters() != b.mfx_type.parameters() {
            Some(format!("Mfx ({}) parameters change", a.mfx_type.name()))
        } else {
            None
        }
    }
}