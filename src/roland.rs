use crate::bits::{Bit, BitStream};

pub struct RD300NX {

}

#[derive(Debug)]
pub struct ParseError {
    _file_byte: usize,
    _message: String
}

impl From<Vec<u8>> for RD300NX {
    fn from(bytes: Vec<u8>) -> Self {
        let stream: BitStream = bytes.into_iter().into();
        Self::parse(stream).unwrap()
    }
}

impl RD300NX {
    fn parse(mut data: BitStream) -> Result<Self, ParseError> {
        println!("Total bits: {}", data.len());
        let f1 = "Harp in G";
        let f2 = "Concert Grand";
        let f3 = "Xylophone";
        if let Some(s1) = data.search(f1) {
            println!("Found '{}' at offset {}", f1, s1);//0
        }
        if let Some(s2) = data.search(f2) {
            println!("Found '{}' at offset {}", f2, s2);//17280 = 2160 bytes
        }
        if let Some(s3) = data.search(f3) {
            println!("Found '{}' at offset {}", f3, s3);//34560
        }
        todo!();
    }
}
