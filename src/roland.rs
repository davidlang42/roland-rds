use std::fmt::Display;

pub struct RD300NX {

}

#[derive(Debug, Copy, Clone)]
pub struct Bit(bool);

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 {
            '1'
        } else {
            '0'
        })
    }
}

impl Bit {
    const ONE: Bit = Bit(true);
    const ZERO: Bit = Bit(false);
}

pub struct Bits(Vec<Bit>);

impl Display for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in &self.0 {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

pub struct BitsRef<'a, const N: usize>([&'a Bit; N]);

impl<'a, const N: usize> Display for BitsRef<'a, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bit in self.0 {
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseError {
    _file_byte: usize,
    _message: String
}

impl From<Vec<u8>> for RD300NX {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bits = Vec::with_capacity(bytes.len() * 8);
        for byte in bytes.iter().take(14) {
            println!("raw: {}", byte);
            let temp = byte_to_bits(*byte);
            println!("byte: {}", Bits(temp.iter().map(|t| *t).collect()));
            for bit in temp {
                bits.push(bit);
            }
        }
        Self::parse(bits).unwrap()
    }
}

impl RD300NX {
    fn parse(bits: Vec<Bit>) -> Result<Self, ParseError> {
        println!("Total bits: {}", bits.len());
        let mut iter = bits.iter();
        for i in 0..16 {
            let bits = iter.next_chunk::<7>().unwrap(); //TODO handle error
            let ascii = bits_to_u8(bits);
            //let ch = take_char(&mut iter)?;
            println!("#{} [{}]: '{}' ({})", i, BitsRef(bits), ascii as char, ascii);
        }
        todo!();
    }
}

fn take_char<'a, N: Iterator<Item = &'a Bit>>(iter: &mut N) -> Result<char, ParseError> {
    let bits = iter.next_chunk::<7>().unwrap(); //TODO handle error
    let ascii = bits_to_u8(bits);
    Ok(ascii as char)
}

fn bits_to_u8<const N: usize>(bits: [&Bit; N]) -> u8 {
    let mut num = 0;
    let mut bit_value = 2u8.pow((bits.len() - 1) as u32);
    for bit in bits {
        if bit.0 {
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