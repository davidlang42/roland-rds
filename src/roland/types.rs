use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Parameter(i16); // 12768-52768 (-20000 - +20000)

impl Parameter {
    const ZERO: u16 = 32768;
}

impl From<u16> for Parameter {
    fn from(value: u16) -> Self {
        if value >= Self::ZERO {
            Self((value - Self::ZERO) as i16)
        } else {
            Self(-1 * (Self::ZERO - value) as i16)
        }
    }
}

impl Into<u16> for Parameter {
    fn into(self) -> u16 {
        if self.0 >= 0 {
            self.0 as u16 + Self::ZERO
        } else {
            Self::ZERO - self.0.abs() as u16
        }
    }
}

impl Default for Parameter {
    fn default() -> Self {
        Self::from(Self::ZERO)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum OutputPort { // 0-5
    ALL,
    INT,
    OUT1,
    OUT2,
    OUT3,
    USB
}

impl From<u8> for OutputPort {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for OutputPort {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for OutputPort {
    fn default() -> Self {
        Self::from(0)
    }
}
