use schemars::JsonSchema;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

use super::numeric::Parameter;

#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, PartialEq, Copy, Clone)]
pub enum FilterType {
    Off,
    LowPassFilter,
    HighPassFilter
}

impl From<Parameter> for FilterType {
    fn from(value: Parameter) -> Self {
        Self::iter().nth(Into::<u16>::into(value) as usize).unwrap()
    }
}

impl Into<Parameter> for FilterType {
    fn into(self) -> Parameter {
        (Self::iter().position(|s| s == self).unwrap() as u16).into()
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
        Self::iter().nth(Into::<u16>::into(value) as usize).unwrap()
    }
}

impl Into<Parameter> for RateMode {
    fn into(self) -> Parameter {
        (Self::iter().position(|s| s == self).unwrap() as u16).into()
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
        Self::iter().nth(Into::<u16>::into(value) as usize).unwrap()
    }
}

impl Into<Parameter> for DelayMode {
    fn into(self) -> Parameter {
        (Self::iter().position(|s| s == self).unwrap() as u16).into()
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
        Self::iter().nth(Into::<u16>::into(value) as usize).unwrap()
    }
}

impl Into<Parameter> for NoteLength {
    fn into(self) -> Parameter {
        (Self::iter().position(|s| s == self).unwrap() as u16).into()
    }
}