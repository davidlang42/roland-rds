use std::{fmt::Display, str::FromStr};

#[derive(Debug, Copy, Clone)]
pub struct Bit(bool);

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Bit {
    pub const ONE: Bit = Bit(true);
    pub const ZERO: Bit = Bit(false);

    pub fn on(&self) -> bool {
        self.0
    }

    pub fn to_char(&self) -> char {
        if self.0 {
            '1'
        } else {
            '0'
        }
    }
}

#[derive(Debug)]
pub struct Bits<const N: usize>([Bit; N]);

impl<const N: usize> Display for Bits<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, bit) in self.0.iter().enumerate() {
            if i % 8 == 0 && i != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

pub enum BitsError {
    IncompleteByteBeforeEnd(String),
    InvalidDigit(char)
}

impl<const N: usize> FromStr for Bits<N> {
    type Err = BitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<&str> = s.split(" ").collect();
        let mut bits = Vec::new();
        for i in 0..bytes.len() {
            if i != bytes.len() - 1 && bytes[i].len() != 8 {
                return Err(BitsError::IncompleteByteBeforeEnd(bytes[i].to_string()));
            }
            for c in bytes[i].chars() {
                let bit = match c {
                    '0' => Bit::ZERO,
                    '1' => Bit::ONE,
                    _ => return Err(BitsError::InvalidDigit(c))
                };
                bits.push(bit);
            }
        }
        Ok(Bits(bits.try_into().unwrap()))
    }
}

impl From<u8> for Bits<8> {
    fn from(byte: u8) -> Self {
        Self::from_byte(byte)
    }
}

impl<const N: usize> Into<u8> for Bits<N> {
    fn into(self) -> u8 {
        Self::to_byte(self.0)
    }
}

impl<const N: usize> Into<u16> for Bits<N> {
    fn into(self) -> u16 { //TODO basically duplicated
        if N > 16 {
            panic!("Too big"); //TODO enforce compiler & better message
        }
        let mut num = 0;
        let mut bit_value = 2u16.pow((self.0.len() - 1) as u32);
        for bit in self.0 {
            if bit.on() {
                num += bit_value;
            }
            bit_value /= 2;
        }
        num
    }
}

impl<const N: usize> Bits<N> {
    fn to_byte(bits: [Bit; N]) -> u8 {
        if N > 8 {
            panic!("Too big"); //TODO enforce compiler & better message
        }
        let mut num = 0;
        let mut bit_value = 2u8.pow((bits.len() - 1) as u32);
        for bit in bits {
            if bit.on() {
                num += bit_value;
            }
            bit_value /= 2;
        }
        num
    }

    pub fn from_byte(mut byte: u8) -> Bits<N> {
        if N > 8 {
            panic!("Too big"); //TODO enforce compiler & better message
        }
        if N != 8 && byte >= 2u8.pow(N as u32) {
            panic!("Too small"); //TODO enforce compiler & better msg
        }
        let mut bits = [Bit::ZERO; N];
        let mut bit_value = 2u8.pow((bits.len() - 1) as u32);
        for bit_index in 0..bits.len() {
            if byte >= bit_value {
                byte -= bit_value;
                bits[bit_index] = Bit::ONE;
            }
            bit_value /= 2;
        }
        Bits(bits)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        if N % 8 != 0 {
            panic!("Incomplete bytes");//TODO better msg + compiler enforse
        }
        let mut start = 0;
        let mut bytes = Vec::new();
        while start < self.0.len() {
            let end = start + 8;
            bytes.push(Bits::<8>::to_byte(self.0[start..end].try_into().unwrap()));
            start = end;
        }
        bytes
    }

    pub fn compress(text: [char; 16]) -> Bits<112> {
        let mut bits = Vec::new();
        for ch in text {
            for bit in Bits::<7>::from_byte(ch as u8).0 {
                bits.push(bit);
            }
        }
        Bits(bits.try_into().unwrap())
    }
}

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
    pub fn read<I: IntoIterator<Item = u8>>(bytes: I) -> Self {
        let mut bits = Vec::new();
        for byte in bytes {
            for bit in Bits::from(byte).0 {
                bits.push(bit);
            }
        }
        Self {
            bits,
            index: 0
        }
    }

    pub fn eof(&self) -> bool {
        self.index >= self.bits.len()
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

    pub fn get_u8<const N: usize>(&mut self) -> u8 {
        if N > 8 {
            panic!("Cannot get u8 from {} bits", N);
        }
        let bits = self.get_bits::<N>();
        bits.into()
    }

    pub fn get_u16<const N: usize>(&mut self) -> u16 {
        if N > 16 {
            panic!("Cannot get u16 from {} bits", N);
        }
        let bits = self.get_bits::<N>();
        bits.into()
    }

    pub fn get_char(&mut self) -> char {
        let ascii = self.get_u8::<7>();
        ascii as char
    }
}
