use schemars::JsonSchema;
use validator::Validate;

use crate::json::serialize_default_terminated_array;
use crate::json::validation::unused_by_rd300nx_err;

use crate::roland::types::enums::Pan;
use crate::roland::types::numeric::Parameter;
use super::{UnusedParameters, Parameters};
use super::discrete::{LogFrequency, QFactor, FineFrequency, LinearFrequency, FilterSlope, EvenPercent, StepLinearFrequency};
use super::parameters::{Level, Switch, Gain, UInt, Int, BoostGain, BoostWidth, RateMode, SuperFilterType, Wave, NoteLength, SimpleFilterType, Direction, Phase, Vowel, SpeakerType, PhaserMode, PhaserPolarity};

//TODO validate all fields of all Parameters types
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MfxType { // 0-255
    Thru(UnusedParameters<32>),
    Equalizer(EqualizerParameters),
    Spectrum(SpectrumParameters),
    Isolator(IsolatorParameters),
    LowBoost(LowBoostParameters),
    SuperFilter(SuperFilterParameters),
    StepFilter(StepFilterParameters),
    Enhancer(EnhancerParameters),
    AutoWah(AutoWahParameters),
    Humanizer(HumanizerParameters),
    SpeakerSimulator(SpeakerSimulatorParameters),
    Phaser(PhaserParameters),
    StepPhaser(StepPhaserParameters),
    MultiStagePhaser(UnusedParameters<32>), //TODO implement parameters
    InfinitePhaser(UnusedParameters<32>), //TODO implement parameters
    RingModulator(UnusedParameters<32>), //TODO implement parameters
    StepRingModulator(UnusedParameters<32>), //TODO implement parameters
    Tremolo(UnusedParameters<32>), //TODO implement parameters
    AutoPan(UnusedParameters<32>), //TODO implement parameters
    StepPan(UnusedParameters<32>), //TODO implement parameters
    Slicer(UnusedParameters<32>), //TODO implement parameters
    Rotary(UnusedParameters<32>), //TODO implement parameters
    VkRotary(UnusedParameters<32>), //TODO implement parameters
    Chorus(UnusedParameters<32>), //TODO implement parameters
    Flanger(UnusedParameters<32>), //TODO implement parameters
    StepFlanger(UnusedParameters<32>), //TODO implement parameters
    HexaChorus(UnusedParameters<32>), //TODO implement parameters
    TremoloChorus(UnusedParameters<32>), //TODO implement parameters
    SpaceD(UnusedParameters<32>), //TODO implement parameters
    Chorus3D(UnusedParameters<32>), //TODO implement parameters
    Flanger3D(UnusedParameters<32>), //TODO implement parameters
    StepFlanger3D(UnusedParameters<32>), //TODO implement parameters
    TwoBandChorus(UnusedParameters<32>), //TODO implement parameters
    TwoBandFlanger(UnusedParameters<32>), //TODO implement parameters
    TwoBandStepFlanger(UnusedParameters<32>), //TODO implement parameters
    Overdrive(UnusedParameters<32>), //TODO implement parameters
    Distortion(UnusedParameters<32>), //TODO implement parameters
    VsOverdrive(UnusedParameters<32>), //TODO implement parameters
    VsDistortion(UnusedParameters<32>), //TODO implement parameters
    GuitarAmpSimulator(UnusedParameters<32>), //TODO implement parameters
    Compressor(UnusedParameters<32>), //TODO implement parameters
    Limiter(UnusedParameters<32>), //TODO implement parameters
    Gate(UnusedParameters<32>), //TODO implement parameters
    Delay(UnusedParameters<32>), //TODO implement parameters
    LongDelay(UnusedParameters<32>), //TODO implement parameters
    SerialDelay(UnusedParameters<32>), //TODO implement parameters
    ModulationDelay(UnusedParameters<32>), //TODO implement parameters
    ThreeTapPanDelay(UnusedParameters<32>), //TODO implement parameters
    FourTapPanDelay(UnusedParameters<32>), //TODO implement parameters
    MultiTapDelay(UnusedParameters<32>), //TODO implement parameters
    ReverseDelay(UnusedParameters<32>), //TODO implement parameters
    ShuffleDelay(UnusedParameters<32>), //TODO implement parameters
    Delay3D(UnusedParameters<32>), //TODO implement parameters
    TimeCtrlDelay(UnusedParameters<32>), //TODO implement parameters
    LongTimeCtrlDelay(UnusedParameters<32>), //TODO implement parameters
    TapeEcho(UnusedParameters<32>), //TODO implement parameters
    LofiNoise(UnusedParameters<32>), //TODO implement parameters
    LofiCompress(UnusedParameters<32>), //TODO implement parameters
    LofiRadio(UnusedParameters<32>), //TODO implement parameters
    Telephone(UnusedParameters<32>), //TODO implement parameters
    Photograph(UnusedParameters<32>), //TODO implement parameters
    PitchShifter(UnusedParameters<32>), //TODO implement parameters
    TwoVoicePitchShifter(UnusedParameters<32>), //TODO implement parameters
    StepPitchShifter(UnusedParameters<32>), //TODO implement parameters
    Reverb(UnusedParameters<32>), //TODO implement parameters
    GatedReverb(UnusedParameters<32>), //TODO implement parameters
    ChorusOverdrive(UnusedParameters<32>), //TODO implement parameters
    OverdriveFlanger(UnusedParameters<32>), //TODO implement parameters
    OverdriveDelay(UnusedParameters<32>), //TODO implement parameters
    DistortionChorus(UnusedParameters<32>), //TODO implement parameters
    DistortionFlanger(UnusedParameters<32>), //TODO implement parameters
    DistortionDelay(UnusedParameters<32>), //TODO implement parameters
    EnhancerChorus(UnusedParameters<32>), //TODO implement parameters
    EnhancerFlanger(UnusedParameters<32>), //TODO implement parameters
    EnhancerDelay(UnusedParameters<32>), //TODO implement parameters
    ChorusDelay(UnusedParameters<32>), //TODO implement parameters
    FlangerDelay(UnusedParameters<32>), //TODO implement parameters
    ChorusFlanger(UnusedParameters<32>), //TODO implement parameters
    UnusedVrChorus(UnusedParameters<32>), //RD700NX only
    UnusedVrTremolo(UnusedParameters<32>), //RD700NX only
    UnusedVrAutoWah(UnusedParameters<32>), //RD700NX only
    UnusedVrPhaser(UnusedParameters<32>), //RD700NX only
    UnusedOrganMulti(UnusedParameters<32>), //RD700NX only
    UnusedLinedrive(UnusedParameters<32>), //RD700NX only
    UnusedSmallPhaser(UnusedParameters<32>), //RD700NX only
    SympatheticResonance(UnusedParameters<32>), //TODO implement parameters //RD300NX only
    Other(OtherMfxParameters)
}

impl MfxType {
    pub fn from(number: u8, parameters: [Parameter; 32]) -> Self {
        match number {
            0 => Self::Thru(parameters.into()),
            1 => Self::Equalizer(parameters.into()),
            2 => Self::Spectrum(parameters.into()),
            3 => Self::Isolator(parameters.into()),
            4 => Self::LowBoost(parameters.into()),
            5 => Self::SuperFilter(parameters.into()),
            6 => Self::StepFilter(parameters.into()),
            7 => Self::Enhancer(parameters.into()),
            8 => Self::AutoWah(parameters.into()),
            9 => Self::Humanizer(parameters.into()),
            10 => Self::SpeakerSimulator(parameters.into()),
            11 => Self::Phaser(parameters.into()),
            12 => Self::StepPhaser(parameters.into()),
            13 => Self::MultiStagePhaser(parameters.into()),
            14 => Self::InfinitePhaser(parameters.into()),
            15 => Self::RingModulator(parameters.into()),
            16 => Self::StepRingModulator(parameters.into()),
            17 => Self::Tremolo(parameters.into()),
            18 => Self::AutoPan(parameters.into()),
            19 => Self::StepPan(parameters.into()),
            20 => Self::Slicer(parameters.into()),
            21 => Self::Rotary(parameters.into()),
            22 => Self::VkRotary(parameters.into()),
            23 => Self::Chorus(parameters.into()),
            24 => Self::Flanger(parameters.into()),
            25 => Self::StepFlanger(parameters.into()),
            26 => Self::HexaChorus(parameters.into()),
            27 => Self::TremoloChorus(parameters.into()),
            28 => Self::SpaceD(parameters.into()),
            29 => Self::Chorus3D(parameters.into()),
            30 => Self::Flanger3D(parameters.into()),
            31 => Self::StepFlanger3D(parameters.into()),
            32 => Self::TwoBandChorus(parameters.into()),
            33 => Self::TwoBandFlanger(parameters.into()),
            34 => Self::TwoBandStepFlanger(parameters.into()),
            35 => Self::Overdrive(parameters.into()),
            36 => Self::Distortion(parameters.into()),
            37 => Self::VsOverdrive(parameters.into()),
            38 => Self::VsDistortion(parameters.into()),
            39 => Self::GuitarAmpSimulator(parameters.into()),
            40 => Self::Compressor(parameters.into()),
            41 => Self::Limiter(parameters.into()),
            42 => Self::Gate(parameters.into()),
            43 => Self::Delay(parameters.into()),
            44 => Self::LongDelay(parameters.into()),
            45 => Self::SerialDelay(parameters.into()),
            46 => Self::ModulationDelay(parameters.into()),
            47 => Self::ThreeTapPanDelay(parameters.into()),
            48 => Self::FourTapPanDelay(parameters.into()),
            49 => Self::MultiTapDelay(parameters.into()),
            50 => Self::ReverseDelay(parameters.into()),
            51 => Self::ShuffleDelay(parameters.into()),
            52 => Self::Delay3D(parameters.into()),
            53 => Self::TimeCtrlDelay(parameters.into()),
            54 => Self::LongTimeCtrlDelay(parameters.into()),
            55 => Self::TapeEcho(parameters.into()),
            56 => Self::LofiNoise(parameters.into()),
            57 => Self::LofiCompress(parameters.into()),
            58 => Self::LofiRadio(parameters.into()),
            59 => Self::Telephone(parameters.into()),
            60 => Self::Photograph(parameters.into()),
            61 => Self::PitchShifter(parameters.into()),
            62 => Self::TwoVoicePitchShifter(parameters.into()),
            63 => Self::StepPitchShifter(parameters.into()),
            64 => Self::Reverb(parameters.into()),
            65 => Self::GatedReverb(parameters.into()),
            66 => Self::ChorusOverdrive(parameters.into()),
            67 => Self::OverdriveFlanger(parameters.into()),
            68 => Self::OverdriveDelay(parameters.into()),
            69 => Self::DistortionChorus(parameters.into()),
            70 => Self::DistortionFlanger(parameters.into()),
            71 => Self::DistortionDelay(parameters.into()),
            72 => Self::EnhancerChorus(parameters.into()),
            73 => Self::EnhancerFlanger(parameters.into()),
            74 => Self::EnhancerDelay(parameters.into()),
            75 => Self::ChorusDelay(parameters.into()),
            76 => Self::FlangerDelay(parameters.into()),
            77 => Self::ChorusFlanger(parameters.into()),
            78 => Self::UnusedVrChorus(parameters.into()),
            79 => Self::UnusedVrTremolo(parameters.into()),
            80 => Self::UnusedVrAutoWah(parameters.into()),
            81 => Self::UnusedVrPhaser(parameters.into()),
            82 => Self::UnusedOrganMulti(parameters.into()),
            83 => Self::UnusedLinedrive(parameters.into()),
            84 => Self::UnusedSmallPhaser(parameters.into()),
            85 => Self::SympatheticResonance(parameters.into()),
            mfx_number => Self::Other(OtherMfxParameters { mfx_number, unknown: parameters.into() })
        }
    }

    pub fn number(&self) -> u8 {
        match self {
            Self::Thru(_) => 0,
            Self::Equalizer(_) => 1,
            Self::Spectrum(_) => 2,
            Self::Isolator(_) => 3,
            Self::LowBoost(_) => 4,
            Self::SuperFilter(_) => 5,
            Self::StepFilter(_) => 6,
            Self::Enhancer(_) => 7,
            Self::AutoWah(_) => 8,
            Self::Humanizer(_) => 9,
            Self::SpeakerSimulator(_) => 10,
            Self::Phaser(_) => 11,
            Self::StepPhaser(_) => 12,
            Self::MultiStagePhaser(_) => 13,
            Self::InfinitePhaser(_) => 14,
            Self::RingModulator(_) => 15,
            Self::StepRingModulator(_) => 16,
            Self::Tremolo(_) => 17,
            Self::AutoPan(_) => 18,
            Self::StepPan(_) => 19,
            Self::Slicer(_) => 20,
            Self::Rotary(_) => 21,
            Self::VkRotary(_) => 22,
            Self::Chorus(_) => 23,
            Self::Flanger(_) => 24,
            Self::StepFlanger(_) => 25,
            Self::HexaChorus(_) => 26,
            Self::TremoloChorus(_) => 27,
            Self::SpaceD(_) => 28,
            Self::Chorus3D(_) => 29,
            Self::Flanger3D(_) => 30,
            Self::StepFlanger3D(_) => 31,
            Self::TwoBandChorus(_) => 32,
            Self::TwoBandFlanger(_) => 33,
            Self::TwoBandStepFlanger(_) => 34,
            Self::Overdrive(_) => 35,
            Self::Distortion(_) => 36,
            Self::VsOverdrive(_) => 37,
            Self::VsDistortion(_) => 38,
            Self::GuitarAmpSimulator(_) => 39,
            Self::Compressor(_) => 40,
            Self::Limiter(_) => 41,
            Self::Gate(_) => 42,
            Self::Delay(_) => 43,
            Self::LongDelay(_) => 44,
            Self::SerialDelay(_) => 45,
            Self::ModulationDelay(_) => 46,
            Self::ThreeTapPanDelay(_) => 47,
            Self::FourTapPanDelay(_) => 48,
            Self::MultiTapDelay(_) => 49,
            Self::ReverseDelay(_) => 50,
            Self::ShuffleDelay(_) => 51,
            Self::Delay3D(_) => 52,
            Self::TimeCtrlDelay(_) => 53,
            Self::LongTimeCtrlDelay(_) => 54,
            Self::TapeEcho(_) => 55,
            Self::LofiNoise(_) => 56,
            Self::LofiCompress(_) => 57,
            Self::LofiRadio(_) => 58,
            Self::Telephone(_) => 59,
            Self::Photograph(_) => 60,
            Self::PitchShifter(_) => 61,
            Self::TwoVoicePitchShifter(_) => 62,
            Self::StepPitchShifter(_) => 63,
            Self::Reverb(_) => 64,
            Self::GatedReverb(_) => 65,
            Self::ChorusOverdrive(_) => 66,
            Self::OverdriveFlanger(_) => 67,
            Self::OverdriveDelay(_) => 68,
            Self::DistortionChorus(_) => 69,
            Self::DistortionFlanger(_) => 70,
            Self::DistortionDelay(_) => 71,
            Self::EnhancerChorus(_) => 72,
            Self::EnhancerFlanger(_) => 73,
            Self::EnhancerDelay(_) => 74,
            Self::ChorusDelay(_) => 75,
            Self::FlangerDelay(_) => 76,
            Self::ChorusFlanger(_) => 77,
            Self::UnusedVrChorus(_) => 78,
            Self::UnusedVrTremolo(_) => 79,
            Self::UnusedVrAutoWah(_) => 80,
            Self::UnusedVrPhaser(_) => 81,
            Self::UnusedOrganMulti(_) => 82,
            Self::UnusedLinedrive(_) => 83,
            Self::UnusedSmallPhaser(_) => 84,
            Self::SympatheticResonance(_) => 85,
            Self::Other(o) => o.mfx_number
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Thru(_) => "Thru".into(),
            Self::Equalizer(_) => "Equalizer".into(),
            Self::Spectrum(_) => "Spectrum".into(),
            Self::Isolator(_) => "Isolator".into(),
            Self::LowBoost(_) => "LowBoost".into(),
            Self::SuperFilter(_) => "SuperFilter".into(),
            Self::StepFilter(_) => "StepFilter".into(),
            Self::Enhancer(_) => "Enhancer".into(),
            Self::AutoWah(_) => "AutoWah".into(),
            Self::Humanizer(_) => "Humanizer".into(),
            Self::SpeakerSimulator(_) => "SpeakerSimulator".into(),
            Self::Phaser(_) => "Phaser".into(),
            Self::StepPhaser(_) => "StepPhaser".into(),
            Self::MultiStagePhaser(_) => "MultiStagePhaser".into(),
            Self::InfinitePhaser(_) => "InfinitePhaser".into(),
            Self::RingModulator(_) => "RingModulator".into(),
            Self::StepRingModulator(_) => "StepRingModulator".into(),
            Self::Tremolo(_) => "Tremolo".into(),
            Self::AutoPan(_) => "AutoPan".into(),
            Self::StepPan(_) => "StepPan".into(),
            Self::Slicer(_) => "Slicer".into(),
            Self::Rotary(_) => "Rotary".into(),
            Self::VkRotary(_) => "VkRotary".into(),
            Self::Chorus(_) => "Chorus".into(),
            Self::Flanger(_) => "Flanger".into(),
            Self::StepFlanger(_) => "StepFlanger".into(),
            Self::HexaChorus(_) => "HexaChorus".into(),
            Self::TremoloChorus(_) => "TremoloChorus".into(),
            Self::SpaceD(_) => "SpaceD".into(),
            Self::Chorus3D(_) => "Chorus3D".into(),
            Self::Flanger3D(_) => "Flanger3D".into(),
            Self::StepFlanger3D(_) => "StepFlanger3D".into(),
            Self::TwoBandChorus(_) => "TwoBandChorus".into(),
            Self::TwoBandFlanger(_) => "TwoBandFlanger".into(),
            Self::TwoBandStepFlanger(_) => "TwoBandStepFlanger".into(),
            Self::Overdrive(_) => "Overdrive".into(),
            Self::Distortion(_) => "Distortion".into(),
            Self::VsOverdrive(_) => "VsOverdrive".into(),
            Self::VsDistortion(_) => "VsDistortion".into(),
            Self::GuitarAmpSimulator(_) => "GuitarAmpSimulator".into(),
            Self::Compressor(_) => "Compressor".into(),
            Self::Limiter(_) => "Limiter".into(),
            Self::Gate(_) => "Gate".into(),
            Self::Delay(_) => "Delay".into(),
            Self::LongDelay(_) => "LongDelay".into(),
            Self::SerialDelay(_) => "SerialDelay".into(),
            Self::ModulationDelay(_) => "ModulationDelay".into(),
            Self::ThreeTapPanDelay(_) => "ThreeTapPanDelay".into(),
            Self::FourTapPanDelay(_) => "FourTapPanDelay".into(),
            Self::MultiTapDelay(_) => "MultiTapDelay".into(),
            Self::ReverseDelay(_) => "ReverseDelay".into(),
            Self::ShuffleDelay(_) => "ShuffleDelay".into(),
            Self::Delay3D(_) => "Delay3D".into(),
            Self::TimeCtrlDelay(_) => "TimeCtrlDelay".into(),
            Self::LongTimeCtrlDelay(_) => "LongTimeCtrlDelay".into(),
            Self::TapeEcho(_) => "TapeEcho".into(),
            Self::LofiNoise(_) => "LofiNoise".into(),
            Self::LofiCompress(_) => "LofiCompress".into(),
            Self::LofiRadio(_) => "LofiRadio".into(),
            Self::Telephone(_) => "Telephone".into(),
            Self::Photograph(_) => "Photograph".into(),
            Self::PitchShifter(_) => "PitchShifter".into(),
            Self::TwoVoicePitchShifter(_) => "TwoVoicePitchShifter".into(),
            Self::StepPitchShifter(_) => "StepPitchShifter".into(),
            Self::Reverb(_) => "Reverb".into(),
            Self::GatedReverb(_) => "GatedReverb".into(),
            Self::ChorusOverdrive(_) => "ChorusOverdrive".into(),
            Self::OverdriveFlanger(_) => "OverdriveFlanger".into(),
            Self::OverdriveDelay(_) => "OverdriveDelay".into(),
            Self::DistortionChorus(_) => "DistortionChorus".into(),
            Self::DistortionFlanger(_) => "DistortionFlanger".into(),
            Self::DistortionDelay(_) => "DistortionDelay".into(),
            Self::EnhancerChorus(_) => "EnhancerChorus".into(),
            Self::EnhancerFlanger(_) => "EnhancerFlanger".into(),
            Self::EnhancerDelay(_) => "EnhancerDelay".into(),
            Self::ChorusDelay(_) => "ChorusDelay".into(),
            Self::FlangerDelay(_) => "FlangerDelay".into(),
            Self::ChorusFlanger(_) => "ChorusFlanger".into(),
            Self::UnusedVrChorus(_) => "UnusedVrChorus".into(),
            Self::UnusedVrTremolo(_) => "UnusedVrTremolo".into(),
            Self::UnusedVrAutoWah(_) => "UnusedVrAutoWah".into(),
            Self::UnusedVrPhaser(_) => "UnusedVrPhaser".into(),
            Self::UnusedOrganMulti(_) => "UnusedOrganMulti".into(),
            Self::UnusedLinedrive(_) => "UnusedLinedrive".into(),
            Self::UnusedSmallPhaser(_) => "UnusedSmallPhaser".into(),
            Self::SympatheticResonance(_) => "SympatheticResonance".into(),
            Self::Other(o) => format!("Other({})", o.mfx_number)
        }
    }
    
    pub fn parameters(&self) -> [Parameter; 32] {
        match self {
            Self::Thru(p) => p.parameters(),
            Self::Equalizer(p) => p.parameters(),
            Self::Spectrum(p) => p.parameters(),
            Self::Isolator(p) => p.parameters(),
            Self::LowBoost(p) => p.parameters(),
            Self::SuperFilter(p) => p.parameters(),
            Self::StepFilter(p) => p.parameters(),
            Self::Enhancer(p) => p.parameters(),
            Self::AutoWah(p) => p.parameters(),
            Self::Humanizer(p) => p.parameters(),
            Self::SpeakerSimulator(p) => p.parameters(),
            Self::Phaser(p) => p.parameters(),
            Self::StepPhaser(p) => p.parameters(),
            Self::MultiStagePhaser(p) => p.parameters(),
            Self::InfinitePhaser(p) => p.parameters(),
            Self::RingModulator(p) => p.parameters(),
            Self::StepRingModulator(p) => p.parameters(),
            Self::Tremolo(p) => p.parameters(),
            Self::AutoPan(p) => p.parameters(),
            Self::StepPan(p) => p.parameters(),
            Self::Slicer(p) => p.parameters(),
            Self::Rotary(p) => p.parameters(),
            Self::VkRotary(p) => p.parameters(),
            Self::Chorus(p) => p.parameters(),
            Self::Flanger(p) => p.parameters(),
            Self::StepFlanger(p) => p.parameters(),
            Self::HexaChorus(p) => p.parameters(),
            Self::TremoloChorus(p) => p.parameters(),
            Self::SpaceD(p) => p.parameters(),
            Self::Chorus3D(p) => p.parameters(),
            Self::Flanger3D(p) => p.parameters(),
            Self::StepFlanger3D(p) => p.parameters(),
            Self::TwoBandChorus(p) => p.parameters(),
            Self::TwoBandFlanger(p) => p.parameters(),
            Self::TwoBandStepFlanger(p) => p.parameters(),
            Self::Overdrive(p) => p.parameters(),
            Self::Distortion(p) => p.parameters(),
            Self::VsOverdrive(p) => p.parameters(),
            Self::VsDistortion(p) => p.parameters(),
            Self::GuitarAmpSimulator(p) => p.parameters(),
            Self::Compressor(p) => p.parameters(),
            Self::Limiter(p) => p.parameters(),
            Self::Gate(p) => p.parameters(),
            Self::Delay(p) => p.parameters(),
            Self::LongDelay(p) => p.parameters(),
            Self::SerialDelay(p) => p.parameters(),
            Self::ModulationDelay(p) => p.parameters(),
            Self::ThreeTapPanDelay(p) => p.parameters(),
            Self::FourTapPanDelay(p) => p.parameters(),
            Self::MultiTapDelay(p) => p.parameters(),
            Self::ReverseDelay(p) => p.parameters(),
            Self::ShuffleDelay(p) => p.parameters(),
            Self::Delay3D(p) => p.parameters(),
            Self::TimeCtrlDelay(p) => p.parameters(),
            Self::LongTimeCtrlDelay(p) => p.parameters(),
            Self::TapeEcho(p) => p.parameters(),
            Self::LofiNoise(p) => p.parameters(),
            Self::LofiCompress(p) => p.parameters(),
            Self::LofiRadio(p) => p.parameters(),
            Self::Telephone(p) => p.parameters(),
            Self::Photograph(p) => p.parameters(),
            Self::PitchShifter(p) => p.parameters(),
            Self::TwoVoicePitchShifter(p) => p.parameters(),
            Self::StepPitchShifter(p) => p.parameters(),
            Self::Reverb(p) => p.parameters(),
            Self::GatedReverb(p) => p.parameters(),
            Self::ChorusOverdrive(p) => p.parameters(),
            Self::OverdriveFlanger(p) => p.parameters(),
            Self::OverdriveDelay(p) => p.parameters(),
            Self::DistortionChorus(p) => p.parameters(),
            Self::DistortionFlanger(p) => p.parameters(),
            Self::DistortionDelay(p) => p.parameters(),
            Self::EnhancerChorus(p) => p.parameters(),
            Self::EnhancerFlanger(p) => p.parameters(),
            Self::EnhancerDelay(p) => p.parameters(),
            Self::ChorusDelay(p) => p.parameters(),
            Self::FlangerDelay(p) => p.parameters(),
            Self::ChorusFlanger(p) => p.parameters(),
            Self::UnusedVrChorus(p) => p.parameters(),
            Self::UnusedVrTremolo(p) => p.parameters(),
            Self::UnusedVrAutoWah(p) => p.parameters(),
            Self::UnusedVrPhaser(p) => p.parameters(),
            Self::UnusedOrganMulti(p) => p.parameters(),
            Self::UnusedLinedrive(p) => p.parameters(),
            Self::UnusedSmallPhaser(p) => p.parameters(),
            Self::SympatheticResonance(p) => p.parameters(),
            Self::Other(p) => p.parameters()
        }
    }

    pub fn is_off(&self) -> bool {
        match self {
            Self::Thru(_) => true,
            _ => false
        }
    }
}

impl Default for MfxType {
    fn default() -> Self {
        Self::from(0, [Parameter::default(); 32])
    }
}

impl Validate for MfxType {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Self::Thru(p) => p.validate(),
            Self::Equalizer(p) => p.validate(),
            Self::Spectrum(p) => p.validate(),
            Self::Isolator(p) => p.validate(),
            Self::LowBoost(p) => p.validate(),
            Self::SuperFilter(p) => p.validate(),
            Self::StepFilter(p) => p.validate(),
            Self::Enhancer(p) => p.validate(),
            Self::AutoWah(p) => p.validate(),
            Self::Humanizer(p) => p.validate(),
            Self::SpeakerSimulator(p) => p.validate(),
            Self::Phaser(p) => p.validate(),
            Self::StepPhaser(p) => p.validate(),
            Self::MultiStagePhaser(p) => p.validate(),
            Self::InfinitePhaser(p) => p.validate(),
            Self::RingModulator(p) => p.validate(),
            Self::StepRingModulator(p) => p.validate(),
            Self::Tremolo(p) => p.validate(),
            Self::AutoPan(p) => p.validate(),
            Self::StepPan(p) => p.validate(),
            Self::Slicer(p) => p.validate(),
            Self::Rotary(p) => p.validate(),
            Self::VkRotary(p) => p.validate(),
            Self::Chorus(p) => p.validate(),
            Self::Flanger(p) => p.validate(),
            Self::StepFlanger(p) => p.validate(),
            Self::HexaChorus(p) => p.validate(),
            Self::TremoloChorus(p) => p.validate(),
            Self::SpaceD(p) => p.validate(),
            Self::Chorus3D(p) => p.validate(),
            Self::Flanger3D(p) => p.validate(),
            Self::StepFlanger3D(p) => p.validate(),
            Self::TwoBandChorus(p) => p.validate(),
            Self::TwoBandFlanger(p) => p.validate(),
            Self::TwoBandStepFlanger(p) => p.validate(),
            Self::Overdrive(p) => p.validate(),
            Self::Distortion(p) => p.validate(),
            Self::VsOverdrive(p) => p.validate(),
            Self::VsDistortion(p) => p.validate(),
            Self::GuitarAmpSimulator(p) => p.validate(),
            Self::Compressor(p) => p.validate(),
            Self::Limiter(p) => p.validate(),
            Self::Gate(p) => p.validate(),
            Self::Delay(p) => p.validate(),
            Self::LongDelay(p) => p.validate(),
            Self::SerialDelay(p) => p.validate(),
            Self::ModulationDelay(p) => p.validate(),
            Self::ThreeTapPanDelay(p) => p.validate(),
            Self::FourTapPanDelay(p) => p.validate(),
            Self::MultiTapDelay(p) => p.validate(),
            Self::ReverseDelay(p) => p.validate(),
            Self::ShuffleDelay(p) => p.validate(),
            Self::Delay3D(p) => p.validate(),
            Self::TimeCtrlDelay(p) => p.validate(),
            Self::LongTimeCtrlDelay(p) => p.validate(),
            Self::TapeEcho(p) => p.validate(),
            Self::LofiNoise(p) => p.validate(),
            Self::LofiCompress(p) => p.validate(),
            Self::LofiRadio(p) => p.validate(),
            Self::Telephone(p) => p.validate(),
            Self::Photograph(p) => p.validate(),
            Self::PitchShifter(p) => p.validate(),
            Self::TwoVoicePitchShifter(p) => p.validate(),
            Self::StepPitchShifter(p) => p.validate(),
            Self::Reverb(p) => p.validate(),
            Self::GatedReverb(p) => p.validate(),
            Self::ChorusOverdrive(p) => p.validate(),
            Self::OverdriveFlanger(p) => p.validate(),
            Self::OverdriveDelay(p) => p.validate(),
            Self::DistortionChorus(p) => p.validate(),
            Self::DistortionFlanger(p) => p.validate(),
            Self::DistortionDelay(p) => p.validate(),
            Self::EnhancerChorus(p) => p.validate(),
            Self::EnhancerFlanger(p) => p.validate(),
            Self::EnhancerDelay(p) => p.validate(),
            Self::ChorusDelay(p) => p.validate(),
            Self::FlangerDelay(p) => p.validate(),
            Self::ChorusFlanger(p) => p.validate(),
            Self::UnusedVrChorus(_) => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedVrTremolo(_) => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedVrAutoWah(_) => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedVrPhaser(_) => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedOrganMulti(_) => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedLinedrive(_) => Err(unused_by_rd300nx_err("0", self)),
            Self::UnusedSmallPhaser(_) => Err(unused_by_rd300nx_err("0", self)),
            Self::SympatheticResonance(p) => p.validate(),
            Self::Other(p) => p.validate()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct OtherMfxParameters {
    #[validate(range(min = 86))]
    mfx_number: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 32>")]
    #[validate]
    unknown: [Parameter; 32]
}

// similar to Parameters<32> but can't implement from because of the mfx_number
impl OtherMfxParameters {
    fn parameters(&self) -> [Parameter; 32] {
        self.unknown
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct EqualizerParameters {
    low_freq: LogFrequency<20, 400>,
    low_gain: Gain,
    mid1_freq: LogFrequency<200, 8000>,
    mid1_gain: Gain,
    mid1_q: QFactor,
    mid2_freq: LogFrequency<200, 8000>,
    mid2_gain: Gain,
    mid2_q: QFactor,
    high_freq: LogFrequency<2000, 16000>,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 21>")]
    #[validate]
    unused_parameters: [Parameter; 21]
}

impl Default for EqualizerParameters {
    fn default() -> Self {
        Self {
            low_freq: LogFrequency(200),
            low_gain: Int(0),
            mid1_freq: LogFrequency(1000),
            mid1_gain: Int(0),
            mid1_q: QFactor(0.5),
            mid2_freq: LogFrequency(2000),
            mid2_gain: Int(0),
            mid2_q: QFactor(0.5),
            high_freq: LogFrequency(4000),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct SpectrumParameters {
    band1_250hz: Gain,
    band2_500hz: Gain,
    band3_1000hz: Gain,
    band4_1250hz: Gain,
    band5_2000hz: Gain,
    band6_3150hz: Gain,
    band7_4000hz: Gain,
    band8_8000hz: Gain,
    q: QFactor,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl Default for SpectrumParameters {
    fn default() -> Self {
        Self {
            band1_250hz: Int(0),
            band2_500hz: Int(0),
            band3_1000hz: Int(0),
            band4_1250hz: Int(0),
            band5_2000hz: Int(0),
            band6_3150hz: Int(0),
            band7_4000hz: Int(0),
            band8_8000hz: Int(0),
            q: QFactor(0.5),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct IsolatorParameters {
    boost_cut_low: BoostGain,
    boost_cut_mid: BoostGain,
    boost_cut_high: BoostGain,
    a_phase_low_sw: Switch,
    a_phase_low_level: Level,
    a_phase_mid_sw: Switch,
    a_phase_mid_level: Level,
    low_boost_sw: Switch,
    low_boost_level: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl Default for IsolatorParameters {
    fn default() -> Self {
        Self {
            boost_cut_low: Int(0),
            boost_cut_mid: Int(0),
            boost_cut_high: Int(0),
            a_phase_low_sw: Switch(false),
            a_phase_low_level: UInt(127),
            a_phase_mid_sw: Switch(false),
            a_phase_mid_level: UInt(127),
            low_boost_sw: Switch(false),
            low_boost_level: UInt(127),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct LowBoostParameters {
    boost_freq: FineFrequency,
    boost_gain: Int<0, 12>,
    boost_width: BoostWidth,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 26>")]
    #[validate]
    unused_parameters: [Parameter; 26]
}

impl Default for LowBoostParameters {
    fn default() -> Self {
        Self {
            boost_freq: FineFrequency(80),
            boost_gain: Int(6),
            boost_width: BoostWidth::Wide,
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct SuperFilterParameters {
    filter_type: SuperFilterType,
    filter_slope: FilterSlope,
    filter_cutoff: Level,
    filter_resonance: Level,
    filter_gain: Int<0, 12>,
    modulation_sw: Switch,
    modulation_wave: Wave,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    attack: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 19>")]
    #[validate]
    unused_parameters: [Parameter; 19]
}

impl Default for SuperFilterParameters {
    fn default() -> Self {
        Self {
            filter_type: SuperFilterType::HighPassFilter,
            filter_slope: FilterSlope(-36),
            filter_cutoff: UInt(30),
            filter_resonance: UInt(40),
            filter_gain: Int(0),
            modulation_sw: Switch(false),
            modulation_wave: Wave::Triangle,
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(40),
            attack: UInt(50),
            level: UInt(127), //TODO confirm on keyboard
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct StepFilterParameters {
    step1: Level,
    step2: Level,
    step3: Level,
    step4: Level,
    step5: Level,
    step6: Level,
    step7: Level,
    step8: Level,
    step9: Level,
    step10: Level,
    step11: Level,
    step12: Level,
    step13: Level,
    step14: Level,
    step15: Level,
    step16: Level,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    attack: Level,
    filter_type: SuperFilterType,
    filter_slope: FilterSlope,
    filter_resonance: Level,
    filter_gain: Int<0, 12>,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 7>")]
    #[validate]
    unused_parameters: [Parameter; 7]
}

impl Default for StepFilterParameters {
    fn default() -> Self {
        Self {
            step1: UInt(60),
            step2: UInt(30),
            step3: UInt(60),
            step4: UInt(30),
            step5: UInt(60),
            step6: UInt(30),
            step7: UInt(60),
            step8: UInt(30),
            step9: UInt(60),
            step10: UInt(60),
            step11: UInt(30),
            step12: UInt(60),
            step13: UInt(60),
            step14: UInt(30),
            step15: UInt(60),
            step16: UInt(30),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            attack: UInt(50),
            filter_type: SuperFilterType::HighPassFilter,
            filter_slope: FilterSlope(-36),
            filter_resonance: UInt(40),
            filter_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct EnhancerParameters {
    sensitivity: Level,
    mix: Level,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 27>")]
    #[validate]
    unused_parameters: [Parameter; 27]
}

impl Default for EnhancerParameters {
    fn default() -> Self {
        Self {
            sensitivity: UInt(64),
            mix: UInt(64),
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct AutoWahParameters {
    filter_type: SimpleFilterType,
    manual: Level,
    peak: Level,
    sensitivity: Level,
    polarity: Direction,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 19>")]
    #[validate]
    unused_parameters: [Parameter; 19]
}

impl Default for AutoWahParameters {
    fn default() -> Self {
        Self {
            filter_type: SimpleFilterType::BandPassFilter,
            manual: UInt(60),
            peak: UInt(40),
            sensitivity: UInt(0),
            polarity: Direction::Up,
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(2.0),
            rate_note: NoteLength::QuarterNote,
            depth: UInt(60),
            phase: UInt(0),
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(100),
            unused_parameters: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct HumanizerParameters {
    drive_sw: Switch,
    drive: Level,
    vowel1: Vowel,
    vowel2: Vowel,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    input_sync_sw: Switch,
    input_sync_threshold: Level,
    manual: Level,
    low_gain: Gain,
    high_gain: Gain,
    pan: Pan,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 17>")]
    #[validate]
    unused_parameters: [Parameter; 17]
}

impl Default for HumanizerParameters {
    fn default() -> Self {
        Self {
            drive_sw: Switch(true),
            drive: UInt(127),
            vowel1: Vowel::U,
            vowel2: Vowel::A,
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::HalfNote,
            depth: UInt(127),
            input_sync_sw: Switch(false),
            input_sync_threshold: UInt(60),
            manual: UInt(50),
            low_gain: Int(0),
            high_gain: Int(0),
            pan: Pan::Centre,
            level: UInt(100),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct SpeakerSimulatorParameters {
    speaker: SpeakerType,
    mic_setting: Int<1, 3>,
    mic_level: Level,
    direct_level: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 27>")]
    #[validate]
    unused_parameters: [Parameter; 27]
}

impl Default for SpeakerSimulatorParameters {
    fn default() -> Self {
        Self {
            speaker: SpeakerType::BuiltIn2,
            mic_setting: Int(2),
            mic_level: UInt(127),
            direct_level: UInt(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct PhaserParameters {
    mode: PhaserMode,
    manual: Level,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    polarity: PhaserPolarity,
    resonance: Level,
    cross_feedback: EvenPercent,
    mix: Level,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 19>")]
    #[validate]
    unused_parameters: [Parameter; 19]
}

impl Default for PhaserParameters {
    fn default() -> Self {
        Self {
            mode: PhaserMode::TwelveStage,
            manual: UInt(64),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::DoubleNote,
            depth: UInt(40),
            polarity: PhaserPolarity::Synchro,
            resonance: UInt(40),
            cross_feedback: EvenPercent(0),
            mix: UInt(127),
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct StepPhaserParameters {
    mode: PhaserMode,
    manual: Level,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    polarity: PhaserPolarity,
    resonance: Level,
    cross_feedback: EvenPercent,
    step_rate_mode: RateMode,
    step_rate_hz: StepLinearFrequency,
    step_rate_note: NoteLength,
    mix: Level,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 16>")]
    #[validate]
    unused_parameters: [Parameter; 16]
}

impl Default for StepPhaserParameters {
    fn default() -> Self {
        Self {
            mode: PhaserMode::TwelveStage,
            manual: UInt(64),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(1.5),
            rate_note: NoteLength::DoubleNoteTriplet,
            depth: UInt(40),
            polarity: PhaserPolarity::Synchro,
            resonance: UInt(40),
            cross_feedback: EvenPercent(0),
            step_rate_mode: RateMode::Note,
            step_rate_hz: StepLinearFrequency(8.0),
            step_rate_note: NoteLength::SixteenthNote,
            mix: UInt(127),
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}