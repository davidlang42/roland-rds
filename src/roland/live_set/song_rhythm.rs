use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use crate::roland::max;

#[derive(Serialize, Deserialize, Debug)]
pub struct SongRhythm {
    reserved1: Bits<1>,
    audio_level: u8, // max 127
    song_level: u8, // max 127
    song_output_port: u8, // max 5 (ALL, INT, OUT1, OUT2, OUT3, USB)
    reserved2: Bits<1>,
    rhythm_set: u8, // max 13
    rhythm_level: u8, // max 127
    rhythm_pattern: u8, // max 200
    rhythm_midi_out_channel: u8, // max 16 (OFF, 1-16)
    rhythm_output_port: u8, // max 5 (ALL, INT, OUT1, OUT2, OUT3, USB)
    unused: Bits<2>
}

impl Bytes<6> for SongRhythm {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bits| {
            bits.set_bits(&self.reserved1);
            bits.set_u8::<7>(self.audio_level);
            bits.set_u8::<7>(self.song_level);
            bits.set_u8::<3>(max(self.song_output_port, 5));
            bits.set_bits(&self.reserved2);
            bits.set_u8::<4>(max(self.rhythm_set, 13));
            bits.set_u8::<7>(self.rhythm_level);
            bits.set_u8::<8>(max(self.rhythm_pattern, 200));
            bits.set_u8::<5>(max(self.rhythm_midi_out_channel, 16));
            bits.set_u8::<3>(max(self.rhythm_output_port, 5));
            bits.set_bits(&self.unused);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            Ok(Self {
                reserved1: data.get_bits(),
                audio_level: data.get_u8::<7>(),
                song_level: data.get_u8::<7>(),
                song_output_port: data.get_u8::<3>(),
                reserved2: data.get_bits(),
                rhythm_set: data.get_u8::<4>(),
                rhythm_level: data.get_u8::<7>(),
                rhythm_pattern: data.get_u8::<8>(),
                rhythm_midi_out_channel: data.get_u8::<5>(),
                rhythm_output_port: data.get_u8::<3>(),
                unused: data.get_bits()
            })
        })
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json())
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}