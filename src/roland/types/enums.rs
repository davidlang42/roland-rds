use schemars::JsonSchema;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, Display};
use validator::Validate;

use crate::json::{validation::{validate_control_change, out_of_range_err, unused_by_rd300nx_err}, type_name_pretty, schema::{one_of_schema, enum_schema, u8_schema, single_property_schema, enum_except_one_schema}};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
pub enum TransmitPort { // 0-4
    All,
    Out1,
    Out2,
    Out3,
    Usb
}

impl From<u8> for TransmitPort {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for TransmitPort {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for TransmitPort {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
pub enum MonoPolyOnly { // 0-1
    Mono,
    Poly
}

impl From<u8> for MonoPolyOnly {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for MonoPolyOnly {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for MonoPolyOnly {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum Pan { // 0-127 (L64 - 63R)
    Left(u8),
    Centre,
    Right(u8)
}

impl Pan {
    const CENTRE: u8 = 64;
    const MIN_LR: u8 = 1;
    const MAX_L: u8 = 64;
    const MAX_R: u8 = 63;
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

impl Validate for Pan {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Left(l) if *l < Self::MIN_LR || *l > Self::MAX_L => Err(out_of_range_err("Left", &Self::MIN_LR, &Self::MAX_L)),
            Self::Right(r) if *r < Self::MIN_LR || *r > Self::MAX_R => Err(out_of_range_err("Right", &Self::MIN_LR, &Self::MAX_R)),
            _ => Ok(())
        }
    }
}

impl JsonSchema for Pan {
    fn schema_name() -> String {
        type_name_pretty::<Pan>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        one_of_schema(vec![
            single_property_schema("Left", u8_schema(1, Self::MAX_L)),
            enum_schema(vec!["Centre".into()]),
            single_property_schema("Right", u8_schema(1, Self::MAX_R))
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, Hash, Eq, Ord, PartialOrd, JsonSchema, Display)]
pub enum Layer { // 0-3
    Upper1,
    Upper2,
    Lower1,
    UnusedLower2
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema)]
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

impl KeyTouchVelocity {
    const MIN_FIXED: u8 = 1;
    const MAX_FIXED: u8 = 127;
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

impl Validate for KeyTouchVelocity {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Fixed(f) if *f < Self::MIN_FIXED || *f > Self::MAX_FIXED => Err(out_of_range_err("Fixed", &Self::MIN_FIXED, &Self::MAX_FIXED)),
            _ => Ok(())
        }
    }
}

impl JsonSchema for KeyTouchVelocity {
    fn schema_name() -> String {
        type_name_pretty::<KeyTouchVelocity>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        one_of_schema(vec![
            enum_schema(vec!["Real".into()]),
            single_property_schema("Fixed", u8_schema(Self::MIN_FIXED, Self::MAX_FIXED))
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
pub enum ReverbType { // 0-6
    Off,
    Reverb,
    Room,
    Hall,
    Plate,
    Gm2Reverb,
    Cathedral
}

impl From<u8> for ReverbType {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for ReverbType {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for ReverbType {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum SoundFocusType { // 0-31
    PianoType1,
    PianoType2,
    UnusedEPianoType, // RD700NX only
    SoundLift,
    Enhancer,
    MidBoost,
    Other(u8)
}

impl SoundFocusType {
    const MIN_OTHER: u8 = 6;
    const MAX_OTHER: u8 = 31;
}

impl From<u8> for SoundFocusType {
    fn from(value: u8) -> Self {
        if value <= 5 {
            Self::iter().nth(value as usize).unwrap()
        } else {
            Self::Other(value)
        }
    }
}

impl Into<u8> for SoundFocusType {
    fn into(self) -> u8 {
        if let Self::Other(value) = self {
            value
        } else {
            Self::iter().position(|s| s == self).unwrap() as u8
        }
    }
}

impl Default for SoundFocusType {
    fn default() -> Self {
        Self::from(0)
    }
}

impl Validate for SoundFocusType {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Other(o) if *o < Self::MIN_OTHER || *o > Self::MAX_OTHER => Err(out_of_range_err("Other", &Self::MIN_OTHER, &Self::MAX_OTHER)),
            Self::UnusedEPianoType => Err(unused_by_rd300nx_err("0", self)),
            _ => Ok(())
        }
    }
}

impl JsonSchema for SoundFocusType {
    fn schema_name() -> String {
        type_name_pretty::<SoundFocusType>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        one_of_schema(vec![
            enum_except_one_schema::<SoundFocusType>("Other"),
            single_property_schema("Other", u8_schema(Self::MIN_OTHER, Self::MAX_OTHER))
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
pub enum MfxType { // 0-255
    Thru,
    Equalizer,
    Spectrum,
    Isolator,
    LowBoost,
    SuperFilter,
    StepFilter,
    Enhancer,
    AutoWah,
    Humanizer,
    SpeakerSimulator,
    Phaser,
    StepPhaser,
    MultiStagePhaser,
    InfinitePhaser,
    RingModulator,
    StepRingModulator,
    Tremolo,
    AutoPan,
    StepPan,
    Slicer,
    Rotary,
    VkRotary,
    Chorus,
    Flanger,
    StepFlanger,
    HexaChorus,
    TremoloChorus,
    SpaceD,
    Chorus3D,
    Flanger3D,
    StepFlanger3D,
    TwoBandChorus,
    TwoBandFlanger,
    TwoBandStepFlanger,
    Overdrive,
    Distortion,
    VsOverdrive,
    VsDistortion,
    GuitarAmpSimulator,
    Compressor,
    Limiter,
    Gate,
    Delay,
    LongDelay,
    SerialDelay,
    ModulationDelay,
    ThreeTapPanDelay,
    FourTapPanDelay,
    MultiTapDelay,
    ReverseDelay,
    ShuffleDelay,
    Delay3D,
    TimeCtrlDelay,
    LongTimeCtrlDelay,
    TapeEcho,
    LofiNoise,
    LofiCompress,
    LofiRadio,
    Telephone,
    Photograph,
    PitchShifter,
    TwoVoicePitchShifter,
    StepPitchShifter,
    Reverb,
    GatedReverb,
    ChorusOverdrive,
    OverdriveFlanger,
    OverdriveDelay,
    DistortionChorus,
    DistortionFlanger,
    DistortionDelay,
    EnhancerChorus,
    EnhancerFlanger,
    EnhancerDelay,
    ChorusDelay,
    FlangerDelay,
    ChorusFlanger,
    UnusedVrChorus, //RD700NX only
    UnusedVrTremolo, //RD700NX only
    UnusedVrAutoWah, //RD700NX only
    UnusedVrPhaser, //RD700NX only
    UnusedOrganMulti, //RD700NX only
    UnusedLinedrive, //RD700NX only
    UnusedSmallPhaser, //RD700NX only
    SympatheticResonance, //RD300NX only
    Other(u8)
}

impl From<u8> for MfxType {
    fn from(value: u8) -> Self {
        if value <= 85 {
            Self::iter().nth(value as usize).unwrap()
        } else {
            Self::Other(value)
        }
    }
}

impl Into<u8> for MfxType {
    fn into(self) -> u8 {
        if let Self::Other(value) = self {
            value
        } else {
            Self::iter().position(|s| s == self).unwrap() as u8
        }
    }
}

impl Default for MfxType {
    fn default() -> Self {
        Self::from(0)
    }
}

impl Validate for MfxType {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::UnusedVrChorus => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedVrTremolo => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedVrAutoWah => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedVrPhaser => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedOrganMulti => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedLinedrive => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedSmallPhaser => Err(unused_by_rd300nx_err("0", self)),
            _ => Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, EnumIter, PartialEq, JsonSchema)]
pub enum OptionalMidiChannel { // 0-16 (OFF, 1-16)
    Off,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    Channel12,
    Channel13,
    Channel14,
    Channel15,
    Channel16
}

impl From<u8> for OptionalMidiChannel {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for OptionalMidiChannel {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for OptionalMidiChannel {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, EnumIter, PartialEq, Hash, Eq, Ord, PartialOrd, JsonSchema, Display)]
pub enum MidiChannel { // 0-15 (1-16)
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    Channel12,
    Channel13,
    Channel14,
    Channel15,
    Channel16
}

impl From<u8> for MidiChannel {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for MidiChannel {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for MidiChannel {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum VoiceReserve {  // 0-64 (0-63, full)
    Voices(u8),
    Full
}

impl VoiceReserve {
    const MAX_VOICES: u8 = 63;
}

impl From<u8> for VoiceReserve {
    fn from(value: u8) -> Self {
        if value == 64 {
            Self::Full
        } else {
            Self::Voices(value)
        }
    }
}

impl Into<u8> for VoiceReserve {
    fn into(self) -> u8 {
        match self {
            Self::Full => 64,
            Self::Voices(v) => v
        }
    }
}

impl Default for VoiceReserve {
    fn default() -> Self {
        Self::from(0)
    }
}

impl Validate for VoiceReserve {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Voices(v) if *v > Self::MAX_VOICES => Err(out_of_range_err("Voices", &0, &Self::MAX_VOICES)),
            _ => Ok(())
        }
    }
}

impl JsonSchema for VoiceReserve {
    fn schema_name() -> String {
        type_name_pretty::<VoiceReserve>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        one_of_schema(vec![
            single_property_schema("Voices", u8_schema(0, Self::MAX_VOICES)),
            enum_schema(vec!["Full".into()]),
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
pub enum HarmonicBar { // 1-9 (16',5-1/3',8',4',2-2/3',1-3/5',2',1-1/3',1')
    F16,
    F5_1_3,
    F8,
    F4,
    F2_2_3,
    F1_3_5,
    F2,
    F1_1_3,
    F1
}

impl From<u8> for HarmonicBar {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize - 1).unwrap()
    }
}

impl Into<u8> for HarmonicBar {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8 + 1
    }
}

impl Default for HarmonicBar {
    fn default() -> Self {
        Self::from(1)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, JsonSchema)]
pub enum PartMode {
    Parts16,
    Parts16PlusPerformance
}

impl From<bool> for PartMode {
    fn from(value: bool) -> Self {
        match value {
            false => Self::Parts16,
            true => Self::Parts16PlusPerformance
        }
    }
}

impl Into<bool> for PartMode {
    fn into(self) -> bool {
        match self {
            Self::Parts16 => false,
            Self::Parts16PlusPerformance => true
        }
    }
}

impl Default for PartMode {
    fn default() -> Self {
        Self::from(false)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
pub enum ButtonFunction { // 0-20
    Off,
    CouplePlusOctave,
    CoupleMinusOctave,
    CouplePlus2Octave,
    CoupleMinus2Octave,
    CouplePlus5th,
    CoupleMinus4th,
    OctaveUp,
    OctaveDown,
    StartStop,
    TapTempo,
    SongPlayStop,
    SongReset,
    SongBackward,
    SongForward,
    Mfx1Switch,
    UnusedMfx2Switch,
    RotarySpeed,
    LiveSetUp, // system only
    LiveSetDown, // system only
    PanelLock // system only
}

impl From<u8> for ButtonFunction {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for ButtonFunction {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for ButtonFunction {
    fn default() -> Self {
        Self::from(0)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum PedalFunction { // 0-146 (OFF, CC00 - CC127, BEND-UP, BEND-DOWN, AFTERTOUCH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, RHY PLY/STP, SONG PLY/STP, SONG RESET, MFX1 SW, MFX2 SW, MFX1 CONTROL, MFX2 CONTROL, ROTARY SPEED, SOUND FOCUS VALUE, LIVE SET UP, LIVE SET DOWN)
    Off,
    ControlChange(u8),
    BendUp,
    BendDown,
    AfterTouch,
    OctaveUp,
    OctaveDown,
    StartStop,
    TapTempo,
    RhythmPlayStop,
    SongPlayStop,
    SongReset,
    Mfx1Switch,
    UnusedMfx2Switch,
    RotarySpeed,
    SoundFocusValue,
    LiveSetUp, // system only
    LiveSetDown // system only
}

impl Validate for PedalFunction {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if let Self::ControlChange(cc) = self {
            validate_control_change(cc)
        } else {
            Ok(())
        }
    }
}

impl From<u8> for PedalFunction {
    fn from(value: u8) -> Self {
        if value == 0 {
            Self::Off
        } else if value <= 128 {
            Self::ControlChange(value - 1)
        } else {
            Self::iter().nth(value as usize - 127).unwrap()
        }
    }
}

impl Into<u8> for PedalFunction {
    fn into(self) -> u8 {
        match self {
            Self::Off => 0,
            Self::ControlChange(value) => value + 1,
            _ => Self::iter().position(|s| s == self).unwrap() as u8 + 127
        }
    }
}

impl Default for PedalFunction {
    fn default() -> Self {
        Self::from(0)
    }
}

impl JsonSchema for PedalFunction {
    fn schema_name() -> String {
        type_name_pretty::<PedalFunction>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        one_of_schema(vec![
            enum_except_one_schema::<PedalFunction>("ControlChange"),
            single_property_schema("ControlChange", u8_schema(0, 127))
        ])
    }
}

impl PedalFunction {
    pub fn tone_remain_warning(a: &Self, b: &Self, a_enable: bool, b_enable: bool) -> Option<String> {
        if !a.can_affect_tone_remain() && !b.can_affect_tone_remain() {
            None // neither of these types of PedalFunction can affect tone remain
        } else if a.can_affect_tone_remain() && a_enable && !b_enable {
            Some(format!("({:?}) STOPS working", a))
        } else if b.can_affect_tone_remain() && !a_enable && b_enable {
            Some(format!("({:?}) STARTS working", b))
        } else if !a_enable && !b_enable {
            None // PedalFunction doesn't matter if not turned on
        } else if a != b {
            Some(format!("({:?}) changes to {:?}", a, b))
        } else {
            None
        }
    }

    fn can_affect_tone_remain(&self) -> bool {
        match self {
            Self::ControlChange(_) => true,
            Self::BendUp => true,
            Self::BendDown => true,
            Self::AfterTouch => true,
            Self::RotarySpeed => true,
            _ => false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter)]
pub enum SliderFunction { // 0-133 (OFF, CC00 - CC127, BEND-UP, BEND-DOWN, AFTERTOUCH, MFX1 CONTROL, MFX2 CONTROL)
    Off,
    ControlChange(u8),
    BendUp,
    BendDown,
    AfterTouch,
    Mfx1Control,
    UnusedMfx2Control
}

impl From<u8> for SliderFunction {
    fn from(value: u8) -> Self {
        if value == 0 {
            Self::Off
        } else if value <= 128 {
            Self::ControlChange(value - 1)
        } else {
            Self::iter().nth(value as usize - 127).unwrap()
        }
    }
}

impl Into<u8> for SliderFunction {
    fn into(self) -> u8 {
        match self {
            Self::Off => 0,
            Self::ControlChange(value) => value + 1,
            _ => Self::iter().position(|s| s == self).unwrap() as u8 + 127
        }
    }
}

impl Default for SliderFunction {
    fn default() -> Self {
        Self::from(0)
    }
}

impl Validate for SliderFunction {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if let Self::ControlChange(cc) = self {
            validate_control_change(cc)
        } else {
            Ok(())
        }
    }
}

impl JsonSchema for SliderFunction {
    fn schema_name() -> String {
        type_name_pretty::<SliderFunction>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        one_of_schema(vec![
            enum_except_one_schema::<SliderFunction>("ControlChange"),
            single_property_schema("ControlChange", u8_schema(0, 127))
        ])
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, EnumIter, JsonSchema)]
pub enum Temperament { // 0-7
    Equal,
    JustMajor,
    JustMinor,
    Pythagorean,
    Kirnberger,
    MeanTone,
    Werckmeister,
    Arabic
}

impl From<u8> for Temperament {
    fn from(value: u8) -> Self {
        Self::iter().nth(value as usize).unwrap()
    }
}

impl Into<u8> for Temperament {
    fn into(self) -> u8 {
        Self::iter().position(|s| s == self).unwrap() as u8
    }
}

impl Default for Temperament {
    fn default() -> Self {
        Self::from(0)
    }
}
