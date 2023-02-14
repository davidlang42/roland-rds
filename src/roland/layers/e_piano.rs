use std::fmt::Debug;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};

#[derive(Serialize, Deserialize, Debug)]
pub struct EPianoLayer(Bits<48>);

impl Bytes<6> for EPianoLayer {
    fn to_bytes(&self) -> Result<Box<[u8; Self::BYTE_SIZE]>, BytesError> {
        BitStream::write_fixed(|bs| {
            Ok(bs.set_bits(&self.0))
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |bs| {
            Ok(Self(bs.get_bits()))
        })
    }
}
