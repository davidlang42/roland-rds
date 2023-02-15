
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum PianoKey { // 0-87
    A0,
    Bb0,
    B0,
    C1,
    Db1,
    D1,
    Eb1,
    E1,
    F1,
    Gb1,
    G1,
    Ab1,
    A1,
    Bb1,
    B1,
    C2,
    Db2,
    D2,
    Eb2,
    E2,
    F2,
    Gb2,
    G2,
    Ab2,
    A2,
    Bb2,
    B2,
    C3,
    Db3,
    D3,
    Eb3,
    E3,
    F3,
    Gb3,
    G3,
    Ab3,
    A3,
    Bb3,
    B3,
    C4,
    Db4,
    D4,
    Eb4,
    E4,
    F4,
    Gb4,
    G4,
    Ab4,
    A4,
    Bb4,
    B4,
    C5,
    Db5,
    D5,
    Eb5,
    E5,
    F5,
    Gb5,
    G5,
    Ab5,
    A5,
    Bb5,
    B5,
    C6,
    Db6,
    D6,
    Eb6,
    E6,
    F6,
    Gb6,
    G6,
    Ab6,
    A6,
    Bb6,
    B6,
    C7,
    Db7,
    D7,
    Eb7,
    E7,
    F7,
    Gb7,
    G7,
    Ab7,
    A7,
    Bb7,
    B7,
    C8
}

impl From<u8> for PianoKey {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for PianoKey {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for PianoKey {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, Hash, Eq, Ord, PartialOrd)]
pub enum MidiNote { // 0-127
    CMinus1,
    DbMinus1,
    DMinus1,
    EbMinus1,
    EMinus1,
    FMinus1,
    GbMinus1,
    GMinus1,
    AbMinus1,
    AMinus1,
    BbMinus1,
    BMinus1,
    C0,
    Db0,
    D0,
    Eb0,
    E0,
    F0,
    Gb0,
    G0,
    Ab0,
    A0,
    Bb0,
    B0,
    C1,
    Db1,
    D1,
    Eb1,
    E1,
    F1,
    Gb1,
    G1,
    Ab1,
    A1,
    Bb1,
    B1,
    C2,
    Db2,
    D2,
    Eb2,
    E2,
    F2,
    Gb2,
    G2,
    Ab2,
    A2,
    Bb2,
    B2,
    C3,
    Db3,
    D3,
    Eb3,
    E3,
    F3,
    Gb3,
    G3,
    Ab3,
    A3,
    Bb3,
    B3,
    C4,
    Db4,
    D4,
    Eb4,
    E4,
    F4,
    Gb4,
    G4,
    Ab4,
    A4,
    Bb4,
    B4,
    C5,
    Db5,
    D5,
    Eb5,
    E5,
    F5,
    Gb5,
    G5,
    Ab5,
    A5,
    Bb5,
    B5,
    C6,
    Db6,
    D6,
    Eb6,
    E6,
    F6,
    Gb6,
    G6,
    Ab6,
    A6,
    Bb6,
    B6,
    C7,
    Db7,
    D7,
    Eb7,
    E7,
    F7,
    Gb7,
    G7,
    Ab7,
    A7,
    Bb7,
    B7,
    C8,
    Db8,
    D8,
    Eb8,
    E8,
    F8,
    Gb8,
    G8,
    Ab8,
    A8,
    Bb8,
    B8,
    C9,
    Db9,
    D9,
    Eb9,
    E9,
    F9,
    Gb9,
    G9
}

impl From<u8> for MidiNote {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for MidiNote {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for MidiNote {
    fn default() -> Self {
        Self::from(0)
    }
}
    
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum KeyNote { // 0-11
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B
}

impl From<u8> for KeyNote {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for KeyNote {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for KeyNote {
    fn default() -> Self {
        Self::from(0)
    }
}