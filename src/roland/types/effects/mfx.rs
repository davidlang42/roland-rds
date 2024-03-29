use schemars::JsonSchema;
use validator::Validate;

use crate::json::schema::{single_property_schema_of, one_of_schema};
use crate::json::{serialize_default_terminated_array, type_name_pretty};
use crate::json::validation::unused_by_rd300nx_err;

use crate::roland::types::enums::Pan;
use crate::roland::types::numeric::Parameter;
use super::{UnusedParameters, Parameters};
use super::discrete::{LogFrequency, QFactor, FineFrequency, LinearFrequency, FilterSlope, EvenPercent, StepLinearFrequency, Balance, LogMilliseconds, LogFrequencyOrByPass, HumFrequency, Feedback, ByPassOrLogFrequency, LogFrequencyOrByPassOffByOne, Phase, GateTime};
use super::parameters::{Level, Switch, Gain, UInt, Int, BoostGain, BoostWidth, RateMode, SuperFilterType, Wave, NoteLength, SimpleFilterType, Direction, Vowel, SpeakerType, PhaserMode, PhaserPolarity, MultiPhaserMode, ModWave, SlicerMode, Speed, FilterType, OutputMode, AmpType, MicSetting, PreAmpType, PreAmpGain, CompressionRatio, PostGain, GateMode, DelayMode, LinearMilliseconds, PhaseType, FeedbackMode, TapeHeads, LofiType, NoiseType, DiscType, DiscTypeWithRandom, Semitones, ReverbOnlyCharacter, GateType};

#[derive(Serialize, Deserialize, Debug)]
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
    MultiStagePhaser(MultiStagePhaserParameters),
    InfinitePhaser(InfinitePhaserParameters),
    RingModulator(RingModulatorParameters),
    StepRingModulator(StepRingModulatorParameters),
    Tremolo(CyclicalParameters),
    AutoPan(CyclicalParameters),
    StepPan(StepPanParameters),
    Slicer(SlicerParameters),
    Rotary(RotaryParameters),
    VkRotary(VkRotaryParameters),
    Chorus(ChorusParameters),
    Flanger(FlangerParameters),
    StepFlanger(StepFlangerParameters),
    HexaChorus(HexaChorusParameters),
    TremoloChorus(TremoloChorusParameters),
    SpaceD(SpaceDParameters),
    Chorus3D(Chorus3DParameters),
    Flanger3D(Flanger3DParameters),
    StepFlanger3D(StepFlanger3DParameters),
    TwoBandChorus(TwoBandChorusParameters),
    TwoBandFlanger(TwoBandFlangerParameters),
    TwoBandStepFlanger(TwoBandStepFlangerParameters),
    Overdrive(DriveParameters<0, 70>), // 0=AmpType::Small
    Distortion(DriveParameters<3, 50>), // 3=AmpType::ThreeStack
    VsOverdrive(VsDriveParameters<0>), // 0=AmpType::Small
    VsDistortion(VsDriveParameters<3>), // 3=AmpType::ThreeStack
    GuitarAmpSimulator(GuitarAmpSimulatorParameters),
    Compressor(CompressorParameters),
    Limiter(LimiterParameters),
    Gate(GateParameters),
    Delay(DelayParameters),
    LongDelay(LongDelayParameters),
    SerialDelay(SerialDelayParameters),
    ModulationDelay(ModulationDelayParameters),
    ThreeTapPanDelay(ThreeTapPanDelayParameters),
    FourTapPanDelay(FourTapPanDelayParameters),
    MultiTapDelay(MultiTapDelayParameters),
    ReverseDelay(ReverseDelayParameters),
    ShuffleDelay(ShuffleDelayParameters),
    Delay3D(Delay3DParameters),
    TimeCtrlDelay(TimeCtrlDelayParameters),
    LongTimeCtrlDelay(LongTimeCtrlDelayParameters),
    TapeEcho(TapeEchoParameters),
    LofiNoise(LofiNoiseParameters),
    LofiCompress(LofiCompressParameters),
    LofiRadio(LofiRadioParameters),
    Telephone(TelephoneParameters),
    Phonograph(PhonographParameters),
    PitchShifter(PitchShifterParameters),
    TwoVoicePitchShifter(TwoVoicePitchShifterParameters),
    StepPitchShifter(StepPitchShifterParameters),
    Reverb(ReverbParameters),
    GatedReverb(GatedReverbParameters),
    OverdriveChorus(DriveChorusParameters<64, 80>),
    OverdriveFlanger(DriveFlangerParameters<64>),
    OverdriveDelay(DriveDelayParameters<64>),
    DistortionChorus(DriveChorusParameters<127, 70>),
    DistortionFlanger(DriveFlangerParameters<127>),
    DistortionDelay(DriveDelayParameters<127>),
    EnhancerChorus(EnhancerChorusParameters),
    EnhancerFlanger(EnhancerFlangerParameters),
    EnhancerDelay(EnhancerDelayParameters),
    ChorusDelay(ChorusDelayParameters),
    FlangerDelay(FlangerDelayParameters),
    ChorusFlanger(ChorusFlangerParameters),
    UnusedVrChorus(UnusedParameters<32>), //RD700NX only
    UnusedVrTremolo(UnusedParameters<32>), //RD700NX only
    UnusedVrAutoWah(UnusedParameters<32>), //RD700NX only
    UnusedVrPhaser(UnusedParameters<32>), //RD700NX only
    UnusedOrganMulti(UnusedParameters<32>), //RD700NX only
    UnusedLinedrive(UnusedParameters<32>), //RD700NX only
    UnusedSmallPhaser(UnusedParameters<32>), //RD700NX only
    SympatheticResonance(SympatheticResonanceParameters), //RD300NX only
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
            60 => Self::Phonograph(parameters.into()),
            61 => Self::PitchShifter(parameters.into()),
            62 => Self::TwoVoicePitchShifter(parameters.into()),
            63 => Self::StepPitchShifter(parameters.into()),
            64 => Self::Reverb(parameters.into()),
            65 => Self::GatedReverb(parameters.into()),
            66 => Self::OverdriveChorus(parameters.into()),
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
            Self::Phonograph(_) => 60,
            Self::PitchShifter(_) => 61,
            Self::TwoVoicePitchShifter(_) => 62,
            Self::StepPitchShifter(_) => 63,
            Self::Reverb(_) => 64,
            Self::GatedReverb(_) => 65,
            Self::OverdriveChorus(_) => 66,
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
            Self::Phonograph(_) => "Photograph".into(),
            Self::PitchShifter(_) => "PitchShifter".into(),
            Self::TwoVoicePitchShifter(_) => "TwoVoicePitchShifter".into(),
            Self::StepPitchShifter(_) => "StepPitchShifter".into(),
            Self::Reverb(_) => "Reverb".into(),
            Self::GatedReverb(_) => "GatedReverb".into(),
            Self::OverdriveChorus(_) => "ChorusOverdrive".into(),
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
            Self::Phonograph(p) => p.parameters(),
            Self::PitchShifter(p) => p.parameters(),
            Self::TwoVoicePitchShifter(p) => p.parameters(),
            Self::StepPitchShifter(p) => p.parameters(),
            Self::Reverb(p) => p.parameters(),
            Self::GatedReverb(p) => p.parameters(),
            Self::OverdriveChorus(p) => p.parameters(),
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

    #[allow(dead_code)] // used by tests, potentially useful if using this as a library
    pub fn default(&self) -> Self {
        match self {
            Self::Thru(_) => Self::Thru(Default::default()),
            Self::Equalizer(_) => Self::Equalizer(Default::default()),
            Self::Spectrum(_) => Self::Spectrum(Default::default()),
            Self::Isolator(_) => Self::Isolator(Default::default()),
            Self::LowBoost(_) => Self::LowBoost(Default::default()),
            Self::SuperFilter(_) => Self::SuperFilter(Default::default()),
            Self::StepFilter(_) => Self::StepFilter(Default::default()),
            Self::Enhancer(_) => Self::Enhancer(Default::default()),
            Self::AutoWah(_) => Self::AutoWah(Default::default()),
            Self::Humanizer(_) => Self::Humanizer(Default::default()),
            Self::SpeakerSimulator(_) => Self::SpeakerSimulator(Default::default()),
            Self::Phaser(_) => Self::Phaser(Default::default()),
            Self::StepPhaser(_) => Self::StepPhaser(Default::default()),
            Self::MultiStagePhaser(_) => Self::MultiStagePhaser(Default::default()),
            Self::InfinitePhaser(_) => Self::InfinitePhaser(Default::default()),
            Self::RingModulator(_) => Self::RingModulator(Default::default()),
            Self::StepRingModulator(_) => Self::StepRingModulator(Default::default()),
            Self::Tremolo(_) => Self::Tremolo(Default::default()),
            Self::AutoPan(_) => Self::AutoPan(Default::default()),
            Self::StepPan(_) => Self::StepPan(Default::default()),
            Self::Slicer(_) => Self::Slicer(Default::default()),
            Self::Rotary(_) => Self::Rotary(Default::default()),
            Self::VkRotary(_) => Self::VkRotary(Default::default()),
            Self::Chorus(_) => Self::Chorus(Default::default()),
            Self::Flanger(_) => Self::Flanger(Default::default()),
            Self::StepFlanger(_) => Self::StepFlanger(Default::default()),
            Self::HexaChorus(_) => Self::HexaChorus(Default::default()),
            Self::TremoloChorus(_) => Self::TremoloChorus(Default::default()),
            Self::SpaceD(_) => Self::SpaceD(Default::default()),
            Self::Chorus3D(_) => Self::Chorus3D(Default::default()),
            Self::Flanger3D(_) => Self::Flanger3D(Default::default()),
            Self::StepFlanger3D(_) => Self::StepFlanger3D(Default::default()),
            Self::TwoBandChorus(_) => Self::TwoBandChorus(Default::default()),
            Self::TwoBandFlanger(_) => Self::TwoBandFlanger(Default::default()),
            Self::TwoBandStepFlanger(_) => Self::TwoBandStepFlanger(Default::default()),
            Self::Overdrive(_) => Self::Overdrive(Default::default()),
            Self::Distortion(_) => Self::Distortion(Default::default()),
            Self::VsOverdrive(_) => Self::VsOverdrive(Default::default()),
            Self::VsDistortion(_) => Self::VsDistortion(Default::default()),
            Self::GuitarAmpSimulator(_) => Self::GuitarAmpSimulator(Default::default()),
            Self::Compressor(_) => Self::Compressor(Default::default()),
            Self::Limiter(_) => Self::Limiter(Default::default()),
            Self::Gate(_) => Self::Gate(Default::default()),
            Self::Delay(_) => Self::Delay(Default::default()),
            Self::LongDelay(_) => Self::LongDelay(Default::default()),
            Self::SerialDelay(_) => Self::SerialDelay(Default::default()),
            Self::ModulationDelay(_) => Self::ModulationDelay(Default::default()),
            Self::ThreeTapPanDelay(_) => Self::ThreeTapPanDelay(Default::default()),
            Self::FourTapPanDelay(_) => Self::FourTapPanDelay(Default::default()),
            Self::MultiTapDelay(_) => Self::MultiTapDelay(Default::default()),
            Self::ReverseDelay(_) => Self::ReverseDelay(Default::default()),
            Self::ShuffleDelay(_) => Self::ShuffleDelay(Default::default()),
            Self::Delay3D(_) => Self::Delay3D(Default::default()),
            Self::TimeCtrlDelay(_) => Self::TimeCtrlDelay(Default::default()),
            Self::LongTimeCtrlDelay(_) => Self::LongTimeCtrlDelay(Default::default()),
            Self::TapeEcho(_) => Self::TapeEcho(Default::default()),
            Self::LofiNoise(_) => Self::LofiNoise(Default::default()),
            Self::LofiCompress(_) => Self::LofiCompress(Default::default()),
            Self::LofiRadio(_) => Self::LofiRadio(Default::default()),
            Self::Telephone(_) => Self::Telephone(Default::default()),
            Self::Phonograph(_) => Self::Phonograph(Default::default()),
            Self::PitchShifter(_) => Self::PitchShifter(Default::default()),
            Self::TwoVoicePitchShifter(_) => Self::TwoVoicePitchShifter(Default::default()),
            Self::StepPitchShifter(_) => Self::StepPitchShifter(Default::default()),
            Self::Reverb(_) => Self::Reverb(Default::default()),
            Self::GatedReverb(_) => Self::GatedReverb(Default::default()),
            Self::OverdriveChorus(_) => Self::OverdriveChorus(Default::default()),
            Self::OverdriveFlanger(_) => Self::OverdriveFlanger(Default::default()),
            Self::OverdriveDelay(_) => Self::OverdriveDelay(Default::default()),
            Self::DistortionChorus(_) => Self::DistortionChorus(Default::default()),
            Self::DistortionFlanger(_) => Self::DistortionFlanger(Default::default()),
            Self::DistortionDelay(_) => Self::DistortionDelay(Default::default()),
            Self::EnhancerChorus(_) => Self::EnhancerChorus(Default::default()),
            Self::EnhancerFlanger(_) => Self::EnhancerFlanger(Default::default()),
            Self::EnhancerDelay(_) => Self::EnhancerDelay(Default::default()),
            Self::ChorusDelay(_) => Self::ChorusDelay(Default::default()),
            Self::FlangerDelay(_) => Self::FlangerDelay(Default::default()),
            Self::ChorusFlanger(_) => Self::ChorusFlanger(Default::default()),
            Self::UnusedVrChorus(_) => Self::UnusedVrChorus(Default::default()),
            Self::UnusedVrTremolo(_) => Self::UnusedVrTremolo(Default::default()),
            Self::UnusedVrAutoWah(_) => Self::UnusedVrAutoWah(Default::default()),
            Self::UnusedVrPhaser(_) => Self::UnusedVrPhaser(Default::default()),
            Self::UnusedOrganMulti(_) => Self::UnusedOrganMulti(Default::default()),
            Self::UnusedLinedrive(_) => Self::UnusedLinedrive(Default::default()),
            Self::UnusedSmallPhaser(_) => Self::UnusedSmallPhaser(Default::default()),
            Self::SympatheticResonance(_) => Self::SympatheticResonance(Default::default()),
            Self::Other(p) => Self::Other(OtherMfxParameters { mfx_number: p.mfx_number, unknown: [Default::default(); 32] })
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
            Self::Phonograph(p) => p.validate(),
            Self::PitchShifter(p) => p.validate(),
            Self::TwoVoicePitchShifter(p) => p.validate(),
            Self::StepPitchShifter(p) => p.validate(),
            Self::Reverb(p) => p.validate(),
            Self::GatedReverb(p) => p.validate(),
            Self::OverdriveChorus(p) => p.validate(),
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

impl JsonSchema for MfxType {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        one_of_schema(vec![
            single_property_schema_of::<UnusedParameters<32>>("Thru", gen),
            single_property_schema_of::<EqualizerParameters>("Equalizer", gen),
            single_property_schema_of::<SpectrumParameters>("Spectrum", gen),
            single_property_schema_of::<IsolatorParameters>("Isolator", gen),
            single_property_schema_of::<LowBoostParameters>("LowBoost", gen),
            single_property_schema_of::<SuperFilterParameters>("SuperFilter", gen),
            single_property_schema_of::<StepFilterParameters>("StepFilter", gen),
            single_property_schema_of::<EnhancerParameters>("Enhancer", gen),
            single_property_schema_of::<AutoWahParameters>("AutoWah", gen),
            single_property_schema_of::<HumanizerParameters>("Humanizer", gen),
            single_property_schema_of::<SpeakerSimulatorParameters>("SpeakerSimulator", gen),
            single_property_schema_of::<PhaserParameters>("Phaser", gen),
            single_property_schema_of::<StepPhaserParameters>("StepPhaser", gen),
            single_property_schema_of::<MultiStagePhaserParameters>("MultiStagePhaser", gen),
            single_property_schema_of::<InfinitePhaserParameters>("InfinitePhaser", gen),
            single_property_schema_of::<RingModulatorParameters>("RingModulator", gen),
            single_property_schema_of::<StepRingModulatorParameters>("StepRingModulator", gen),
            single_property_schema_of::<CyclicalParameters>("Tremolo", gen),
            single_property_schema_of::<CyclicalParameters>("AutoPan", gen),
            single_property_schema_of::<StepPanParameters>("StepPan", gen),
            single_property_schema_of::<SlicerParameters>("Slicer", gen),
            single_property_schema_of::<RotaryParameters>("Rotary", gen),
            single_property_schema_of::<VkRotaryParameters>("VkRotary", gen),
            single_property_schema_of::<ChorusParameters>("Chorus", gen),
            single_property_schema_of::<FlangerParameters>("Flanger", gen),
            single_property_schema_of::<StepFlangerParameters>("StepFlanger", gen),
            single_property_schema_of::<HexaChorusParameters>("HexaChorus", gen),
            single_property_schema_of::<TremoloChorusParameters>("TremoloChorus", gen),
            single_property_schema_of::<SpaceDParameters>("SpaceD", gen),
            single_property_schema_of::<Chorus3DParameters>("Chorus3D", gen),
            single_property_schema_of::<Flanger3DParameters>("Flanger3D", gen),
            single_property_schema_of::<StepFlanger3DParameters>("StepFlanger3D", gen),
            single_property_schema_of::<TwoBandChorusParameters>("TwoBandChorus", gen),
            single_property_schema_of::<TwoBandFlangerParameters>("TwoBandFlanger", gen),
            single_property_schema_of::<TwoBandStepFlangerParameters>("TwoBandStepFlanger", gen),
            single_property_schema_of::<DriveParameters<0, 70>>("Overdrive", gen),
            single_property_schema_of::<DriveParameters<3, 50>>("Distortion", gen),
            single_property_schema_of::<VsDriveParameters<0>>("VsOverdrive", gen),
            single_property_schema_of::<VsDriveParameters<3>>("VsDistortion", gen),
            single_property_schema_of::<GuitarAmpSimulatorParameters>("GuitarAmpSimulator", gen),
            single_property_schema_of::<CompressorParameters>("Compressor", gen),
            single_property_schema_of::<LimiterParameters>("Limiter", gen),
            single_property_schema_of::<GateParameters>("Gate", gen),
            single_property_schema_of::<DelayParameters>("Delay", gen),
            single_property_schema_of::<LongDelayParameters>("LongDelay", gen),
            single_property_schema_of::<SerialDelayParameters>("SerialDelay", gen),
            single_property_schema_of::<ModulationDelayParameters>("ModulationDelay", gen),
            single_property_schema_of::<ThreeTapPanDelayParameters>("ThreeTapPanDelay", gen),
            single_property_schema_of::<FourTapPanDelayParameters>("FourTapPanDelay", gen),
            single_property_schema_of::<MultiTapDelayParameters>("MultiTapDelay", gen),
            single_property_schema_of::<ReverseDelayParameters>("ReverseDelay", gen),
            single_property_schema_of::<ShuffleDelayParameters>("ShuffleDelay", gen),
            single_property_schema_of::<Delay3DParameters>("Delay3D", gen),
            single_property_schema_of::<TimeCtrlDelayParameters>("TimeCtrlDelay", gen),
            single_property_schema_of::<LongTimeCtrlDelayParameters>("LongTimeCtrlDelay", gen),
            single_property_schema_of::<TapeEchoParameters>("TapeEcho", gen),
            single_property_schema_of::<LofiNoiseParameters>("LofiNoise", gen),
            single_property_schema_of::<LofiCompressParameters>("LofiCompress", gen),
            single_property_schema_of::<LofiRadioParameters>("LofiRadio", gen),
            single_property_schema_of::<TelephoneParameters>("Telephone", gen),
            single_property_schema_of::<PhonographParameters>("Phonograph", gen),
            single_property_schema_of::<PitchShifterParameters>("PitchShifter", gen),
            single_property_schema_of::<TwoVoicePitchShifterParameters>("TwoVoicePitchShifter", gen),
            single_property_schema_of::<StepPitchShifterParameters>("StepPitchShifter", gen),
            single_property_schema_of::<ReverbParameters>("Reverb", gen),
            single_property_schema_of::<GatedReverbParameters>("GatedReverb", gen),
            single_property_schema_of::<DriveChorusParameters<64, 80>>("OverdriveChorus", gen),
            single_property_schema_of::<DriveFlangerParameters<64>>("OverdriveFlanger", gen),
            single_property_schema_of::<DriveDelayParameters<64>>("OverdriveDelay", gen),
            single_property_schema_of::<DriveChorusParameters<127, 70>>("DistortionChorus", gen),
            single_property_schema_of::<DriveFlangerParameters<127>>("DistortionFlanger", gen),
            single_property_schema_of::<DriveDelayParameters<127>>("DistortionDelay", gen),
            single_property_schema_of::<EnhancerChorusParameters>("EnhancerChorus", gen),
            single_property_schema_of::<EnhancerFlangerParameters>("EnhancerFlanger", gen),
            single_property_schema_of::<EnhancerDelayParameters>("EnhancerDelay", gen),
            single_property_schema_of::<ChorusDelayParameters>("ChorusDelay", gen),
            single_property_schema_of::<FlangerDelayParameters>("FlangerDelay", gen),
            single_property_schema_of::<ChorusFlangerParameters>("ChorusFlanger", gen),
            single_property_schema_of::<UnusedParameters<32>>("UnusedVrChorus", gen),
            single_property_schema_of::<UnusedParameters<32>>("UnusedVrTremolo", gen),
            single_property_schema_of::<UnusedParameters<32>>("UnusedVrAutoWah", gen),
            single_property_schema_of::<UnusedParameters<32>>("UnusedVrPhaser", gen),
            single_property_schema_of::<UnusedParameters<32>>("UnusedOrganMulti", gen),
            single_property_schema_of::<UnusedParameters<32>>("UnusedLinedrive", gen),
            single_property_schema_of::<UnusedParameters<32>>("UnusedSmallPhaser", gen),
            single_property_schema_of::<SympatheticResonanceParameters>("SympatheticResonance", gen),
            single_property_schema_of::<OtherMfxParameters>("Other", gen)
        ])
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

impl Default for OtherMfxParameters {
    fn default() -> Self {
        Self {
            mfx_number: 86,
            unknown: Default::default()
        }
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
            low_boost_level: UInt(64),
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
            level: UInt(127),
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
            phase: Phase(0),
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
    mic_setting: MicSetting,
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
            speaker: SpeakerType::BuiltIn3,
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
    cross_feedback: Feedback,
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
            rate_note: NoteLength::WholeNote,
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
    cross_feedback: Feedback,
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
            rate_note: NoteLength::HalfNoteTriplet,
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

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct MultiStagePhaserParameters {
    mode: MultiPhaserMode,
    manual: Level,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    resonance: Level,
    mix: Level,
    pan: Pan,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 20>")]
    #[validate]
    unused_parameters: [Parameter; 20]
}

impl Default for MultiStagePhaserParameters {
    fn default() -> Self {
        Self {
            mode: MultiPhaserMode::TwentyFourStage,
            manual: UInt(60),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(40),
            resonance: UInt(40),
            mix: UInt(127),
            pan: Pan::Centre,
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct InfinitePhaserParameters {
    mode: Int<1, 4>,
    speed: Int<-100, 100>,
    resonance: Level,
    mix: Level,
    pan: Pan,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 24>")]
    #[validate]
    unused_parameters: [Parameter; 24]
}

impl Default for InfinitePhaserParameters {
    fn default() -> Self {
        Self {
            mode: Int(4),
            speed: Int(40),
            resonance: UInt(80),
            mix: UInt(127),
            pan: Pan::Centre,
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct RingModulatorParameters {
    frequency: Level,
    sensitivity: Level,
    polarity: Direction,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 25>")]
    #[validate]
    unused_parameters: [Parameter; 25]
}

impl Default for RingModulatorParameters {
    fn default() -> Self {
        Self {
            frequency: UInt(60),
            sensitivity: UInt(0),
            polarity: Direction::Up,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct StepRingModulatorParameters {
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
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 8>")]
    #[validate]
    unused_parameters: [Parameter; 8]
}

impl Default for StepRingModulatorParameters {
    fn default() -> Self {
        Self {
            step1: UInt(120),
            step2: UInt(60),
            step3: UInt(120),
            step4: UInt(60),
            step5: UInt(90),
            step6: UInt(90),
            step7: UInt(60),
            step8: UInt(90),
            step9: UInt(60),
            step10: UInt(90),
            step11: UInt(60),
            step12: UInt(90),
            step13: UInt(60),
            step14: UInt(60),
            step15: UInt(90),
            step16: UInt(60),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            attack: UInt(50),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct CyclicalParameters {
    mod_wave: ModWave,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 24>")]
    #[validate]
    unused_parameters: [Parameter; 24]
}

impl Default for CyclicalParameters {
    fn default() -> Self {
        Self {
            mod_wave: ModWave::Triangle,
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(4.0),
            rate_note: NoteLength::EighthNote,
            depth: UInt(96),
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct StepPanParameters {
    step1: Pan,
    step2: Pan,
    step3: Pan,
    step4: Pan,
    step5: Pan,
    step6: Pan,
    step7: Pan,
    step8: Pan,
    step9: Pan,
    step10: Pan,
    step11: Pan,
    step12: Pan,
    step13: Pan,
    step14: Pan,
    step15: Pan,
    step16: Pan,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    attack: Level,
    input_sync_sw: Switch,
    input_sync_threshold: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 9>")]
    #[validate]
    unused_parameters: [Parameter; 9]
}

impl Default for StepPanParameters {
    fn default() -> Self {
        Self {
            step1: Pan::Centre,
            step2: Pan::Left(64),
            step3: Pan::Centre,
            step4: Pan::Left(64),
            step5: Pan::Centre,
            step6: Pan::Right(63),
            step7: Pan::Centre,
            step8: Pan::Right(63),
            step9: Pan::Centre,
            step10: Pan::Left(64),
            step11: Pan::Right(63),
            step12: Pan::Left(64),
            step13: Pan::Centre,
            step14: Pan::Right(63),
            step15: Pan::Left(64),
            step16: Pan::Right(63),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.25),
            rate_note: NoteLength::DoubleNote,
            attack: UInt(50),
            input_sync_sw: Switch(false),
            input_sync_threshold: UInt(60),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct SlicerParameters {
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
    input_sync_sw: Switch,
    input_sync_threshold: Level,
    mode: SlicerMode,
    shuffle: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 7>")]
    #[validate]
    unused_parameters: [Parameter; 7]
}

impl Default for SlicerParameters {
    fn default() -> Self {
        Self {
            step1: UInt(127),
            step2: UInt(0),
            step3: UInt(30),
            step4: UInt(127),
            step5: UInt(127),
            step6: UInt(0),
            step7: UInt(30),
            step8: UInt(0),
            step9: UInt(127),
            step10: UInt(0),
            step11: UInt(30),
            step12: UInt(127),
            step13: UInt(0),
            step14: UInt(0),
            step15: UInt(30),
            step16: UInt(0),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            attack: UInt(50),
            input_sync_sw: Switch(false),
            input_sync_threshold: UInt(60),
            mode: SlicerMode::Legato,
            shuffle: UInt(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct RotaryParameters {
    speed: Speed,
    woofer_slow_rate: LinearFrequency,
    woofer_fast_rate: LinearFrequency,
    woofer_accel: UInt<0, 15>,
    woofer_level: Level,
    tweeter_slow_rate: LinearFrequency,
    tweeter_fast_rate: LinearFrequency,
    tweeter_accel: UInt<0, 15>,
    tweeter_level: Level,
    separation: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 21>")]
    #[validate]
    unused_parameters: [Parameter; 21]
}

impl Default for RotaryParameters {
    fn default() -> Self {
        Self {
            speed: Speed::Slow,
            woofer_slow_rate: LinearFrequency(0.6),
            woofer_fast_rate: LinearFrequency(6.0),
            woofer_accel: UInt(5),
            woofer_level: UInt(127),
            tweeter_slow_rate: LinearFrequency(0.7),
            tweeter_fast_rate: LinearFrequency(7.0),
            tweeter_accel: UInt(12),
            tweeter_level: UInt(120),
            separation: UInt(80),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct VkRotaryParameters {
    speed: Speed,
    brake: Switch,
    woofer_slow_rate: LinearFrequency,
    woofer_fast_rate: LinearFrequency,
    woofer_trans_up: Level,
    woofer_trans_down: Level,
    woofer_level: Level,
    tweeter_slow_rate: LinearFrequency,
    tweeter_fast_rate: LinearFrequency,
    tweeter_trans_up: Level,
    tweeter_trans_down: Level,
    tweeter_level: Level,
    spread: UInt<0, 10>,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 16>")]
    #[validate]
    unused_parameters: [Parameter; 16]
}

impl Default for VkRotaryParameters {
    fn default() -> Self {
        Self {
            speed: Speed::Slow,
            brake: Switch(false),
            woofer_slow_rate: LinearFrequency(0.6),
            woofer_fast_rate: LinearFrequency(6.0),
            woofer_trans_up: UInt(67),
            woofer_trans_down: UInt(67),
            woofer_level: UInt(127),
            tweeter_slow_rate: LinearFrequency(0.7),
            tweeter_fast_rate: LinearFrequency(7.0),
            tweeter_trans_up: UInt(97),
            tweeter_trans_down: UInt(97),
            tweeter_level: UInt(127),
            spread: UInt(10),
            low_gain: Int(0),
            high_gain: Int(6),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ChorusParameters {
    filter_type: FilterType,
    cutoff_freq: LogFrequency<200, 8000>,
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 20>")]
    #[validate]
    unused_parameters: [Parameter; 20]
}

impl Default for ChorusParameters {
    fn default() -> Self {
        Self {
            filter_type: FilterType::HighPassFilter,
            cutoff_freq: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Hertz,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(20),
            phase: Phase(180),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct FlangerParameters {
    filter_type: FilterType,
    cutoff_freq: LogFrequency<200, 8000>,
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    feedback: Feedback,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 19>")]
    #[validate]
    unused_parameters: [Parameter; 19]
}

impl Default for FlangerParameters {
    fn default() -> Self {
        Self {
            filter_type: FilterType::HighPassFilter,
            cutoff_freq: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(40),
            phase: Phase(180),
            feedback: EvenPercent(60),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct StepFlangerParameters {
    filter_type: FilterType,
    cutoff_freq: LogFrequency<200, 8000>,
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    feedback: Feedback,
    step_rate_mode: RateMode,
    step_rate_hz: StepLinearFrequency,
    step_rate_note: NoteLength,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 16>")]
    #[validate]
    unused_parameters: [Parameter; 16]
}

impl Default for StepFlangerParameters {
    fn default() -> Self {
        Self {
            filter_type: FilterType::HighPassFilter,
            cutoff_freq: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(1.5),
            rate_note: NoteLength::HalfNoteTriplet,
            depth: UInt(40),
            phase: Phase(180),
            feedback: EvenPercent(60),
            step_rate_mode: RateMode::Note,
            step_rate_hz: StepLinearFrequency(8.0),
            step_rate_note: NoteLength::SixteenthNote,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct HexaChorusParameters {
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    pre_delay_deviation: UInt<0, 20>,
    depth_deviation: Int<-20, 20>,
    pan_deviation: UInt<0, 20>,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl Default for HexaChorusParameters {
    fn default() -> Self {
        Self {
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Hertz,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(20),
            pre_delay_deviation: UInt(0),
            depth_deviation: Int(0),
            pan_deviation: UInt(20),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TremoloChorusParameters {
    pre_delay: LogMilliseconds,
    cho_rate_mode: RateMode,
    cho_rate_hz: LinearFrequency,
    cho_rate_note: NoteLength,
    cho_depth: Level,
    trem_rate_mode: RateMode,
    trem_rate_hz: LinearFrequency,
    trem_rate_note: NoteLength,
    trem_separation: Level,
    trem_phase: Phase,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 20>")]
    #[validate]
    unused_parameters: [Parameter; 20]
}

impl Default for TremoloChorusParameters {
    fn default() -> Self {
        Self {
            pre_delay: LogMilliseconds(2.0),
            cho_rate_mode: RateMode::Hertz,
            cho_rate_hz: LinearFrequency(0.5),
            cho_rate_note: NoteLength::WholeNote,
            cho_depth: UInt(50),
            trem_rate_mode: RateMode::Hertz,
            trem_rate_hz: LinearFrequency(2.0),
            trem_rate_note: NoteLength::QuarterNote,
            trem_separation: UInt(127),
            trem_phase: Phase(180),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct SpaceDParameters {
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl Default for SpaceDParameters {
    fn default() -> Self {
        Self { 
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Hertz,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(20),
            phase: Phase(180),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct Chorus3DParameters {
    filter_type: FilterType,
    cutoff_freq: LogFrequency<200, 8000>,
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    output_mode: OutputMode,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 19>")]
    #[validate]
    unused_parameters: [Parameter; 19]
}

impl Default for Chorus3DParameters {
    fn default() -> Self {
        Self {
            filter_type: FilterType::HighPassFilter,
            cutoff_freq: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Hertz,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(20),
            phase: Phase(180),
            output_mode: OutputMode::Speaker,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct Flanger3DParameters {
    filter_type: FilterType,
    cutoff_freq: LogFrequency<200, 8000>,
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    feedback: Feedback,
    output_mode: OutputMode,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 18>")]
    #[validate]
    unused_parameters: [Parameter; 18]
}

impl Default for Flanger3DParameters {
    fn default() -> Self {
        Self {
            filter_type: FilterType::HighPassFilter,
            cutoff_freq: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(40),
            phase: Phase(180),
            feedback: EvenPercent(60),
            output_mode: OutputMode::Speaker,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct StepFlanger3DParameters {
    filter_type: FilterType,
    cutoff_freq: LogFrequency<200, 8000>,
    pre_delay: LogMilliseconds,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    feedback: Feedback,
    step_rate_mode: RateMode,
    step_rate_hz: StepLinearFrequency,
    step_rate_note: NoteLength,
    output_mode: OutputMode,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate]
    unused_parameters: [Parameter; 15]
}

impl Default for StepFlanger3DParameters {
    fn default() -> Self {
        Self {
            filter_type: FilterType::HighPassFilter,
            cutoff_freq: LogFrequency(800),
            pre_delay: LogMilliseconds(2.0),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(1.5),
            rate_note: NoteLength::HalfNoteTriplet,
            depth: UInt(40),
            phase: Phase(180),
            feedback: EvenPercent(60),
            step_rate_mode: RateMode::Note,
            step_rate_hz: StepLinearFrequency(8.0),
            step_rate_note: NoteLength::SixteenthNote,
            output_mode: OutputMode::Speaker,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TwoBandChorusParameters {
    split_freq: LogFrequency<200, 8000>,
    low_pre_delay: LogMilliseconds,
    low_rate_mode: RateMode,
    low_rate_hz: LinearFrequency,
    low_rate_note: NoteLength,
    low_depth: Level,
    low_phase: Phase,
    high_pre_delay: LogMilliseconds,
    high_rate_mode: RateMode,
    high_rate_hz: LinearFrequency,
    high_rate_note: NoteLength,
    high_depth: Level,
    high_phase: Phase,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 17>")]
    #[validate]
    unused_parameters: [Parameter; 17]
}

impl Default for TwoBandChorusParameters {
    fn default() -> Self {
        Self {
            split_freq: LogFrequency(800),
            low_pre_delay: LogMilliseconds(4.0),
            low_rate_mode: RateMode::Hertz,
            low_rate_hz: LinearFrequency(0.25),
            low_rate_note: NoteLength::DoubleNote,
            low_depth: UInt(20),
            low_phase: Phase(180),
            high_pre_delay: LogMilliseconds(1.0),
            high_rate_mode: RateMode::Hertz,
            high_rate_hz: LinearFrequency(0.5),
            high_rate_note: NoteLength::WholeNote,
            high_depth: UInt(20),
            high_phase: Phase(180),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TwoBandFlangerParameters {
    split_freq: LogFrequency<200, 8000>,
    low_pre_delay: LogMilliseconds,
    low_rate_mode: RateMode,
    low_rate_hz: LinearFrequency,
    low_rate_note: NoteLength,
    low_depth: Level,
    low_phase: Phase,
    low_feedback: Feedback,
    high_pre_delay: LogMilliseconds,
    high_rate_mode: RateMode,
    high_rate_hz: LinearFrequency,
    high_rate_note: NoteLength,
    high_depth: Level,
    high_phase: Phase,
    high_feedback: Feedback,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate]
    unused_parameters: [Parameter; 15]
}

impl Default for TwoBandFlangerParameters {
    fn default() -> Self {
        Self {
            split_freq: LogFrequency(800),
            low_pre_delay: LogMilliseconds(4.0),
            low_rate_mode: RateMode::Note,
            low_rate_hz: LinearFrequency(0.25),
            low_rate_note: NoteLength::DoubleNote,
            low_depth: UInt(40),
            low_phase: Phase(180),
            low_feedback: EvenPercent(60),
            high_pre_delay: LogMilliseconds(1.0),
            high_rate_mode: RateMode::Note,
            high_rate_hz: LinearFrequency(0.5),
            high_rate_note: NoteLength::WholeNote,
            high_depth: UInt(40),
            high_phase: Phase(180),
            high_feedback: EvenPercent(40),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TwoBandStepFlangerParameters {
    split_freq: LogFrequency<200, 8000>,
    low_pre_delay: LogMilliseconds,
    low_rate_mode: RateMode,
    low_rate_hz: LinearFrequency,
    low_rate_note: NoteLength,
    low_depth: Level,
    low_phase: Phase,
    low_feedback: Feedback,
    low_step_rate_mode: RateMode,
    low_step_rate_hz: StepLinearFrequency,
    low_step_rate_note: NoteLength,
    high_pre_delay: LogMilliseconds,
    high_rate_mode: RateMode,
    high_rate_hz: LinearFrequency,
    high_rate_note: NoteLength,
    high_depth: Level,
    high_phase: Phase,
    high_feedback: Feedback,
    high_step_rate_mode: RateMode,
    high_step_rate_hz: StepLinearFrequency,
    high_step_rate_note: NoteLength,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 9>")]
    #[validate]
    unused_parameters: [Parameter; 9]
}

impl Default for TwoBandStepFlangerParameters {
    fn default() -> Self {
        Self {
            split_freq: LogFrequency(800),
            low_pre_delay: LogMilliseconds(4.0),
            low_rate_mode: RateMode::Note,
            low_rate_hz: LinearFrequency(3.0),
            low_rate_note: NoteLength::QuarterNoteTriplet,
            low_depth: UInt(40),
            low_phase: Phase(180),
            low_feedback: EvenPercent(60),
            low_step_rate_mode: RateMode::Note,
            low_step_rate_hz: StepLinearFrequency(4.0),
            low_step_rate_note: NoteLength::EighthNote,
            high_pre_delay: LogMilliseconds(1.0),
            high_rate_mode: RateMode::Note,
            high_rate_hz: LinearFrequency(1.5),
            high_rate_note: NoteLength::HalfNoteTriplet,
            high_depth: UInt(40),
            high_phase: Phase(180),
            high_feedback: EvenPercent(40),
            high_step_rate_mode: RateMode::Note,
            high_step_rate_hz: StepLinearFrequency(8.0),
            high_step_rate_note: NoteLength::SixteenthNote,
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct DriveParameters<const DEFAULT_AMP_INDEX: i16, const DEFAULT_LEVEL: u16> {
    drive: Level,
    amp_type: AmpType,
    low_gain: Gain,
    high_gain: Gain,
    pan: Pan,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 26>")]
    #[validate]
    unused_parameters: [Parameter; 26]
}

impl<const DAI: i16, const DL: u16> Default for DriveParameters<DAI, DL> {
    fn default() -> Self {
        Self {
            drive: UInt(127),
            amp_type: AmpType::from(Parameter(DAI)),
            low_gain: Int(0),
            high_gain: Int(0),
            pan: Pan::Centre,
            level: UInt(DL),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct VsDriveParameters<const DEFAULT_AMP_INDEX: i16> {
    drive: Level,
    tone: Level,
    amp_sw: Switch,
    amp_type: AmpType,
    low_gain: Gain,
    high_gain: Gain,
    pan: Pan,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 24>")]
    #[validate]
    unused_parameters: [Parameter; 24]
}

impl<const DAI: i16> Default for VsDriveParameters<DAI> {
    fn default() -> Self {
        Self {
            drive: UInt(127),
            tone: UInt(50),
            amp_sw: Switch(true),
            amp_type: AmpType::from(Parameter(DAI)),
            low_gain: Int(0),
            high_gain: Int(0),
            pan: Pan::Centre,
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct GuitarAmpSimulatorParameters {
    pre_amp_sw: Switch,
    pre_amp_type: PreAmpType,
    pre_amp_volume: Level,
    pre_amp_master: Level,
    pre_amp_gain: PreAmpGain,
    pre_amp_bass: Level,
    pre_amp_middle: Level,
    pre_amp_treble: Level,
    pre_amp_presence: Level,
    pre_amp_bright: Switch,
    speaker_sw: Switch,
    speaker_type: SpeakerType,
    mic_setting: MicSetting,
    mic_level: Level,
    direct_level: Level,
    pan: Pan,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate]
    unused_parameters: [Parameter; 15]
}

impl Default for GuitarAmpSimulatorParameters {
    fn default() -> Self {
        Self { 
            pre_amp_sw: Switch(true),
            pre_amp_type: PreAmpType::CleanTwin,
            pre_amp_volume: UInt(80),
            pre_amp_master: UInt(100),
            pre_amp_gain: PreAmpGain::Middle,
            pre_amp_bass: UInt(64),
            pre_amp_middle: UInt(64),
            pre_amp_treble: UInt(64),
            pre_amp_presence: UInt(0),
            pre_amp_bright: Switch(false),
            speaker_sw: Switch(true),
            speaker_type: SpeakerType::BuiltIn1,
            mic_setting: Int(2),
            mic_level: UInt(127),
            direct_level: UInt(0),
            pan: Pan::Centre,
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct CompressorParameters {
    attack: Level,
    threshold: Level,
    post_gain: PostGain,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 26>")]
    #[validate]
    unused_parameters: [Parameter; 26]
}

impl Default for CompressorParameters {
    fn default() -> Self {
        Self {
            attack: UInt(20),
            threshold: UInt(64),
            post_gain: Int(6),
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct LimiterParameters {
    release: Level,
    threshold: Level,
    ratio: CompressionRatio,
    post_gain: PostGain,
    low_gain: Gain,
    high_gain: Gain,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 25>")]
    #[validate]
    unused_parameters: [Parameter; 25]
}

impl Default for LimiterParameters {
    fn default() -> Self {
        Self {
            release: UInt(32),
            threshold: UInt(64),
            ratio: CompressionRatio::FourToOne,
            post_gain: Int(6),
            low_gain: Int(0),
            high_gain: Int(0),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct GateParameters {
    threshold: Level,
    mode: GateMode,
    attack: Level,
    hold: Level,
    release: Level,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 25>")]
    #[validate]
    unused_parameters: [Parameter; 25]
}

impl Default for GateParameters {
    fn default() -> Self {
        Self {
            threshold: UInt(70),
            mode: GateMode::Gate,
            attack: UInt(8),
            hold: UInt(0),
            release: UInt(16),
            balance: Balance(100),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct DelayParameters {
    delay_left_mode: DelayMode,
    delay_left_ms: LinearMilliseconds<1300>,
    delay_left_note: NoteLength,
    delay_right_mode: DelayMode,
    delay_right_ms: LinearMilliseconds<1300>,
    delay_right_note: NoteLength,
    phase_left: PhaseType,
    phase_right: PhaseType,
    feedback_mode: FeedbackMode,
    feedback_percent: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 17>")]
    #[validate]
    unused_parameters: [Parameter; 17]
}

impl Default for DelayParameters {
    fn default() -> Self {
        Self {
            delay_left_mode: DelayMode::Note,
            delay_left_ms: UInt(600),
            delay_left_note: NoteLength::QuarterNote,
            delay_right_mode: DelayMode::Note,
            delay_right_ms: UInt(600),
            delay_right_note: NoteLength::QuarterNote,
            phase_left: PhaseType::Normal,
            phase_right: PhaseType::Normal,
            feedback_mode: FeedbackMode::Normal,
            feedback_percent: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct LongDelayParameters {
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<2600>,
    delay_note: NoteLength,
    phase_type: PhaseType,
    feedback_percent: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    pan: Pan,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 21>")]
    #[validate]
    unused_parameters: [Parameter; 21]
}

impl Default for LongDelayParameters {
    fn default() -> Self {
        Self { 
            delay_mode: DelayMode::Note,
            delay_ms: UInt(1200),
            delay_note: NoteLength::HalfNote,
            phase_type: PhaseType::Normal,
            feedback_percent: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            pan: Pan::Centre,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct SerialDelayParameters {
    delay_1_mode: DelayMode,
    delay_1_ms: LinearMilliseconds<1300>,
    delay_1_note: NoteLength,
    delay_1_feedback: Feedback,
    delay_1_hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_2_mode: DelayMode,
    delay_2_ms: LinearMilliseconds<1300>,
    delay_2_note: NoteLength,
    delay_2_feedback: Feedback,
    delay_2_hf_damp: LogFrequencyOrByPass<200, 8000>,
    pan: Pan,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 17>")]
    #[validate]
    unused_parameters: [Parameter; 17]
}

impl Default for SerialDelayParameters {
    fn default() -> Self {
        Self { 
            delay_1_mode: DelayMode::Note,
            delay_1_ms: UInt(150),
            delay_1_note: NoteLength::SixteenthNote,
            delay_1_feedback: EvenPercent(40),
            delay_1_hf_damp: LogFrequencyOrByPass::ByPass,
            delay_2_mode: DelayMode::Note,
            delay_2_ms: UInt(600),
            delay_2_note: NoteLength::QuarterNote,
            delay_2_feedback: EvenPercent(40),
            delay_2_hf_damp: LogFrequencyOrByPass::ByPass,
            pan: Pan::Centre,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ModulationDelayParameters {
    delay_left_mode: DelayMode,
    delay_left_ms: LinearMilliseconds<1300>,
    delay_left_note: NoteLength,
    delay_right_mode: DelayMode,
    delay_right_ms: LinearMilliseconds<1300>,
    delay_right_note: NoteLength,
    feedback_mode: FeedbackMode,
    feedback_percent: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    depth: Level,
    phase: Phase,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 14>")]
    #[validate]
    unused_parameters: [Parameter; 14]
}

impl Default for ModulationDelayParameters {
    fn default() -> Self {
        Self {
            delay_left_mode: DelayMode::Note,
            delay_left_ms: UInt(600),
            delay_left_note: NoteLength::QuarterNote,
            delay_right_mode: DelayMode::Note,
            delay_right_ms: UInt(600),
            delay_right_note: NoteLength::QuarterNote,
            feedback_mode: FeedbackMode::Normal,
            feedback_percent: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            rate_mode: RateMode::Hertz,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            depth: UInt(20),
            phase: Phase(180),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ThreeTapPanDelayParameters {
    delay_left_mode: DelayMode,
    delay_left_ms: LinearMilliseconds<2600>,
    delay_left_note: NoteLength,
    delay_right_mode: DelayMode,
    delay_right_ms: LinearMilliseconds<2600>,
    delay_right_note: NoteLength,
    delay_centre_mode: DelayMode,
    delay_centre_ms: LinearMilliseconds<2600>,
    delay_centre_note: NoteLength,
    centre_feedback: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    left_level: Level,
    right_level: Level,
    centre_level: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 14>")]
    #[validate]
    unused_parameters: [Parameter; 14]
}

impl Default for ThreeTapPanDelayParameters {
    fn default() -> Self {
        Self {
            delay_left_mode: DelayMode::Note,
            delay_left_ms: UInt(400),
            delay_left_note: NoteLength::QuarterNoteTriplet,
            delay_right_mode: DelayMode::Note,
            delay_right_ms: UInt(800),
            delay_right_note: NoteLength::HalfNoteTriplet,
            delay_centre_mode: DelayMode::Note,
            delay_centre_ms: UInt(1200),
            delay_centre_note: NoteLength::HalfNote,
            centre_feedback: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            left_level: UInt(127),
            right_level: UInt(127),
            centre_level: UInt(127),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct FourTapPanDelayParameters {
    delay_1_mode: DelayMode,
    delay_1_ms: LinearMilliseconds<2600>,
    delay_1_note: NoteLength,
    delay_2_mode: DelayMode,
    delay_2_ms: LinearMilliseconds<2600>,
    delay_2_note: NoteLength,
    delay_3_mode: DelayMode,
    delay_3_ms: LinearMilliseconds<2600>,
    delay_3_note: NoteLength,
    delay_4_mode: DelayMode,
    delay_4_ms: LinearMilliseconds<2600>,
    delay_4_note: NoteLength,
    delay_1_feedback: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_1_level: Level,
    delay_2_level: Level,
    delay_3_level: Level,
    delay_4_level: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 10>")]
    #[validate]
    unused_parameters: [Parameter; 10]
}

impl Default for FourTapPanDelayParameters {
    fn default() -> Self {
        Self {
            delay_1_mode: DelayMode::Note,
            delay_1_ms: UInt(1200),
            delay_1_note: NoteLength::HalfNote,
            delay_2_mode: DelayMode::Note,
            delay_2_ms: UInt(900),
            delay_2_note: NoteLength::DottedQuarterNote,
            delay_3_mode: DelayMode::Note,
            delay_3_ms: UInt(600),
            delay_3_note: NoteLength::QuarterNote,
            delay_4_mode: DelayMode::Note,
            delay_4_ms: UInt(300),
            delay_4_note: NoteLength::EighthNote,
            delay_1_feedback: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            delay_1_level: UInt(127),
            delay_2_level: UInt(127),
            delay_3_level: UInt(127),
            delay_4_level: UInt(127),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct MultiTapDelayParameters {
    delay_1_mode: DelayMode,
    delay_1_ms: LinearMilliseconds<2600>,
    delay_1_note: NoteLength,
    delay_2_mode: DelayMode,
    delay_2_ms: LinearMilliseconds<2600>,
    delay_2_note: NoteLength,
    delay_3_mode: DelayMode,
    delay_3_ms: LinearMilliseconds<2600>,
    delay_3_note: NoteLength,
    delay_4_mode: DelayMode,
    delay_4_ms: LinearMilliseconds<2600>,
    delay_4_note: NoteLength,
    delay_1_feedback: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_1_pan: Pan,
    delay_2_pan: Pan,
    delay_3_pan: Pan,
    delay_4_pan: Pan,
    delay_1_level: Level,
    delay_2_level: Level,
    delay_3_level: Level,
    delay_4_level: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 6>")]
    #[validate]
    unused_parameters: [Parameter; 6]
}

impl Default for MultiTapDelayParameters {
    fn default() -> Self {
        Self {
            delay_1_mode: DelayMode::Note,
            delay_1_ms: UInt(1200),
            delay_1_note: NoteLength::HalfNote,
            delay_2_mode: DelayMode::Note,
            delay_2_ms: UInt(900),
            delay_2_note: NoteLength::DottedQuarterNote,
            delay_3_mode: DelayMode::Note,
            delay_3_ms: UInt(600),
            delay_3_note: NoteLength::QuarterNote,
            delay_4_mode: DelayMode::Note,
            delay_4_ms: UInt(300),
            delay_4_note: NoteLength::EighthNote,
            delay_1_feedback: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            delay_1_pan: Pan::Left(64),
            delay_2_pan: Pan::Right(63),
            delay_3_pan: Pan::Left(32),
            delay_4_pan: Pan::Right(32),
            delay_1_level: UInt(127),
            delay_2_level: UInt(127),
            delay_3_level: UInt(127),
            delay_4_level: UInt(127),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ReverseDelayParameters {
    threshold: Level,
    rev_delay_mode: DelayMode,
    rev_delay_ms: LinearMilliseconds<1300>,
    rev_delay_note: NoteLength,
    rev_delay_feedback: Feedback,
    rev_delay_hf_damp: LogFrequencyOrByPass<200, 8000>,
    rev_delay_pan: Pan,
    rev_delay_level: Level,
    delay_1_mode: DelayMode,
    delay_1_ms: LinearMilliseconds<1300>,
    delay_1_note: NoteLength,
    delay_2_mode: DelayMode,
    delay_2_ms: LinearMilliseconds<1300>,
    delay_2_note: NoteLength,
    delay_3_mode: DelayMode,
    delay_3_ms: LinearMilliseconds<1300>,
    delay_3_note: NoteLength,
    delay_3_feedback: Feedback,
    delay_hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_1_pan: Pan,
    delay_2_pan: Pan,
    delay_1_level: Level,
    delay_2_level: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 5>")]
    #[validate]
    unused_parameters: [Parameter; 5]
}

impl Default for ReverseDelayParameters {
    fn default() -> Self {
        Self {
            threshold: UInt(30),
            rev_delay_mode: DelayMode::Note,
            rev_delay_ms: UInt(600),
            rev_delay_note: NoteLength::QuarterNote,
            rev_delay_feedback: EvenPercent(0),
            rev_delay_hf_damp: LogFrequencyOrByPass::ByPass,
            rev_delay_pan: Pan::Centre,
            rev_delay_level: UInt(127),
            delay_1_mode: DelayMode::Note,
            delay_1_ms: UInt(300),
            delay_1_note: NoteLength::EighthNote,
            delay_2_mode: DelayMode::Note,
            delay_2_ms: UInt(600),
            delay_2_note: NoteLength::QuarterNote,
            delay_3_mode: DelayMode::Note,
            delay_3_ms: UInt(600),
            delay_3_note: NoteLength::QuarterNote,
            delay_3_feedback: EvenPercent(0),
            delay_hf_damp: LogFrequencyOrByPass::ByPass,
            delay_1_pan: Pan::Left(64),
            delay_2_pan: Pan::Right(63),
            delay_1_level: UInt(0),
            delay_2_level: UInt(0),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(20),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ShuffleDelayParameters {
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<2600>,
    delay_note: NoteLength,
    shuffle_rate: UInt<0, 100>,
    acceleration: UInt<0, 15>,
    feedback: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    pan_a: Pan,
    pan_b: Pan,
    level_a: Level,
    level_b: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 17>")]
    #[validate]
    unused_parameters: [Parameter; 17]
}

impl Default for ShuffleDelayParameters {
    fn default() -> Self {
        Self {
            delay_mode: DelayMode::Note,
            delay_ms: UInt(600),
            delay_note: NoteLength::QuarterNote,
            shuffle_rate: UInt(67),
            acceleration: UInt(10),
            feedback: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            pan_a: Pan::Left(64),
            pan_b: Pan::Right(63),
            level_a: UInt(127),
            level_b: UInt(127),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct Delay3DParameters {
    delay_left_mode: DelayMode,
    delay_left_ms: LinearMilliseconds<2600>,
    delay_left_note: NoteLength,
    delay_right_mode: DelayMode,
    delay_right_ms: LinearMilliseconds<2600>,
    delay_right_note: NoteLength,
    delay_centre_mode: DelayMode,
    delay_centre_ms: LinearMilliseconds<2600>,
    delay_centre_note: NoteLength,
    centre_feedback: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    left_level: Level,
    right_level: Level,
    centre_level: Level,
    output_mode: OutputMode,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 13>")]
    #[validate]
    unused_parameters: [Parameter; 13]
}

impl Default for Delay3DParameters {
    fn default() -> Self {
        Self {
            delay_left_mode: DelayMode::Note,
            delay_left_ms: UInt(400),
            delay_left_note: NoteLength::QuarterNoteTriplet,
            delay_right_mode: DelayMode::Note,
            delay_right_ms: UInt(800),
            delay_right_note: NoteLength::HalfNoteTriplet,
            delay_centre_mode: DelayMode::Note,
            delay_centre_ms: UInt(1200),
            delay_centre_note: NoteLength::HalfNote,
            centre_feedback: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            left_level: UInt(64),
            right_level: UInt(64),
            centre_level: UInt(40),
            output_mode: OutputMode::Speaker,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(20),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TimeCtrlDelayParameters {
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<1300>,
    delay_note: NoteLength,
    acceleration: UInt<0, 15>,
    feedback: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl Default for TimeCtrlDelayParameters {
    fn default() -> Self {
        Self {
            delay_mode: DelayMode::Milliseconds,
            delay_ms: UInt(600),
            delay_note: NoteLength::QuarterNote,
            acceleration: UInt(10),
            feedback: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct LongTimeCtrlDelayParameters {
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<2600>,
    delay_note: NoteLength,
    acceleration: UInt<0, 15>,
    feedback: Feedback,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    pan: Pan,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 21>")]
    #[validate]
    unused_parameters: [Parameter; 21]
}

impl Default for LongTimeCtrlDelayParameters {
    fn default() -> Self {
        Self {
            delay_mode: DelayMode::Milliseconds,
            delay_ms: UInt(1200),
            delay_note: NoteLength::HalfNote,
            acceleration: UInt(10),
            feedback: EvenPercent(20),
            hf_damp: LogFrequencyOrByPass::ByPass,
            pan: Pan::Centre,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TapeEchoParameters {
    mode: TapeHeads,
    repeat_rate: Level,
    intensity: Level,
    bass: Gain,
    treble: Gain,
    head_short_pan: Pan,
    head_middle_pan: Pan,
    head_long_pan: Pan,
    tape_distortion: UInt<0, 5>,
    wow_flutter_rate: Level,
    wow_flutter_depth: Level,
    echo_level: Level,
    direct_level: Level,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 18>")]
    #[validate]
    unused_parameters: [Parameter; 18]
}

impl Default for TapeEchoParameters {
    fn default() -> Self {
        Self {
            mode: TapeHeads::Long,
            repeat_rate: UInt(64),
            intensity: UInt(64),
            bass: Int(0),
            treble: Int(0),
            head_short_pan: Pan::Centre,
            head_middle_pan: Pan::Centre,
            head_long_pan: Pan::Centre,
            tape_distortion: UInt(0),
            wow_flutter_rate: UInt(20),
            wow_flutter_depth: UInt(20),
            echo_level: UInt(64),
            direct_level: UInt(127),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct LofiNoiseParameters {
    lofi_type: LofiType,
    post_filter_type: FilterType,
    post_filter_cutoff: LogFrequency<200, 8000>,
    noise_type: NoiseType,
    noise_lpf: LogFrequencyOrByPass<200, 8000>,
    noise_level: Level,
    disc_noise_type: DiscTypeWithRandom,
    disc_noise_lpf: LogFrequencyOrByPass<200, 8000>,
    disc_noise_level: Level,
    hum_noise_type: HumFrequency,
    hum_noise_lpf: LogFrequencyOrByPass<200, 8000>,
    hum_noise_level: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 16>")]
    #[validate]
    unused_parameters: [Parameter; 16]
}

impl Default for LofiNoiseParameters {
    fn default() -> Self {
        Self {
            lofi_type: Int(5),
            post_filter_type: FilterType::LowPassFilter,
            post_filter_cutoff: LogFrequency(4000),
            noise_type: NoiseType::White,
            noise_lpf: LogFrequencyOrByPass::Frequency(LogFrequency(2000)),
            noise_level: UInt(0),
            disc_noise_type: DiscTypeWithRandom::RND,
            disc_noise_lpf: LogFrequencyOrByPass::Frequency(LogFrequency(2000)),
            disc_noise_level: UInt(0),
            hum_noise_type: HumFrequency(50),
            hum_noise_lpf: LogFrequencyOrByPass::Frequency(LogFrequency(2000)),
            hum_noise_level: UInt(0),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(100),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct LofiCompressParameters {
    pre_filter_type: Int<1, 6>,
    lofi_type: LofiType,
    post_filter_type: FilterType,
    post_filter_cutoff: LogFrequency<200, 8000>,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 24>")]
    #[validate]
    unused_parameters: [Parameter; 24]
}

impl Default for LofiCompressParameters {
    fn default() -> Self {
        Self {
            pre_filter_type: Int(2),
            lofi_type: Int(5),
            post_filter_type: FilterType::LowPassFilter,
            post_filter_cutoff: LogFrequency(4000),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(100),
            level: UInt(100),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct LofiRadioParameters {
    lofi_type: LofiType,
    post_filter_type: FilterType,
    post_filter_cutoff: LogFrequency<200, 8000>,
    radio_detune: Level,
    radio_noise_level: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 23>")]
    #[validate]
    unused_parameters: [Parameter; 23]
}

impl Default for LofiRadioParameters {
    fn default() -> Self {
        Self {
            lofi_type: Int(5),
            post_filter_type: FilterType::LowPassFilter,
            post_filter_cutoff: LogFrequency(4000),
            radio_detune: UInt(0),
            radio_noise_level: UInt(64),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(100),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TelephoneParameters {
    voice_quality: UInt<0, 15>,
    treble: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 28>")]
    #[validate]
    unused_parameters: [Parameter; 28]
}

impl Default for TelephoneParameters {
    fn default() -> Self {
        Self {
            voice_quality: UInt(3),
            treble: Int(0),
            balance: Balance(100),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct PhonographParameters {
    signal_distortion: Level,
    frequencey_range: Level,
    disc_type: DiscType,
    scartch_noise_level: Level,
    dust_noise_level: Level,
    hiss_noise_level: Level,
    total_noise_level: Level,
    wow: Level,
    flutter: Level,
    random: Level,
    total_wf: Level,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 19>")]
    #[validate]
    unused_parameters: [Parameter; 19]
}

impl Default for PhonographParameters {
    fn default() -> Self {
        Self {
            signal_distortion: UInt(10),
            frequencey_range: UInt(90),
            disc_type: DiscType::LP,
            scartch_noise_level: UInt(64),
            dust_noise_level: UInt(64),
            hiss_noise_level: UInt(64),
            total_noise_level: UInt(0),
            wow: UInt(80),
            flutter: UInt(80),
            random: UInt(80),
            total_wf: UInt(50),
            balance: Balance(100),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct PitchShifterParameters {
    coarse_semitones: Semitones,
    fine_percent: EvenPercent<100>,
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<1300>,
    delay_note: NoteLength,
    feedback: Feedback,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl Default for PitchShifterParameters {
    fn default() -> Self {
        Self {
            coarse_semitones: Int(0),
            fine_percent: EvenPercent(20),
            delay_mode: DelayMode::Milliseconds,
            delay_ms: UInt(1),
            delay_note: NoteLength::QuarterNote,
            feedback: EvenPercent(0),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct TwoVoicePitchShifterParameters {
    pitch_1_coarse_semitones: Semitones,
    pitch_1_fine_percent: EvenPercent<100>,
    pitch_1_delay_mode: DelayMode,
    pitch_1_delay_ms: LinearMilliseconds<1300>,
    pitch_1_delay_note: NoteLength,
    pitch_1_feedback: Feedback,
    pitch_1_pan: Pan,
    pitch_1_level: Level,
    pitch_2_coarse_semitones: Semitones,
    pitch_2_fine_percent: EvenPercent<100>,
    pitch_2_delay_mode: DelayMode,
    pitch_2_delay_ms: LinearMilliseconds<1300>,
    pitch_2_delay_note: NoteLength,
    pitch_2_feedback: Feedback,
    pitch_2_pan: Pan,
    pitch_2_level: Level,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 12>")]
    #[validate]
    unused_parameters: [Parameter; 12]
}

impl Default for TwoVoicePitchShifterParameters {
    fn default() -> Self {
        Self {
            pitch_1_coarse_semitones: Int(4),
            pitch_1_fine_percent: EvenPercent(0),
            pitch_1_delay_mode: DelayMode::Note,
            pitch_1_delay_ms: UInt(300),
            pitch_1_delay_note: NoteLength::EighthNote,
            pitch_1_feedback: EvenPercent(0),
            pitch_1_pan: Pan::Centre,
            pitch_1_level: UInt(127),
            pitch_2_coarse_semitones: Int(7),
            pitch_2_fine_percent: EvenPercent(0),
            pitch_2_delay_mode: DelayMode::Note,
            pitch_2_delay_ms: UInt(600),
            pitch_2_delay_note: NoteLength::QuarterNote,
            pitch_2_feedback: EvenPercent(0),
            pitch_2_pan: Pan::Centre,
            pitch_2_level: UInt(127),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct StepPitchShifterParameters {
    step1: Semitones,
    step2: Semitones,
    step3: Semitones,
    step4: Semitones,
    step5: Semitones,
    step6: Semitones,
    step7: Semitones,
    step8: Semitones,
    step9: Semitones,
    step10: Semitones,
    step11: Semitones,
    step12: Semitones,
    step13: Semitones,
    step14: Semitones,
    step15: Semitones,
    step16: Semitones,
    rate_mode: RateMode,
    rate_hz: LinearFrequency,
    rate_note: NoteLength,
    attack: Level,
    gate_time: Level,
    fine_percent: EvenPercent<100>,
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<1300>,
    delay_note: NoteLength,
    feedback: Feedback,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 2>")]
    #[validate]
    unused_parameters: [Parameter; 2]
}

impl Default for StepPitchShifterParameters {
    fn default() -> Self {
        Self {
            step1: Int(2),
            step2: Int(0),
            step3: Int(0),
            step4: Int(0),
            step5: Int(3),
            step6: Int(0),
            step7: Int(0),
            step8: Int(3),
            step9: Int(0),
            step10: Int(3),
            step11: Int(0),
            step12: Int(3),
            step13: Int(0),
            step14: Int(0),
            step15: Int(2),
            step16: Int(0),
            rate_mode: RateMode::Note,
            rate_hz: LinearFrequency(0.5),
            rate_note: NoteLength::WholeNote,
            attack: UInt(100),
            gate_time: UInt(100),
            fine_percent: EvenPercent(0),
            delay_mode: DelayMode::Milliseconds,
            delay_ms: UInt(1),
            delay_note: NoteLength::WholeNote,
            feedback: EvenPercent(0),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(100),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ReverbParameters {
    character: ReverbOnlyCharacter,
    pre_delay: LogMilliseconds,
    time: Level,
    hf_damp: LogFrequencyOrByPass<200, 8000>,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 24>")]
    #[validate]
    unused_parameters: [Parameter; 24]
}

impl Default for ReverbParameters {
    fn default() -> Self {
        Self {
            character: ReverbOnlyCharacter::Stage1,
            pre_delay: LogMilliseconds(1.0),
            time: UInt(64),
            hf_damp: LogFrequencyOrByPass::ByPass,
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct GatedReverbParameters {
    gate_type: GateType,
    pre_delay: LogMilliseconds,
    gate_time: GateTime,
    low_gain: Gain,
    high_gain: Gain,
    balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 25>")]
    #[validate]
    unused_parameters: [Parameter; 25]
}

impl Default for GatedReverbParameters {
    fn default() -> Self {
        Self {
            gate_type: GateType::Normal,
            pre_delay: LogMilliseconds(1.0),
            gate_time: GateTime(400),
            low_gain: Int(0),
            high_gain: Int(0),
            balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct DriveChorusParameters<const DEFAULT_DRIVE: u16, const DEFAULT_LEVEL: u16> {
    drive: Level,
    pan: Pan,
    chorus_pre_delay: LogMilliseconds,
    chorus_rate_mode: RateMode,
    chorus_rate_hz: LinearFrequency,
    chorus_rate_note: NoteLength,
    chorus_depth: Level,
    chorus_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 23>")]
    #[validate]
    unused_parameters: [Parameter; 23]
}

impl<const DD: u16, const DL: u16> Default for DriveChorusParameters<DD, DL> {
    fn default() -> Self {
        Self {
            drive: UInt(DD),
            pan: Pan::Centre,
            chorus_pre_delay: LogMilliseconds(2.0),
            chorus_rate_mode: RateMode::Hertz,
            chorus_rate_hz: LinearFrequency(0.5),
            chorus_rate_note: NoteLength::WholeNote,
            chorus_depth: UInt(20),
            chorus_balance: Balance(50),
            level: UInt(DL),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct DriveFlangerParameters<const DEFAULT_DRIVE: u16> {
    drive: Level,
    pan: Pan,
    flanger_pre_delay: LogMilliseconds,
    flanger_rate_mode: RateMode,
    flanger_rate_hz: LinearFrequency,
    flanger_rate_note: NoteLength,
    flanger_depth: Level,
    flanger_feedback: Feedback,
    flanger_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl<const DD: u16> Default for DriveFlangerParameters<DD> {
    fn default() -> Self {
        Self {
            drive: UInt(DD),
            pan: Pan::Centre,
            flanger_pre_delay: LogMilliseconds(2.0),
            flanger_rate_mode: RateMode::Note,
            flanger_rate_hz: LinearFrequency(0.5),
            flanger_rate_note: NoteLength::WholeNote,
            flanger_depth: UInt(40),
            flanger_feedback: EvenPercent(60),
            flanger_balance: Balance(50),
            level: UInt(80),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct DriveDelayParameters<const DEFAULT_DRIVE: u16> {
    drive: Level,
    pan: Pan,
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<2600>,
    delay_note: NoteLength,
    delay_feedback: Feedback,
    delay_hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 23>")]
    #[validate]
    unused_parameters: [Parameter; 23]
}

impl<const DD: u16> Default for DriveDelayParameters<DD> {
    fn default() -> Self {
        Self {
            drive: UInt(DD),
            pan: Pan::Centre,
            delay_mode: DelayMode::Note,
            delay_ms: UInt(600),
            delay_note: NoteLength::QuarterNote,
            delay_feedback: EvenPercent(20),
            delay_hf_damp: LogFrequencyOrByPass::ByPass,
            delay_balance: Balance(10),
            level: UInt(80),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct EnhancerChorusParameters {
    enhancer_sensitivity: Level,
    enhancer_mix: Level,
    chorus_pre_delay: LogMilliseconds,
    chorus_rate_mode: RateMode,
    chorus_rate_hz: LinearFrequency,
    chorus_rate_note: NoteLength,
    chorus_depth: Level,
    chorus_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 23>")]
    #[validate]
    unused_parameters: [Parameter; 23]
}

impl Default for EnhancerChorusParameters {
    fn default() -> Self {
        Self {
            enhancer_sensitivity: UInt(64),
            enhancer_mix: UInt(64),
            chorus_pre_delay: LogMilliseconds(2.0),
            chorus_rate_mode: RateMode::Hertz,
            chorus_rate_hz: LinearFrequency(0.5),
            chorus_rate_note: NoteLength::WholeNote,
            chorus_depth: UInt(20),
            chorus_balance: Balance(50),
            level: UInt(100),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct EnhancerFlangerParameters {
    enhancer_sensitivity: Level,
    enhancer_mix: Level,
    flanger_pre_delay: LogMilliseconds,
    flanger_rate_mode: RateMode,
    flanger_rate_hz: LinearFrequency,
    flanger_rate_note: NoteLength,
    flanger_depth: Level,
    flanger_feedback: Feedback,
    flanger_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 22>")]
    #[validate]
    unused_parameters: [Parameter; 22]
}

impl Default for EnhancerFlangerParameters {
    fn default() -> Self {
        Self {
            enhancer_sensitivity: UInt(64),
            enhancer_mix: UInt(64),
            flanger_pre_delay: LogMilliseconds(2.0),
            flanger_rate_mode: RateMode::Note,
            flanger_rate_hz: LinearFrequency(0.5),
            flanger_rate_note: NoteLength::WholeNote,
            flanger_depth: UInt(40),
            flanger_feedback: EvenPercent(60),
            flanger_balance: Balance(50),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct EnhancerDelayParameters {
    enhancer_sensitivity: Level,
    enhancer_mix: Level,
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<2600>,
    delay_note: NoteLength,
    delay_feedback: Feedback,
    delay_hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 23>")]
    #[validate]
    unused_parameters: [Parameter; 23]
}

impl Default for EnhancerDelayParameters {
    fn default() -> Self {
        Self {
            enhancer_sensitivity: UInt(64),
            enhancer_mix: UInt(64),
            delay_mode: DelayMode::Note,
            delay_ms: UInt(600),
            delay_note: NoteLength::QuarterNote,
            delay_feedback: EvenPercent(20),
            delay_hf_damp: LogFrequencyOrByPass::ByPass,
            delay_balance: Balance(10),
            level: UInt(127),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ChorusDelayParameters {
    chorus_pre_delay: LogMilliseconds,
    chorus_rate_mode: RateMode,
    chorus_rate_hz: LinearFrequency,
    chorus_rate_note: NoteLength,
    chorus_depth: Level,
    chorus_balance: Balance,
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<2600>,
    delay_note: NoteLength,
    delay_feedback: Feedback,
    delay_hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 19>")]
    #[validate]
    unused_parameters: [Parameter; 19]
}

impl Default for ChorusDelayParameters {
    fn default() -> Self {
        Self {
            chorus_pre_delay: LogMilliseconds(2.0),
            chorus_rate_mode: RateMode::Hertz,
            chorus_rate_hz: LinearFrequency(0.5),
            chorus_rate_note: NoteLength::WholeNote,
            chorus_depth: UInt(20),
            chorus_balance: Balance(50),
            delay_mode: DelayMode::Note,
            delay_ms: UInt(600),
            delay_note: NoteLength::QuarterNote,
            delay_feedback: EvenPercent(20),
            delay_hf_damp: LogFrequencyOrByPass::ByPass,
            delay_balance: Balance(10),
            level: UInt(100),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct FlangerDelayParameters {
    flanger_pre_delay: LogMilliseconds,
    flanger_rate_mode: RateMode,
    flanger_rate_hz: LinearFrequency,
    flanger_rate_note: NoteLength,
    flanger_depth: Level,
    flanger_feedback: Feedback,
    flanger_balance: Balance,
    delay_mode: DelayMode,
    delay_ms: LinearMilliseconds<2600>,
    delay_note: NoteLength,
    delay_feedback: Feedback,
    delay_hf_damp: LogFrequencyOrByPass<200, 8000>,
    delay_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 18>")]
    #[validate]
    unused_parameters: [Parameter; 18]
}

impl Default for FlangerDelayParameters {
    fn default() -> Self {
        Self {
            flanger_pre_delay: LogMilliseconds(2.0),
            flanger_rate_mode: RateMode::Note,
            flanger_rate_hz: LinearFrequency(0.5),
            flanger_rate_note: NoteLength::WholeNote,
            flanger_depth: UInt(40),
            flanger_feedback: EvenPercent(60),
            flanger_balance: Balance(50),
            delay_mode: DelayMode::Note,
            delay_ms: UInt(600),
            delay_note: NoteLength::QuarterNote,
            delay_feedback: EvenPercent(20),
            delay_hf_damp: LogFrequencyOrByPass::ByPass,
            delay_balance: Balance(10),
            level: UInt(100),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct ChorusFlangerParameters {
    chorus_pre_delay: LogMilliseconds,
    chorus_rate_mode: RateMode,
    chorus_rate_hz: LinearFrequency,
    chorus_rate_note: NoteLength,
    chorus_depth: Level,
    chorus_balance: Balance,
    flanger_pre_delay: LogMilliseconds,
    flanger_rate_mode: RateMode,
    flanger_rate_hz: LinearFrequency,
    flanger_rate_note: NoteLength,
    flanger_depth: Level,
    flanger_feedback: Feedback,
    flanger_balance: Balance,
    level: Level,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 18>")]
    #[validate]
    unused_parameters: [Parameter; 18]
}

impl Default for ChorusFlangerParameters {
    fn default() -> Self {
        Self {
            chorus_pre_delay: LogMilliseconds(2.0),
            chorus_rate_mode: RateMode::Hertz,
            chorus_rate_hz: LinearFrequency(0.5),
            chorus_rate_note: NoteLength::WholeNote,
            chorus_depth: UInt(20),
            chorus_balance: Balance(50),
            flanger_pre_delay: LogMilliseconds(2.0),
            flanger_rate_mode: RateMode::Note,
            flanger_rate_hz: LinearFrequency(0.5),
            flanger_rate_note: NoteLength::WholeNote,
            flanger_depth: UInt(40),
            flanger_feedback: EvenPercent(60),
            flanger_balance: Balance(50),
            level: UInt(100),
            unused_parameters: Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate, Parameters)]
pub struct SympatheticResonanceParameters {
    depth: Level,
    damper: Level,
    pre_lpf: LogFrequencyOrByPassOffByOne<16, 16000>, // technically this mislabels 15000 as 16000, but it really doesn't matter
    pre_hpf: ByPassOrLogFrequency<16, 16000>, // technically this mislabels 15000 as 16000, but it really doesn't matter
    peaking_freq: LogFrequency<200, 8000>,
    peaking_gain: Gain,
    peaking_q: QFactor,
    hf_damp: LogFrequencyOrByPassOffByOne<16, 16000>, // technically this mislabels 15000 as 16000, but it really doesn't matter
    lf_damp: ByPassOrLogFrequency<16, 16000>, // technically this mislabels 15000 as 16000, but it really doesn't matter
    lid_open: Int<1, 7>,
    level: Level,
    p_sft_amount: Level,
    p_sft_level: Level,
    p_sft_lpf: LogFrequencyOrByPassOffByOne<16, 16000>, // technically this mislabels 15000 as 16000, but it really doesn't matter
    p_sft_hpf: ByPassOrLogFrequency<16, 16000>, // technically this mislabels 15000 as 16000, but it really doesn't matter
    p_sft_to_rev: Level,
    damper_offset: UInt<0, 64>,
    #[serde(deserialize_with = "serialize_default_terminated_array::deserialize")]
    #[serde(serialize_with = "serialize_default_terminated_array::serialize")]
    #[schemars(with = "serialize_default_terminated_array::DefaultTerminatedArraySchema::<Parameter, 15>")]
    #[validate]
    unused_parameters: [Parameter; 15]
}

impl Default for SympatheticResonanceParameters {
    fn default() -> Self {
        Self {
            depth: UInt(52),
            damper: UInt(0),
            pre_lpf: LogFrequencyOrByPassOffByOne::Frequency(LogFrequency(5000)),
            pre_hpf: ByPassOrLogFrequency::ByPass,
            peaking_freq: LogFrequency(250),
            peaking_gain: Int(3),
            peaking_q: QFactor(0.5),
            hf_damp: LogFrequencyOrByPassOffByOne::Frequency(LogFrequency(3150)),
            lf_damp: ByPassOrLogFrequency::Frequency(LogFrequency(200)),
            lid_open: Int(5),
            level: UInt(127),
            p_sft_amount: UInt(23),
            p_sft_level: UInt(99),
            p_sft_lpf: LogFrequencyOrByPassOffByOne::Frequency(LogFrequency(10000)),
            p_sft_hpf: ByPassOrLogFrequency::ByPass,
            p_sft_to_rev: UInt(127),
            damper_offset: UInt(0), // but 34 is the value used by the default Grand Piano
            unused_parameters: Default::default()
        }
    }
}