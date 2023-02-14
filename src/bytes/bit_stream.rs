use super::{Bit, Bits, BytesError};

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
        let mut bits = Vec::with_capacity(N);
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

    pub fn write_fixed<const N: usize, F, E>(f: F) -> Result<Box<[u8; N]>, E> where F: FnOnce(&mut Self) -> Result<(), E> {
        let mut stream = Self::new();
        f(&mut stream)?;
        if stream.len() != N * 8 {
            panic!("Failed to write all {} bytes (found {} bits, expected {})", N, stream.len(), N * 8);
        }
        stream.reset();
        Ok(stream.get_bytes())
    }

    pub fn write_fixed_bits<const B: usize, F, E>(f: F) -> Result<Bits<B>, E> where F: FnOnce(&mut Self) -> Result<(), E> {
        let mut stream = Self::new();
        f(&mut stream)?;
        if stream.len() != B {
            panic!("Failed to write all {} bits (found {} bits, expected {})", B, stream.len(), B);
        }
        stream.reset();
        Ok(stream.get_bits())
    }

    pub fn read_fixed<const N: usize, F, T, E>(bytes: Box<[u8; N]>, f: F) -> Result<T, E> where F: FnOnce(&mut Self) -> Result<T, E> {
        let mut stream = BitStream::read(bytes);
        let result = f(&mut stream)?;
        if stream.offset() < stream.len() {
            panic!("Failed to read all {} bytes (read {} bits, expected {})", N, stream.offset(), N * 8);
        }
        Ok(result)
    }

    pub fn read_fixed_bits<const B: usize, F, T, E>(bits: Bits<B>, f: F) -> Result<T, E> where F: FnOnce(&mut Self) -> Result<T, E> {
        let mut stream = BitStream::read_bits(bits);
        let result = f(&mut stream)?;
        if stream.offset() < stream.len() {
            panic!("Failed to read all {} bits (read {} bits, expected {})", B, stream.offset(), B);
        }
        Ok(result)
    }

    pub fn offset(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn get_bits<const N: usize>(&mut self) -> Bits<N> {
        let mut bits = [Bit::ZERO; N];
        for i in 0..N {
            bits[i] = self.get_bit();
        }
        Bits(bits)
    }

    fn get_bit(&mut self) -> Bit {
        self.next().expect("Tried to read past end of stream")
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

    pub fn get_bytes<const N: usize>(&mut self) -> Box<[u8; N]> {
        let mut bytes = Vec::new();
        for _ in 0..N {
            bytes.push(self.get_full_u8());
        }
        bytes.try_into().unwrap()
    }

    pub fn set_bytes<const N: usize>(&mut self, bytes: Box<[u8; N]>) {
        for byte in *bytes {
            self.set_full_u8(byte);
        }
    }

    pub fn get_bool(&mut self) -> bool {
        self.get_bit().on()
    }

    pub fn set_bool(&mut self, value: bool) {
        self.set_bit(if value { Bit::ONE } else { Bit::ZERO });
    }

    pub fn get_u8<const N: usize>(&mut self, min: u8, max: u8) -> Result<u8, BytesError> {
        if N > 8 {
            panic!("Cannot get u8 from {} bits", N);
        }
        let bits = self.get_bits::<N>();
        in_range(bits.to_u8(), min, max)
    }

    pub fn get_full_u8(&mut self) -> u8 {
        self.get_u8::<8>(u8::MIN, u8::MAX).unwrap()
    }

    pub fn set_u8<const N: usize>(&mut self, value: u8, min: u8, max: u8) -> Result<(), BytesError> {
        if N > 8 {
            panic!("Cannot set u8 into {} bits", N);
        }
        let bits = Bits::<N>::from_u8(in_range(value, min, max)?);
        Ok(self.set_bits(&bits))
    }

    pub fn set_full_u8(&mut self, value: u8) {
        self.set_u8::<8>(value, u8::MIN, u8::MAX).unwrap()
    }

    pub fn get_u16<const N: usize>(&mut self, min: u16, max: u16) -> Result<u16, BytesError> {
        if N > 16 {
            panic!("Cannot get u16 from {} bits", N);
        }
        let bits = self.get_bits::<N>();
        in_range_u16(bits.to_u16(), min, max)
    }

    pub fn get_full_u16(&mut self) -> u16 {
        self.get_u16::<16>(u16::MIN, u16::MAX).unwrap()
    }

    pub fn set_u16<const N: usize>(&mut self, value: u16, min: u16, max: u16) -> Result<(), BytesError> {
        if N > 16 {
            panic!("Cannot set u16 into {} bits", N);
        }
        let bits = Bits::<N>::from_u16(in_range_u16(value, min, max)?);
        Ok(self.set_bits(&bits))
    }

    pub fn set_full_u16(&mut self, value: u16) {
        self.set_u16::<16>(value, u16::MIN, u16::MAX).unwrap()
    }

    pub fn get_char<const N: usize>(&mut self) -> Result<char, BytesError> {
        let ascii = self.get_u8::<N>(0, 255)?;
        Ok(valid_char(ascii)? as char)
    }

    pub fn set_char<const N: usize>(&mut self, value: char) -> Result<(), BytesError> {
        let bits = Bits::<N>::from_u8(valid_char(value as u8)?);
        Ok(self.set_bits(&bits))
    }

    pub fn sum_previous_bytes(&self) -> u16 {
        if self.index % 8 != 0 {
            panic!("Cannot sum previous bytes if not a multiple of 8 bits");
        }
        let mut sum: u16 = 0;
        let mut i = 0;
        while i < self.index {
            let mut bits = [Bit::ZERO; 8];
            for j in 0..8 {
                bits[j] = self.bits[i];
                i += 1;
            }
            let byte = Bits(bits).to_u8();
            sum = sum.wrapping_add(byte as u16);
        }
        sum
    }
}

fn valid_char(ascii: u8) -> Result<u8, BytesError> {
    // Roland keyboards use chars 32 ' ' through 126 '~' inclusive
    if ascii < 32 || ascii > 126 {
        Err(BytesError::InvalidCharacter(ascii as char))
    } else {
        Ok(ascii)
    }
}

fn in_range(value: u8, min: u8, max: u8) -> Result<u8, BytesError> {
    if min > max {
        panic!("Invalid range (min: {}, max: {})", min, max);
    }
    if value < min || value > max {
        Err(BytesError::ValueOutOfRangeU8 { value, min, max })
    } else {
        Ok(value)
    }
}

fn in_range_u16(value: u16, min: u16, max: u16) -> Result<u16, BytesError> {
    if min > max {
        panic!("Invalid range (min: {}, max: {})", min, max);
    }
    if value < min || value > max {
        Err(BytesError::ValueOutOfRangeU16 { value, min, max })
    } else {
        Ok(value)
    }
}
