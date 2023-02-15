use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{Json, StructuredJson, StructuredJsonError};
use crate::roland::types::enums::{OutputPort, OptionalMidiChannel};

#[derive(Serialize, Deserialize, Debug)]
pub struct SongRhythm {
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused1: Bits<1>,
    audio_level: u8, // max 127
    song_level: u8, // max 127
    song_output_port: OutputPort,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused2: Bits<1>,
    rhythm_set: u8, // max 13
    rhythm_level: u8, // max 127
    rhythm_pattern: u8, // max 200
    rhythm_midi_out_channel: OptionalMidiChannel,
    rhythm_output_port: OutputPort,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::zero")]
    unused3: Bits<2>
}

impl Bytes<6> for SongRhythm {
    fn to_bytes(&self) -> Result<Box<[u8; 6]>, BytesError> {
        BitStream::write_fixed(|bits| {
            bits.set_bits(&self.unused1);
            bits.set_u8::<7>(self.audio_level, 0, 127)?;
            bits.set_u8::<7>(self.song_level, 0, 127)?;
            bits.set_u8::<3>(self.song_output_port.into(), 0, 5)?;
            bits.set_bits(&self.unused2);
            bits.set_u8::<4>(self.rhythm_set, 0, 13)?;
            bits.set_u8::<7>(self.rhythm_level, 0, 127)?;
            bits.set_u8::<8>(self.rhythm_pattern, 0, 200)?;
            bits.set_u8::<5>(self.rhythm_midi_out_channel.into(), 0, 16)?;
            bits.set_u8::<3>(self.rhythm_output_port.into(), 0, 5)?;
            bits.set_bits(&self.unused3);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            Ok(Self {
                unused1: data.get_bits(),
                audio_level: data.get_u8::<7>(0, 127)?,
                song_level: data.get_u8::<7>(0, 127)?,
                song_output_port: data.get_u8::<3>(0, 5)?.into(),
                unused2: data.get_bits(),
                rhythm_set: data.get_u8::<4>(0, 13)?,
                rhythm_level: data.get_u8::<7>(0, 127)?,
                rhythm_pattern: data.get_u8::<8>(0, 200)?,
                rhythm_midi_out_channel: data.get_u8::<5>(0, 16)?.into(),
                rhythm_output_port: data.get_u8::<3>(0, 5)?.into(),
                unused3: data.get_bits()
            })
        })
    }
}

impl Json for SongRhythm {
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