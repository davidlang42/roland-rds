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
        const BITS_PER_BYTE: usize = 8;
        const BYTES_PER_LINE: usize = 10;
        for (i, bit) in self.0.iter().enumerate() {
            if i % BITS_PER_BYTE == 0 && i != 0 {
                if i % (BITS_PER_BYTE * BYTES_PER_LINE) == 0 {
                    writeln!(f)?;
                } else {
                    write!(f, " ")?;
                }
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
                    ' ' => continue, // valid byte separator
                    '\r' => continue, // valid line separator
                    '\n' => continue, // valid line separator
                    _ => return Err(BitsError::InvalidDigit(c))
                };
                bits.push(bit);
            }
        }
        Ok(Bits(bits.try_into().unwrap()))
    }
}

impl<const N: usize> Bits<N> {
    fn to_u8(&self) -> u8 {
        if N > 8 {
            panic!("Bits size ({}) is too big for a u8 value", N);
        }
        let mut num = 0;
        let mut bit_value = 2u8.pow((self.0.len() - 1) as u32);
        for bit in self.0 {
            if bit.on() {
                num += bit_value;
            }
            bit_value /= 2;
        }
        num
    }

    fn to_u16(&self) -> u16 {
        if N > 16 {
            panic!("Bits size ({}) is too big for a u16 value", N);
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

    fn from_u8(mut byte: u8) -> Bits<N> {
        if N > 8 {
            panic!("Bits size ({}) is too big for a u8 value", N);
        }
        if N != 8 && byte >= 2u8.pow(N as u32) {
            panic!("Bits size ({}) is too small for u8 value {}", N, byte);
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
            panic!("Bits size ({}) must be a multiple of 8 to avoid incomplete bytes", N);
        }
        let mut start = 0;
        let mut bytes = Vec::new();
        while start < self.0.len() {
            let end = start + 8;
            bytes.push(Bits::<8>(self.0[start..end].try_into().unwrap()).to_u8());
            start = end;
        }
        bytes
    }
}

impl Bits<7> {
    pub fn compress(text: [char; 16]) -> Bits<112> {
        let mut bits = Vec::new();
        for ch in text {
            for bit in Self::from_u8(ch as u8).0 {
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
            for bit in Bits::<8>::from_u8(byte).0 {
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
        bits.to_u8()
    }

    pub fn get_u16<const N: usize>(&mut self) -> u16 {
        if N > 16 {
            panic!("Cannot get u16 from {} bits", N);
        }
        let bits = self.get_bits::<N>();
        bits.to_u16()
    }

    pub fn get_char(&mut self) -> char {
        let ascii = self.get_u8::<7>();
        ascii as char
    }
}