use std::fmt::Debug;
use schemars::JsonSchema;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{StructuredJson, Json, StructuredJsonError};
use crate::roland::types::enums::{Polarity, SettingMode, OptionalMidiChannel, PartMode, ButtonFunction, PedalFunction, Temperament};
use crate::roland::types::notes::KeyNote;
use crate::roland::types::numeric::Offset1Dp;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[schemars(rename = "SystemCommon")]
pub struct Common {
    master_tune_percent: Offset1Dp<1024>, // 24-2024 (-100.0 - +100.0)
    master_level: u8, // max 127
    live_set_control_channel: OptionalMidiChannel,
    damper_polarity: Polarity,
    fc1_polarity: Polarity,
    fc2_polarity: Polarity,
    eq_mode: SettingMode,
    pedal_mode: SettingMode,
    s1_s2_mode: SettingMode,
    fc1_assign: PedalFunction, // 0-146
    fc2_assign: PedalFunction, // 0-146
    s1_assign: ButtonFunction, // 0-20
    s2_assign: ButtonFunction, // 0-20
    tone_remain: bool,
    receive_gm_gm2_system_on: bool,
    receive_gs_reset: bool,
    part_mode: PartMode,
    unsure: Bits<2>,
    temperament: Temperament,
    temperament_key: KeyNote,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<7>::zero")]
    unused: Bits<7>
}

impl Bytes<10> for Common {
    fn to_bytes(&self) -> Result<Box<[u8; Self::BYTE_SIZE]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_u16::<16>(self.master_tune_percent.into(), 24, 2024)?;
            bs.set_u8::<7>(self.master_level, 0, 127)?;
            bs.set_u8::<5>(self.live_set_control_channel.into(), 0, 16)?;
            bs.set_bool(self.damper_polarity.into());
            bs.set_bool(self.fc1_polarity.into());
            bs.set_bool(self.fc2_polarity.into());
            bs.set_bool(self.eq_mode.into());
            bs.set_bool(self.pedal_mode.into());
            bs.set_bool(self.s1_s2_mode.into());
            bs.set_u8::<8>(self.fc1_assign.into(), 0, 146)?;
            bs.set_u8::<8>(self.fc2_assign.into(), 0, 146)?;
            bs.set_u8::<5>(self.s1_assign.into(), 0, 20)?;
            bs.set_u8::<5>(self.s2_assign.into(), 0, 20)?;
            bs.set_bool(self.tone_remain);
            bs.set_bool(self.receive_gm_gm2_system_on);
            bs.set_bool(self.receive_gs_reset);
            bs.set_bool(self.part_mode.into());
            bs.set_bits(&self.unsure);
            bs.set_u8::<3>(self.temperament.into(), 0, 7)?;
            bs.set_u8::<4>(self.temperament_key.into(), 0, 11)?;
            bs.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            Ok(Self {
                master_tune_percent: bs.get_u16::<16>(24, 2024)?.into(),
                master_level: bs.get_u8::<7>(0, 127)?,
                live_set_control_channel: bs.get_u8::<5>(0, 16)?.into(),
                damper_polarity: bs.get_bool().into(),
                fc1_polarity: bs.get_bool().into(),
                fc2_polarity: bs.get_bool().into(),
                eq_mode: bs.get_bool().into(),
                pedal_mode: bs.get_bool().into(),
                s1_s2_mode: bs.get_bool().into(),
                fc1_assign: bs.get_u8::<8>(0, 146)?.into(),
                fc2_assign: bs.get_u8::<8>(0, 146)?.into(),
                s1_assign: bs.get_u8::<5>(0, 20)?.into(),
                s2_assign: bs.get_u8::<5>(0, 20)?.into(),
                tone_remain: bs.get_bool(),
                receive_gm_gm2_system_on: bs.get_bool(),
                receive_gs_reset: bs.get_bool(),
                part_mode: bs.get_bool().into(),
                unsure: bs.get_bits(),
                temperament: bs.get_u8::<3>(0, 7)?.into(),
                temperament_key: bs.get_u8::<4>(0, 11)?.into(),
                unused: bs.get_bits()
            })
        })
    }
}

impl Json for Common {
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