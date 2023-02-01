use std::fmt::Display;

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
            bytes.push(Self::to_byte(self.0[start..end].try_into().unwrap()));
            start = end;
        }
        bytes
    }

    pub fn compress(text: [char; 16]) -> Bits<112> { //TODO I really should be able to make this generic
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

impl<I: Iterator<Item = u8>> From<I> for BitStream {
    fn from(bytes: I) -> Self {
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
}

impl Display for BitStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in self.index..self.bits.len() {
            write!(f, "{}", self.bits[i])?;
        }
        Ok(())
    }
}

impl BitStream {
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn offset(&self) -> usize {
        self.index
    }

    pub fn eof(&self) -> bool {
        self.index >= self.bits.len()
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.index = offset;
    }

    pub fn get_bool(&mut self) -> Option<bool> {
        let bit = self.next()?;
        Some(bit.on())
    }

    pub fn get_bits<const N: usize>(&mut self) -> Option<Bits<N>> {
        let mut result = Vec::new();
        for _ in 0..N {
            result.push(self.next()?);
        }
        Some(Bits(result.try_into().unwrap()))
    }

    pub fn get_u8<const N: usize>(&mut self) -> Option<u8> {
        if N > 8 { //TODO enfore this with compiler
            panic!("Cannot get u8 from {} bits", N);
        }
        let bits = self.get_bits::<N>()?;
        Some(bits.into())
    }

    pub fn get_u16<const N: usize>(&mut self) -> Option<u16> {
        if N > 16 { //TODO enfore this with compiler
            panic!("Cannot get u16 from {} bits", N);
        }
        let bits = self.get_bits::<N>()?;
        Some(bits.into())
    }

    pub fn get_char(&mut self) -> Option<char> {
        let ascii = self.get_u8::<7>()?;
        Some(ascii as char)
    }

    pub fn search(&mut self, find: &str) -> Option<usize> {
        let original_index = self.index;
        let mut search_index = self.index;
        while search_index < self.len() {
            let mut success = true;
            for ch in find.chars() {
                if self.get_char().unwrap() != ch { //TODO this will crash
                    success = false;
                    break;
                }
            }
            if success {
                self.set_offset(original_index);
                return Some(search_index);
            } else {
                search_index += 1;
                self.set_offset(search_index);
            }
        }
        self.set_offset(original_index);
        None
    }
}
