use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::{StructuredJson, Json};

#[derive(Serialize, Deserialize, Debug)]
pub struct Common(Bits<80>);
//TODO need to do these fields because some settings are relevant to live sets (should be 61 bits + 3 unused + 2 more bytes of unknown data?)
//TODO find where the setting about 16PARTS/16PARTS+LAYERS is, because thats important too

impl Bytes<10> for Common {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bs| {
            bs.set_bits(&self.0);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            Ok(Self(bs.get_bits()))
        })
    }
}

impl Json for Common {
    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json())
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}