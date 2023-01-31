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

    // pub fn string<'a, I: IntoIterator<Item = &'a Bit>>(bits: I) -> String {
    //     let mut s = String::new();
    //     for bit in bits {
    //         s.push(bit.to_char())
    //     }
    //     s
    // }
}

// pub struct Bits<const N: usize>([Bit; N]);

// impl<const N: usize> Display for Bits<N> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for bit in self.0 {
//             write!(f, "{}", bit)?;
//         }
//         Ok(())
//     }
// }

// impl From<u8> for Bits<8> {
//     fn from(byte: u8) -> Self {
        
//     }
// }

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
            for bit in byte_to_bits(byte) {
                bits.push(bit);
            }
        }
        Self {
            bits,
            index: 0
        }
    }
}

impl BitStream {
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn offset(&self) -> usize {
        self.index
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }

    pub fn set_offset(&mut self, offset: usize) {
        self.index = offset;
    }

    pub fn get_bool(&mut self) -> Option<bool> {
        let bit = self.next()?;
        Some(bit.on())
    }

    fn get_bits(&mut self, bits: usize) -> Option<Vec<Bit>> {
        let mut result = Vec::with_capacity(bits);
        for _ in 0..bits {
            result.push(self.next()?);
        }
        Some(result)
    }

    pub fn get_u8(&mut self, bits: usize) -> Option<u8> {
        if bits > 8 {
            panic!("Cannot get u8 from {} bits", bits);
        }
        let bits = self.get_bits(bits)?;
        Some(bits_to_u8(&bits))
    }

    pub fn get_char(&mut self) -> Option<char> {
        let ascii = self.get_u8(7)?;
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

fn bits_to_u8(bits: &Vec<Bit>) -> u8 {
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

fn byte_to_bits(mut byte: u8) -> [Bit; 8] {
    let mut bits = [Bit::ZERO; 8];
    let mut bit_value = 2u8.pow((bits.len() - 1) as u32);
    for bit_index in 0..bits.len() {
        if byte >= bit_value {
            byte -= bit_value;
            bits[bit_index] = Bit::ONE;
        }
        bit_value /= 2;
    }
    bits
}