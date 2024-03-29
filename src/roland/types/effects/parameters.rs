use schemars::JsonSchema;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use validator::Validate;

use crate::{json::{type_name_pretty, validation::out_of_range_err, schema::{u16_schema, i16_schema}}, roland::types::enums::Pan};

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

/// Parameter(0-5) === ReverbOnlyCharacter(Room1 - Hall2)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum ReverbOnlyCharacter {
    Room1,
    Room2,
    Stage1,
    Stage2,
    Hall1,
    Hall2
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

/// Parameter(0-5) === ModWave(Triange - Trapezoidal)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum ModWave {
    Triangle,
    Square,
    Sine,
    Saw1, // upward
    Saw2, // downward
    Trapezoidal
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
    #[serde(rename = "4-Stage")]
    FourStage,
    #[serde(rename = "8-Stage")]
    EightStage,
    #[serde(rename = "12-Stage")]
    TwelveStage
}

/// Parameter(0-5) === MultiPhaserMode(FourStage - TwentyFourStage)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum MultiPhaserMode {
    #[serde(rename = "4-Stage")]
    FourStage,
    #[serde(rename = "8-Stage")]
    EightStage,
    #[serde(rename = "12-Stage")]
    TwelveStage,
    #[serde(rename = "16-Stage")]
    SixteenStage,
    #[serde(rename = "20-Stage")]
    TwentyStage,
    #[serde(rename = "24-Stage")]
    TwentyFourStage
}

/// Parameter(0-1) === PhaserPolarity(Inverse, Synchro)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum PhaserPolarity {
    Inverse,
    Synchro
}

/// Parameter(0-1) === SlicerMode(Legato, Slash)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum SlicerMode {
    Legato,
    Slash
}

/// Parameter(0-1) === Speed(Slow, Fast)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum Speed {
    Slow,
    Fast
}

/// Parameter(0-1) === OutputMode(Speaker, Phones)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum OutputMode {
    Speaker,
    Phones
}

/// Parameter(0-3) === AmpType(Small - ThreeStack)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum AmpType {
    Small,
    BuiltIn,
    TwoStack,
    ThreeStack
}

/// Parameter(0-13) === PreAmpType(Jc120 - Fuzz)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum PreAmpType {
    Jc120,
    CleanTwin,
    MatchDrive,
    BgLead,
    Ms19591,
    Ms19592,
    Ms19591_2,
    SldnLead,
    Metal5150,
    MetalLead,
    Od1,
    Od2Turbo,
    Distortion,
    Fuzz
}

/// Parameter(0-2) === PreAmpGain(Low, Middle, High)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum PreAmpGain {
    Low,
    Middle,
    High
}

/// Parameter(0-3) === CompressionRatio(1.5:1 - 100:1)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum CompressionRatio {
    #[serde(rename = "1.5:1")]
    OnePointFiveToOne,
    #[serde(rename = "2:1")]
    TwoToOne,
    #[serde(rename = "4:1")]
    FourToOne,
    #[serde(rename = "100:1")]
    OneHundredToOne
}

/// Parameter(0-1) === GateMode(Gate, Duck)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum GateMode {
    Gate,
    Duck
}

/// Parameter(0-1) === PhaseType(Normal, Inverse)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum PhaseType {
    Normal,
    Inverse
}

/// Parameter(0-1) === FeedbackMode(Normal, Cross)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum FeedbackMode {
    Normal,
    Cross
}

/// Parameter(0-6) === TapeHeads(Short - ShortMiddleLong)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum TapeHeads {
    Short,
    Middle,
    Long,
    ShortMiddle,
    ShortLong,
    MiddleLong,
    ShortMiddleLong
}

/// Parameter(0-1) === NoiseType(White, Pink)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum NoiseType {
    White,
    Pink
}

/// Parameter(0-3) === DiscTypeWithRandom(LP - RND)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum DiscTypeWithRandom {
    LP,
    EP,
    SP,
    RND
}

/// Parameter(0-2) === DiscType(LP - SP)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum DiscType {
    LP,
    EP,
    SP
}

/// Parameter(0-3) === GateType(Normal - Sweep2)
#[derive(Serialize, Deserialize, Debug, JsonSchema, EnumIter, EnumParameter, PartialEq, Copy, Clone)]
pub enum GateType {
    Normal,
    Reverse,
    Sweep1,
    Sweep2
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
pub type PreLpf = UInt<0, 7>;

/// Parameter(0-(MAX-MIN)) === Int(MIN-MAX)
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Int<const MIN: isize, const MAX: isize>(pub isize);

impl<const MIN: isize, const MAX: isize> Int<MIN, MAX> {
    fn validate_generic_range() -> () {
        if MIN > MAX {
            panic!("Invalid Int range: MIN({}) > MAX({})", MIN, MAX);
        }
    }
}

impl<const MIN: isize, const MAX: isize> Into<Parameter> for Int<MIN, MAX> {
    fn into(self) -> Parameter {
        Self::validate_generic_range();
        if self.0 < MIN || self.0 > MAX {
            panic!("Invalid Parameter: Int<{},{}>({})", MIN, MAX, self.0);
        }
        Parameter(self.0 as i16 - MIN as i16)
    }
}

impl<const MIN: isize, const MAX: isize> From<Parameter> for Int<MIN, MAX> {
    fn from(value: Parameter) -> Self {
        Self::validate_generic_range();
        if value.0 < 0 || value.0 > (MAX as i16 - MIN as i16) {
            panic!("Invalid Int<{},{}>: Parameter({})", MIN, MAX, value.0);
        }
        Self((value.0 + MIN as i16) as isize)
    }
}

impl<const MIN: isize, const MAX: isize> JsonSchema for Int<MIN, MAX> {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        i16_schema(MIN as i16, MAX as i16)
    }
}

impl<const MIN: isize, const MAX: isize> Validate for Int<MIN, MAX> {
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
pub type PostGain = Int<0, 18>;
pub type Size = Int<1, 8>;
pub type MicSetting = Int<1, 3>;
pub type LofiType = Int<1, 9>;
pub type Semitones = Int<-24, 12>;

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