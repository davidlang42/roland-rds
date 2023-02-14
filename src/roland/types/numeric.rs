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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct OffsetU8<const OFFSET: u8>(i8); // MIN(0)-MAX(255) (MIN-OFFSET - MAX-OFFSET)

impl<const O: u8> OffsetU8<O> {
    const ZERO: u8 = O;
}

impl<const O: u8> From<u8> for OffsetU8<O> {
    fn from(value: u8) -> Self {
        if value >= Self::ZERO {
            Self((value - Self::ZERO) as i8)
        } else {
            Self(-1 * (Self::ZERO - value) as i8)
        }
    }
}

impl<const O: u8> Into<u8> for OffsetU8<O> {
    fn into(self) -> u8 {
        if self.0 >= 0 {
            self.0 as u8 + Self::ZERO
        } else {
            Self::ZERO - self.0.abs() as u8
        }
    }
}

impl<const O: u8> Default for OffsetU8<O> {
    fn default() -> Self {
        Self::from(Self::ZERO)
    }
}


#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct OneIndexedU16(u16); // 0-65534 (1-65535)

impl From<u16> for OneIndexedU16 {
    fn from(value: u16) -> Self {
        Self(value + 1)
    }
}

impl Into<u16> for OneIndexedU16 {
    fn into(self) -> u16 {
        self.0 - 1
    }
}

impl Default for OneIndexedU16 {
    fn default() -> Self {
        Self::from(0)
    }
}
