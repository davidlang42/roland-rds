use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};

#[derive(Serialize, Deserialize, Debug)]
pub struct SongRhythm(Bits<48>);
//TODO fields are well defined by the 700NX midi implementation, but CBF doing the boilerplate rn (should be 44 or 45 bits + 4 or 3 unused)

impl Bytes<6> for SongRhythm {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        let mut bits = BitStream::new();
        bits.set_bits(&self.0);
        bits.reset();
        Box::new(bits.get_bytes())
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        let mut data = BitStream::read(bytes);
        Ok(Self(data.get_bits()))
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