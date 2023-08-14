use schemars::JsonSchema;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use super::super::numeric::Parameter;

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

impl Default for FilterType {
    fn default() -> Self {
        Self::from(Parameter::default())
    }
}

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

impl Default for RateMode {
    fn default() -> Self {
        Self::from(Parameter::default())
    }
}

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

impl Default for DelayMode {
    fn default() -> Self {
        Self::from(Parameter::default())
    }
}

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

impl Default for ReverbCharacter {
    fn default() -> Self {
        Self::from(Parameter::default())
    }
}

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

impl Default for Gm2ReverbCharacter {
    fn default() -> Self {
        Self::from(Parameter::default())
    }
}
