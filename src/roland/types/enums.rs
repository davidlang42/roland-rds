use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum OutputPort { // 0-5
    All,
    Int,
    Out1,
    Out2,
    Out3,
    Usb
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum MonoPoly { // 0-2
    Mono,
    Poly,
    MonoLegato
}

impl From<u8> for MonoPoly {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for MonoPoly {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for MonoPoly {
    fn default() -> Self {
        Self::from(0)
    }
}

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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum NuanceType { // 0-2
    Type1,
    Type2,
    Type3
}

impl From<u8> for NuanceType {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for NuanceType {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for NuanceType {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum StretchTuneType { // 0-2
    Off,
    Preset,
    User
}

impl From<u8> for StretchTuneType {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for StretchTuneType {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for StretchTuneType {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Pan { // 0-127 (L64 - 63R)
    Left(u8),
    Centre,
    Right(u8)
}

impl Pan {
    const CENTRE: u8 = 64;
}

impl From<u8> for Pan {
    fn from(value: u8) -> Self {
        if value > Self::CENTRE {
            Self::Right(value - Self::CENTRE)
        } else if value < Self::CENTRE {
            Self::Left(Self::CENTRE - value)
        } else {
            Self::Centre
        }
    }
}

impl Into<u8> for Pan {
    fn into(self) -> u8 {
        match self {
            Self::Left(l) => Self::CENTRE - l,
            Self::Centre => Self::CENTRE,
            Self::Right(r) => Self::CENTRE + r
        }
    }
}

impl Default for Pan {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum PatchCategory { // 0-3
    OneTouchPiano,
    OneTouchEPiano,
    Preset,
    User
}

impl From<u8> for PatchCategory {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for PatchCategory {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for PatchCategory {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum SettingMode {
    LiveSet,
    System
}

impl From<bool> for SettingMode {
    fn from(value: bool) -> Self {
        match value {
            false => Self::LiveSet,
            true => Self::System
        }
    }
}

impl Into<bool> for SettingMode {
    fn into(self) -> bool {
        match self {
            Self::LiveSet => false,
            Self::System => true
        }
    }
}

impl Default for SettingMode {
    fn default() -> Self {
        Self::from(false)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum Polarity {
    Standard,
    Reverse
}

impl From<bool> for Polarity {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Standard,
            true => Self::Reverse
        }
    }
}

impl Into<bool> for Polarity {
    fn into(self) -> bool {
        match self {
            Self::Standard => false,
            Self::Reverse => true
        }
    }
}

impl Default for Polarity {
    fn default() -> Self {
        Self::from(false)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum Layer { // 0-3
    Upper1,
    Upper2,
    Lower1,
    Lower2
}

impl From<u8> for Layer {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for Layer {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum KeyOffPosition {
    Standard,
    Deep
}

impl From<bool> for KeyOffPosition {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Standard,
            true => Self::Deep
        }
    }
}

impl Into<bool> for KeyOffPosition {
    fn into(self) -> bool {
        match self {
            Self::Standard => false,
            Self::Deep => true
        }
    }
}

impl Default for KeyOffPosition {
    fn default() -> Self {
        Self::from(false)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum SliderSelect {
    LayerLevel,
    Control
}

impl From<bool> for SliderSelect {
    fn from(value: bool) -> Self {
        match value {
            false => Self::LayerLevel,
            true => Self::Control
        }
    }
}

impl Into<bool> for SliderSelect {
    fn into(self) -> bool {
        match self {
            Self::LayerLevel => false,
            Self::Control => true
        }
    }
}

impl Default for SliderSelect {
    fn default() -> Self {
        Self::from(false)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum KeyTouchVelocity { // 0-127 (REAL, 1-127)
    Real,
    Fixed(u8)
}

impl From<u8> for KeyTouchVelocity {
    fn from(value: u8) -> Self {
        if value == 0 {
            Self::Real
        } else {
            Self::Fixed(value)
        }
    }
}

impl Into<u8> for KeyTouchVelocity {
    fn into(self) -> u8 {
        match self {
            Self::Real => 0,
            Self::Fixed(v) => v
        }
    }
}

impl Default for KeyTouchVelocity {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum KeyTouchCurveType { // 1-5
    SuperLight,
    Light,
    Medium,
    Heavy,
    SuperHeavy
}

impl From<u8> for KeyTouchCurveType {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize - 1).unwrap()
    }
}

impl Into<u8> for KeyTouchCurveType {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8 + 1
    }
}

impl Default for KeyTouchCurveType {
    fn default() -> Self {
        Self::from(1)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum OutputSelect { // 0-2
    Main,
    Rev,
    Both,
}

impl From<u8> for OutputSelect {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for OutputSelect {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for OutputSelect {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum ChorusType { // 0-3
    Off,
    Chorus,
    Delay,
    Gm2Chorus
}

impl From<u8> for ChorusType {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for ChorusType {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for ChorusType {
    fn default() -> Self {
        Self::from(0)
    }
}
