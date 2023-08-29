use schemars::JsonSchema;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use super::super::numeric::Parameter;

/// Parameter(0-?)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq, Copy, Clone)]
pub enum FilterType {
    Off,
    LowPassFilter,
    HighPassFilter
}

impl From<Parameter> for FilterType {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(value.0 as usize).unwrap()
    }
}

impl Into<Parameter> for FilterType {
    fn into(self) -> Parameter {
        Parameter(Self::iter().position(|s| s == self).unwrap() as i16)
    }
}

/// Parameter(0-?)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq, Copy, Clone)]
pub enum RateMode {
    Hertz,
    Note
}

impl From<Parameter> for RateMode {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(value.0 as usize).unwrap()
    }
}

impl Into<Parameter> for RateMode {
    fn into(self) -> Parameter {
        Parameter(Self::iter().position(|s| s == self).unwrap() as i16)
    }
}

/// Parameter(0-?)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq, Copy, Clone)]
pub enum DelayMode {
    Milliseconds,
    Note
}

impl From<Parameter> for DelayMode {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(value.0 as usize).unwrap()
    }
}

impl Into<Parameter> for DelayMode {
    fn into(self) -> Parameter {
        Parameter(Self::iter().position(|s| s == self).unwrap() as i16)
    }
}

/// Parameter(0-?)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq, Copy, Clone)]
pub enum NoteLength {
    SixtyFourthNoteTriplet,
    SixtyFourthNote,
    ThirtySecondNoteTriplet,
    ThirtySecondNote,
    SixteenthNoteTriplet,
    DottedThirtySecondNote,
    SixteenthNote,
    EighthNoteTriplet,
    DottedSixteenthNote,
    EighthNote,
    QuarterNoteTriplet,
    DottedEighthNote,
    QuarterNote,
    HalfNoteTriplet,
    DottedQuarterNote,
    HalfNote,
    WholeNoteTriplet,
    DottedHalfNote,
    WholeNote,
    DoubleNoteTriplet,
    DottedWholeNote,
    DoubleNote
}

impl From<Parameter> for NoteLength {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(value.0 as usize).unwrap()
    }
}

impl Into<Parameter> for NoteLength {
    fn into(self) -> Parameter {
        Parameter(Self::iter().position(|s| s == self).unwrap() as i16)
    }
}

/// Parameter(0-?)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq, Copy, Clone)]
pub enum ReverbCharacter {
    Room1,
    Room2,
    Stage1,
    Stage2,
    Hall1,
    Hall2,
    Delay,
    PanDelay
}

impl From<Parameter> for ReverbCharacter {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(value.0 as usize).unwrap()
    }
}

impl Into<Parameter> for ReverbCharacter {
    fn into(self) -> Parameter {
        Parameter(Self::iter().position(|s| s == self).unwrap() as i16)
    }
}

/// Parameter(0-?)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq, Copy, Clone)]
pub enum Gm2ReverbCharacter {
    Room1,
    Room2,
    Room3,
    Hall1,
    Hall2,
    Plate,
    Delay,
    PanDelay
}

impl From<Parameter> for Gm2ReverbCharacter {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(value.0 as usize).unwrap()
    }
}

impl Into<Parameter> for Gm2ReverbCharacter {
    fn into(self) -> Parameter {
        Parameter(Self::iter().position(|s| s == self).unwrap() as i16)
    }
}

/// Parameter(0-127) === Level(0-127)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Level(pub u8);
//TODO #[validate(range(max = 127))]

impl Into<Parameter> for Level {
    fn into(self) -> Parameter {
        Parameter(self.0 as i16)
    }
}

impl From<Parameter> for Level {
    fn from(value: Parameter) -> Self {
        Self(value.0 as u8)
    }
}

/// Parameter(0-180) === Phase(0-180)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Phase(pub u8);
//TODO #[validate(range(max = 180))]

impl Into<Parameter> for Phase {
    fn into(self) -> Parameter {
        Parameter(self.0 as i16)
    }
}

impl From<Parameter> for Phase {
    fn from(value: Parameter) -> Self {
        Self(value.0 as u8)
    }
}

/// Parameter(0-7) === PreLpf(0-7)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PreLpf(pub u8);
//TODO #[validate(range(max = 7))]

impl Into<Parameter> for PreLpf {
    fn into(self) -> Parameter {
        Parameter(self.0 as i16)
    }
}

impl From<Parameter> for PreLpf {
    fn from(value: Parameter) -> Self {
        Self(value.0 as u8)
    }
}

/// Parameter(1-?) === LinearMilliseconds(1-MAX)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LinearMilliseconds<const MAX: u16>(pub u16);
//TODO #[validate(range(min = 1, max = 1000)]

impl<const MAX: u16> Into<Parameter> for LinearMilliseconds<MAX> {
    fn into(self) -> Parameter {
        Parameter(self.0 as i16)
    }
}

impl<const MAX: u16> From<Parameter> for LinearMilliseconds<MAX> {
    fn from(value: Parameter) -> Self {
        Self(value.0 as u16)
    }
}

//TODO confirm
/// Parameter(0- MAX-1) === Size(1-MAX)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Size(pub u8);
//TODO #[validate(range(min = 1, max = MAX)]

impl Into<Parameter> for Size {
    fn into(self) -> Parameter {
        Parameter((self.0 - 1) as i16)
    }
}

impl From<Parameter> for Size {
    fn from(value: Parameter) -> Self {
        Self(value.0 as u8 + 1)
    }
}

/// Parameter(0-1) === Switch(False, True)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Switch(pub bool);

impl Into<Parameter> for Switch {
    fn into(self) -> Parameter {
        Parameter(if self.0 { 1 } else { 0 })
    }
}

impl From<Parameter> for Switch {
    fn from(value: Parameter) -> Self {
        Self(value.0 == 1)
    }
}

/// Parameter(0-?) === Gain(MIN-MAX)
#[derive(Debug, Copy, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Gain<const MIN: i8, const MAX: i8>(pub i8);
//TODO #[validate(range(max = 127))]

impl<const MIN: i8, const MAX: i8> Into<Parameter> for Gain<MIN, MAX> {
    fn into(self) -> Parameter {
        Parameter((self.0 - MIN) as i16)
    }
}

impl<const MIN: i8, const MAX: i8> From<Parameter> for Gain<MIN, MAX> {
    fn from(value: Parameter) -> Self {
        Self(value.0 as i8 + MIN)
    }
}