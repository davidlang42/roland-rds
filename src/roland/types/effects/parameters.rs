use schemars::JsonSchema;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use validator::Validate;

use crate::{json::{type_name_pretty, validation::out_of_range_err, schema::{u16_schema, i8_schema}}, roland::types::enums::Pan};

use super::super::numeric::Parameter;

/// Parameter(0-2) === FilterType(Off, LowPassFilter, HighPassFilter)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum FilterType {
    Off,
    LowPassFilter,
    HighPassFilter
}

/// Parameter(0-2) === SuperFilterType(LowPassFilter - Notch)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum SuperFilterType {
    LowPassFilter,
    BandPassFilter,
    HighPassFilter,
    Notch
}

/// Parameter(0-1) === SimpleFilterType(LowPassFilter, BandPassFilter)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum SimpleFilterType {
    LowPassFilter,
    BandPassFilter
}

/// Parameter(0-1) === RateMode(Hertz, Note)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum RateMode {
    Hertz,
    Note
}

/// Parameter(0-1) === DelayMode(Milliseconds, Note)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum DelayMode {
    Milliseconds,
    Note
}

/// Parameter(0-21) === NoteLength(64th Triplet - Double Note)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
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

/// Parameter(0-7) === ReverbCharacter(Room1 - PanDelay)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
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

/// Parameter(0-7) === Gm2ReverbCharacter(Room1 - PanDelay)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
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

/// Parameter(0-2) === BoostWidth(Wide, Mid, Narrow)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum BoostWidth {
    Wide,
    Mid,
    Narrow
}

/// Parameter(0-4) === Wave(Triange - Saw2)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum Wave {
    Triangle,
    Square,
    Sine,
    Saw1, // upward
    Saw2 // downward
}

/// Parameter(0-1) === Direction(Up, Down)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down
}

/// Parameter(0-4) === Vowel(A, E, I, O, U)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum Vowel {
    A,
    E,
    I,
    O,
    U
}

/// Parameter(0-15) === SpeakerType(Small1 - ThreeStack)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum SpeakerType {
    Small1,
    Small2,
    Middle,
    Jc120,
    BuiltIn1,
    BuiltIn2,
    BuiltIn3,
    BuiltIn4,
    BuiltIn5,
    BgStack1,
    BgStack2,
    MsStack1,
    MsStack2,
    MetalStack,
    TwoStack,
    ThreeStack
}

/// Parameter(0-2) === PhaserMode(FourStage - TwelveStage)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum PhaserMode {
    FourStage,
    EightStage,
    TwelveStage
}

/// Parameter(0-5) === MultiPhaserMode(FourStage - TwentyFourStage)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum MultiPhaserMode {
    FourStage,
    EightStage,
    TwelveStage,
    SixteenStage,
    TwentyStage,
    TwentyFourStage
}

/// Parameter(0-1) === PhaserPolarity(Inverse, Synchro)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum PhaserPolarity {
    Inverse,
    Synchro
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
        match value.0 {
            0 => Self(false),
            1 => Self(true),
            v => panic!("Invalid Switch: Parameter({})", v)
        }
    }
}

/// Parameter(MIN-MAX) === UInt(MIN-MAX)
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct UInt<const MIN: u16, const MAX: u16>(pub u16);

impl<const MIN: u16, const MAX: u16> UInt<MIN, MAX> {
    fn validate_generic_range() -> () {
        if MIN > Parameter::MAX as u16 {
            panic!("Invalid UInt range: MIN({}) > {}", MIN, Parameter::MAX);
        }
        if MAX > Parameter::MAX as u16 {
            panic!("Invalid UInt range: MAX({}) > {}", MAX, Parameter::MAX);
        }
        if MIN > MAX {
            panic!("Invalid UInt range: MIN({}) > MAX({})", MIN, MAX);
        }
    }
}

impl<const MIN: u16, const MAX: u16> Into<Parameter> for UInt<MIN, MAX> {
    fn into(self) -> Parameter {
        Self::validate_generic_range();
        if self.0 < MIN || self.0 > MAX {
            panic!("Invalid Parameter: UInt<{},{}>({})", MIN, MAX, self.0);
        }
        Parameter(self.0 as i16)
    }
}

impl<const MIN: u16, const MAX: u16> From<Parameter> for UInt<MIN, MAX> {
    fn from(value: Parameter) -> Self {
        Self::validate_generic_range();
        if value.0 < MIN as i16 || value.0 > MAX as i16 {
            panic!("Invalid UInt<{},{}>: Parameter({})", MIN, MAX, value.0);
        }
        Self(value.0 as u16)
    }
}

impl<const MIN: u16, const MAX: u16> JsonSchema for UInt<MIN, MAX> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        u16_schema(MIN, MAX)
    }
}

impl<const MIN: u16, const MAX: u16> Validate for UInt<MIN, MAX> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0 < MIN || self.0 > MAX {
            Err(out_of_range_err("0", &MIN, &MAX))
        } else {
            Ok(())
        }
    }
}

pub type Level = UInt<0, 127>;
pub type LinearMilliseconds<const MAX: u16> = UInt<1, MAX>;
pub type Phase = UInt<0, 180>;
pub type PreLpf = UInt<0, 7>;

/// Parameter(0-(MAX-MIN)) === Int(MIN-MAX)
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Int<const MIN: i8, const MAX: i8>(pub i8);

impl<const MIN: i8, const MAX: i8> Int<MIN, MAX> {
    fn validate_generic_range() -> () {
        if MIN > MAX {
            panic!("Invalid Int range: MIN({}) > MAX({})", MIN, MAX);
        }
    }
}

impl<const MIN: i8, const MAX: i8> Into<Parameter> for Int<MIN, MAX> {
    fn into(self) -> Parameter {
        Self::validate_generic_range();
        if self.0 < MIN || self.0 > MAX {
            panic!("Invalid Parameter: Int<{},{}>({})", MIN, MAX, self.0);
        }
        Parameter(self.0 as i16 - MIN as i16)
    }
}

impl<const MIN: i8, const MAX: i8> From<Parameter> for Int<MIN, MAX> {
    fn from(value: Parameter) -> Self {
        Self::validate_generic_range();
        if value.0 < 0 || value.0 > (MAX as i16 - MIN as i16) {
            panic!("Invalid Int<{},{}>: Parameter({})", MIN, MAX, value.0);
        }
        Self((value.0 + MIN as i16) as i8)
    }
}

impl<const MIN: i8, const MAX: i8> JsonSchema for Int<MIN, MAX> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        i8_schema(MIN, MAX)
    }
}

impl<const MIN: i8, const MAX: i8> Validate for Int<MIN, MAX> {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.0 < MIN || self.0 > MAX {
            Err(out_of_range_err("0", &MIN, &MAX))
        } else {
            Ok(())
        }
    }
}

pub type Gain = Int<-15, 15>;
pub type DampGain = Int<-36, 0>;
pub type BoostGain = Int<-60, 4>;
pub type Size = Int<1, 8>;

// Parameter(0-127) === Pan(L64-63R)
impl Into<Parameter> for Pan {
    fn into(self) -> Parameter {
        let v: u8 = self.into();
        if v > 127 {
            panic!("Invalid Parameter: {:?}", self);
        }
        Parameter(v as i16)
    }
}

impl From<Parameter> for Pan {
    fn from(value: Parameter) -> Self {
        if value.0 < 0 || value.0 > 127 {
            panic!("Invalid Pan: Parameter({})", value.0);
        }
        (value.0 as u8).into()
    }
}