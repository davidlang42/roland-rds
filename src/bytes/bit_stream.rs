use super::{Bit, Bits};

pub struct BitStream {
    bits: Vec<Bit>,
    index: usize
}

impl Iterator for BitStream {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.bits.len() {
            let bit = self.bits[self.index];
            self.index += 1;
            Some(bit)
        } else {
            None
        }
    }
}

impl BitStream {
    fn read<const N: usize>(bytes: Box<[u8; N]>) -> Self {
        let mut bits = Vec::new();
        for byte in bytes.as_slice() {
            for bit in Bits::<8>::from_u8(*byte).0 {
                bits.push(bit);
            }
        }
        Self {
            bits,
            index: 0
        }
    }

    fn read_bits<const B: usize>(bits: Bits<B>) -> Self {
        Self {
            bits: bits.0.into_iter().collect(),
            index: 0
        }
    }

    fn new() -> Self {
        Self {
            bits: Vec::new(),
            index: 0
        }
    }

    pub fn write_fixed<const N: usize, F>(f: F) -> Box<[u8; N]> where F: FnOnce(&mut Self) {
        let mut stream = Self::new();
        f(&mut stream);
        if stream.len() != N * 8 {
            panic!("Failed to write all {} bytes (found {} bits, expected {})", N, stream.len(), N * 8);
        }
        stream.reset();
        Box::new(stream.get_bytes())
    }

    pub fn write_fixed_bits<const B: usize, F>(f: F) -> Bits<B> where F: FnOnce(&mut Self) {
        let mut stream = Self::new();
        f(&mut stream);
        if stream.len() != B {
            panic!("Failed to write all {} bits (found {} bits, expected {})", B, stream.len(), B);
        }
        stream.reset();
        stream.get_bits()
    }

    pub fn read_fixed<const N: usize, F, T>(bytes: Box<[u8; N]>, f: F) -> T where F: FnOnce(&mut Self) -> T {
        let mut stream = BitStream::read(bytes);
        let result = f(&mut stream);
        if stream.position() < stream.len() {
            panic!("Failed to read all {} bytes (read {} bits, expected {})", N, stream.position(), N * 8);
        }
        result
    }

    pub fn read_fixed_bits<const B: usize, F, T>(bits: Bits<B>, f: F) -> T where F: FnOnce(&mut Self) -> T {
        let mut stream = BitStream::read_bits(bits);
        let result = f(&mut stream);
        if stream.position() < stream.len() {
            panic!("Failed to read all {} bits (read {} bits, expected {})", B, stream.position(), B);
        }
        result
    }

    pub fn position(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn get_bits<const N: usize>(&mut self) -> Bits<N> {
        let mut bits = Vec::new();
        for _ in 0..N {
            if let Some(bit) = self.next() {
                bits.push(bit);
            } else {
                panic!("Tried to read past end of stream");
            }
        }
        Bits(bits.try_into().unwrap())
    }

    pub fn set_bits<const N: usize>(&mut self, value: &Bits<N>) {
        for bit in value.0 {
            self.set_bit(bit);
        }
    }

    fn set_bit(&mut self, value: Bit) {
        self.bits.insert(self.index, value);
        self.index += 1;
    }

    pub fn get_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut bytes = Vec::new();
        for _ in 0..N {
            bytes.push(self.get_u8::<8>());
        }
        bytes.try_into().unwrap()
    }

    pub fn get_bool(&mut self) -> bool {
        self.get_bits::<1>().0[0].on()
    }

    pub fn set_bool(&mut self, value: bool) {
        self.set_bit(if value { Bit::ONE } else { Bit::ZERO });
    }

    pub fn get_u8<const N: usize>(&mut self) -> u8 {
        if N > 8 {
            panic!("Cannot get u8 from {} bits", N);
        }
        let bits = self.get_bits::<N>();
        bits.to_u8()
    }

    pub fn set_u8<const N:usize>(&mut self, value: u8) {
        if N > 8 {
            panic!("Cannot set u8 into {} bits", N);
        }
        let bits = Bits::<N>::from_u8(value);
        self.set_bits(&bits);
    }

    pub fn get_u16<const N: usize>(&mut self) -> u16 {
        if N > 16 {
            panic!("Cannot get u16 from {} bits", N);
        }
        let bits = self.get_bits::<N>();
        bits.to_u16()
    }

    pub fn set_u16<const N:usize>(&mut self, value: u16) {
        if N > 16 {
            panic!("Cannot set u16 into {} bits", N);
        }
        let bits = Bits::<N>::from_u16(value);
        self.set_bits(&bits);
    }

    pub fn get_char(&mut self) -> char {
        let ascii = self.get_u8::<7>();
        ascii as char
    }

    pub fn set_char(&mut self, value: char) {
        let bits = Bits::<7>::from_u8(value as u8);
        self.set_bits(&bits);
    }
}
