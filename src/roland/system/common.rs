use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{StructuredJson, Json, StructuredJsonError};
use crate::roland::types::enums::{Polarity, SettingMode};

#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    master_tune: u16, // 24-2024 (-100.0 - +100.0)
    master_level: u8, // max 127
    live_set_control_channel: u8, // max 16 (OFF, 1-16)
    damper_polarity: Polarity,
    fc1_polarity: Polarity,
    fc2_polarity: Polarity,
    eq_mode: SettingMode,
    pedal_mode: SettingMode,
    s1_s2_mode: SettingMode,
    fc1_assign: u8, // max 146 (OFF, CC00 - CC127, BEND-UP, BEND-DOWN, AFTERTOUCH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, RHY PLY/STP, SONG PLY/STP, SONG RESET, MFX1 SW, MFX2 SW, MFX1 CONTROL, MFX2 CONTROL, ROTARY SPEED, SOUND FOCUS VALUE, LIVE SET UP, LIVE SET DOWN)
    fc2_assign: u8, // max 146 (OFF, CC00 - CC127, BEND-UP, BEND-DOWN, AFTERTOUCH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, RHY PLY/STP, SONG PLY/STP, SONG RESET, MFX1 SW, MFX2 SW, MFX1 CONTROL, MFX2 CONTROL, ROTARY SPEED, SOUND FOCUS VALUE, LIVE SET UP, LIVE SET DOWN)
    s1_assign: u8, // max 20 (OFF, COUPLE+1OCT, COUPLE-1OCT, COUPLE+2OCT, COUPLE-2OCT, COUPLE+5TH, COUPLE-4TH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, SONG PLY/STP, SONG RESET, SONG BWD, SONG FWD, MFX1 SW, MFX2 SW, ROTARY SPEED, LIVE SET UP, LIVE SET DOWN, PANEL LOCK)
    s2_assign: u8, // max 20 (OFF, COUPLE+1OCT, COUPLE-1OCT, COUPLE+2OCT, COUPLE-2OCT, COUPLE+5TH, COUPLE-4TH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, SONG PLY/STP, SONG RESET, SONG BWD, SONG FWD, MFX1 SW, MFX2 SW, ROTARY SPEED, LIVE SET UP, LIVE SET DOWN, PANEL LOCK)
    tone_remain: bool,
    unsure: Bits<4>, //TODO figure this out & find the setting about 16PARTS/16PARTS+PERF
    unused: Bits<15>
}

impl Bytes<10> for Common {
    fn to_bytes(&self) -> Result<Box<[u8; Self::BYTE_SIZE]>, BytesError> {
        BitStream::write_fixed(|bs| {
            bs.set_u16::<16>(self.master_tune, 24, 2024)?;
            bs.set_u8::<7>(self.master_level, 0, 127)?;
            bs.set_u8::<5>(self.live_set_control_channel, 0, 16)?;
            bs.set_bool(self.damper_polarity.into());
            bs.set_bool(self.fc1_polarity.into());
            bs.set_bool(self.fc2_polarity.into());
            bs.set_bool(self.eq_mode.into());
            bs.set_bool(self.pedal_mode.into());
            bs.set_bool(self.s1_s2_mode.into());
            bs.set_u8::<8>(self.fc1_assign, 0, 146)?;
            bs.set_u8::<8>(self.fc2_assign, 0, 146)?;
            bs.set_bool(self.tone_remain);
            bs.set_bits(&self.unsure);
            bs.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            Ok(Self {
                master_tune: bs.get_u16::<16>(24, 2024)?,
                master_level: bs.get_u8::<7>(0, 127)?,
                live_set_control_channel: bs.get_u8::<5>(0, 16)?,
                damper_polarity: bs.get_bool().into(),
                fc1_polarity: bs.get_bool().into(),
                fc2_polarity: bs.get_bool().into(),
                eq_mode: bs.get_bool().into(),
                pedal_mode: bs.get_bool().into(),
                s1_s2_mode: bs.get_bool().into(),
                fc1_assign: bs.get_u8::<8>(0, 146)?,
                fc2_assign: bs.get_u8::<8>(0, 146)?,
                s1_assign: bs.get_u8::<5>(0, 20)?,
                s2_assign: bs.get_u8::<5>(0, 20)?,
                tone_remain: bs.get_bool(),
                unsure: bs.get_bits(),
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