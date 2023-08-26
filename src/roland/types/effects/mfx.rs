use schemars::JsonSchema;
use validator::{Validate, ValidationErrors};

use crate::{roland::types::numeric::{Parameter, OffsetU8}, json::{validation::{unused_by_rd300nx_err, validate_boxed_array, merge_all_fixed}, serialize_default_terminated_array}};
use crate::json::validation::valid_boxed_elements;
use super::{UnusedParameters, Parameters, discrete::{LogFrequency, QFactor}};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum MfxType { // 0-255
    Thru(UnusedParameters<32>),
    Equalizer(EqualizerParameters),
    Spectrum(UnusedParameters<32>),
    Isolator(UnusedParameters<32>),
    LowBoost(UnusedParameters<32>),
    SuperFilter(UnusedParameters<32>),
    StepFilter(UnusedParameters<32>),
    Enhancer(UnusedParameters<32>),
    AutoWah(UnusedParameters<32>),
    Humanizer(UnusedParameters<32>),
    SpeakerSimulator(UnusedParameters<32>),
    Phaser(UnusedParameters<32>),
    StepPhaser(UnusedParameters<32>),
    MultiStagePhaser(UnusedParameters<32>),
    InfinitePhaser(UnusedParameters<32>),
    RingModulator(UnusedParameters<32>),
    StepRingModulator(UnusedParameters<32>),
    Tremolo(UnusedParameters<32>),
    AutoPan(UnusedParameters<32>),
    StepPan(UnusedParameters<32>),
    Slicer(UnusedParameters<32>),
    Rotary(UnusedParameters<32>),
    VkRotary(UnusedParameters<32>),
    Chorus(UnusedParameters<32>),
    Flanger(UnusedParameters<32>),
    StepFlanger(UnusedParameters<32>),
    HexaChorus(UnusedParameters<32>),
    TremoloChorus(UnusedParameters<32>),
    SpaceD(UnusedParameters<32>),
    Chorus3D(UnusedParameters<32>),
    Flanger3D(UnusedParameters<32>),
    StepFlanger3D(UnusedParameters<32>),
    TwoBandChorus(UnusedParameters<32>),
    TwoBandFlanger(UnusedParameters<32>),
    TwoBandStepFlanger(UnusedParameters<32>),
    Overdrive(UnusedParameters<32>),
    Distortion(UnusedParameters<32>),
    VsOverdrive(UnusedParameters<32>),
    VsDistortion(UnusedParameters<32>),
    GuitarAmpSimulator(UnusedParameters<32>),
    Compressor(UnusedParameters<32>),
    Limiter(UnusedParameters<32>),
    Gate(UnusedParameters<32>),
    Delay(UnusedParameters<32>),
    LongDelay(UnusedParameters<32>),
    SerialDelay(UnusedParameters<32>),
    ModulationDelay(UnusedParameters<32>),
    ThreeTapPanDelay(UnusedParameters<32>),
    FourTapPanDelay(UnusedParameters<32>),
    MultiTapDelay(UnusedParameters<32>),
    ReverseDelay(UnusedParameters<32>),
    ShuffleDelay(UnusedParameters<32>),
    Delay3D(UnusedParameters<32>),
    TimeCtrlDelay(UnusedParameters<32>),
    LongTimeCtrlDelay(UnusedParameters<32>),
    TapeEcho(UnusedParameters<32>),
    LofiNoise(UnusedParameters<32>),
    LofiCompress(UnusedParameters<32>),
    LofiRadio(UnusedParameters<32>),
    Telephone(UnusedParameters<32>),
    Photograph(UnusedParameters<32>),
    PitchShifter(UnusedParameters<32>),
    TwoVoicePitchShifter(UnusedParameters<32>),
    StepPitchShifter(UnusedParameters<32>),
    Reverb(UnusedParameters<32>),
    GatedReverb(UnusedParameters<32>),
    ChorusOverdrive(UnusedParameters<32>),
    OverdriveFlanger(UnusedParameters<32>),
    OverdriveDelay(UnusedParameters<32>),
    DistortionChorus(UnusedParameters<32>),
    DistortionFlanger(UnusedParameters<32>),
    DistortionDelay(UnusedParameters<32>),
    EnhancerChorus(UnusedParameters<32>),
    EnhancerFlanger(UnusedParameters<32>),
    EnhancerDelay(UnusedParameters<32>),
    ChorusDelay(UnusedParameters<32>),
    FlangerDelay(UnusedParameters<32>),
    ChorusFlanger(UnusedParameters<32>),
    UnusedVrChorus(UnusedParameters<32>), //RD700NX only
    UnusedVrTremolo(UnusedParameters<32>), //RD700NX only
    UnusedVrAutoWah(UnusedParameters<32>), //RD700NX only
    UnusedVrPhaser(UnusedParameters<32>), //RD700NX only
    UnusedOrganMulti(UnusedParameters<32>), //RD700NX only
    UnusedLinedrive(UnusedParameters<32>), //RD700NX only
    UnusedSmallPhaser(UnusedParameters<32>), //RD700NX only
    SympatheticResonance(UnusedParameters<32>), //RD300NX only
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

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct OtherMfxParameters {
    #[validate(range(min = 86))]
    mfx_number: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 32>")]
    unknown: Box<[Parameter; 32]>
}

// similar to Parameters<32> but can't implement from because of the mfx_number
impl OtherMfxParameters {
    fn parameters(&self) -> [Parameter; 32] {
        *self.unknown
    }
}

impl Validate for OtherMfxParameters {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut r = Ok(());
        // technically this should validate that mfx_number is >= 86, but in practise it won't matter
        r = merge_all_fixed(r, "unknown", validate_boxed_array(&self.unknown));
        r
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
pub struct EqualizerParameters {
    low_freq: LogFrequency<20, 400>,
    low_gain: OffsetU8<15, 0, 30>,
    mid1_freq: LogFrequency<200, 8000>,
    mid1_gain: OffsetU8<15, 0, 30>,
    mid1_q: QFactor,
    mid2_freq: LogFrequency<200, 8000>,
    mid2_gain: OffsetU8<15, 0, 30>,
    mid2_q: QFactor,
    high_freq: LogFrequency<2000, 16000>,
    high_gain: OffsetU8<15, 0, 30>,
    #[validate(range(max = 127))]
    level: u8,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 21>")]
    #[validate(custom = "valid_boxed_elements")]
    unused_parameters: Box<[Parameter; 21]>
}

impl From<[Parameter; 32]> for EqualizerParameters {
    fn from(value: [Parameter; 32]) -> Self {
        let mut p = value.into_iter();
        Self {
            low_freq: p.next().unwrap().into(),
            low_gain: (p.next().unwrap().0 as u8).into(),
            mid1_freq: p.next().unwrap().into(),
            mid1_gain: (p.next().unwrap().0 as u8).into(),
            mid1_q: p.next().unwrap().into(),
            mid2_freq: p.next().unwrap().into(),
            mid2_gain: (p.next().unwrap().0 as u8).into(),
            mid2_q: p.next().unwrap().into(),
            high_freq: p.next().unwrap().into(),
            high_gain: (p.next().unwrap().0 as u8).into(),
            level: p.next().unwrap().0 as u8,
            unused_parameters: Box::new(p.collect::<Vec<_>>().try_into().unwrap())
        }
    }
}

impl Parameters<32> for EqualizerParameters {
    fn parameters(&self) -> [Parameter; 32] {
        let mut p: Vec<Parameter> = Vec::new();
        p.push(self.low_freq.into());
        p.push(Parameter(Into::<u8>::into(self.low_gain) as i16));
        p.push(self.mid1_freq.into());
        p.push(Parameter(Into::<u8>::into(self.mid1_gain) as i16));
        p.push(self.mid1_q.into());
        p.push(self.mid2_freq.into());
        p.push(Parameter(Into::<u8>::into(self.mid2_gain) as i16));
        p.push(self.mid2_q.into());
        p.push(self.high_freq.into());
        p.push(Parameter(Into::<u8>::into(self.high_gain) as i16));
        p.push(Parameter(self.level as i16));
        for unused_parameter in self.unused_parameters.iter() {
            p.push(*unused_parameter);
        }
        p.try_into().unwrap()
    }
}

impl Default for EqualizerParameters {
    fn default() -> Self {
        Self {
            low_freq: LogFrequency(200),
            low_gain: OffsetU8::default(),
            mid1_freq: LogFrequency(1000),
            mid1_gain: OffsetU8::default(),
            mid1_q: QFactor(0.5),
            mid2_freq: LogFrequency(2000),
            mid2_gain: OffsetU8::default(),
            mid2_q: QFactor(0.5),
            high_freq: LogFrequency(4000),
            high_gain: OffsetU8::default(),
            level: 127,
            unused_parameters: Default::default()
        }
    }
}
