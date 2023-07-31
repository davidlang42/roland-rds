use std::collections::HashMap;
use std::fmt::Debug;

use schemars::JsonSchema;
use strum::IntoEnumIterator;
use validator::Validate;

use crate::bytes::{Bytes, BytesError, Bits, BitStream};
use crate::json::serialize_map_keys_in_order;
use crate::json::validation::{contains_all_keys, LayerRanges, valid_key_range, valid_velocity_range};
use crate::roland::types::numeric::OffsetU8;
use crate::roland::types::notes::PianoKey;
use crate::roland::types::enums::{Layer, TransmitPort, MonoPolyOnly, Pan, MidiChannel};

#[derive(Serialize, Deserialize, Debug, JsonSchema, Validate)]
#[validate(schema(function = "valid_key_range"))]
#[validate(schema(function = "valid_velocity_range"))]
pub struct ExternalLayer {
    range_lower: PianoKey, // max 87 (A0-C8)
    range_upper: PianoKey, // max 87 (A0-C8), must be >= lower
    #[validate(range(min = 1, max = 127))]
    velocity_range_lower: u8,
    #[validate(range(min = 1, max = 127))]
    velocity_range_upper: u8,
    #[validate]
    velocity_sensitivity: OffsetU8<64, 1, 127>, // 1-127 (-63 - +63)
    #[validate(range(min = 1, max = 127))]
    velocity_max: u8,
    #[validate]
    transpose: OffsetU8<64, 16, 112>, // 16-112 (-48 - +48)
    enable: bool,
    damper: bool,
    fc1: bool,
    fc2: bool,
    modulation: bool,
    bender: bool,
    control_mfx_switch: bool,
    #[serde(deserialize_with = "serialize_map_keys_in_order::deserialize")]
    #[serde(serialize_with = "serialize_map_keys_in_order::serialize")]
    #[schemars(with = "serialize_map_keys_in_order::RequiredMapSchema::<Layer, bool>")]
    #[validate(custom = "contains_all_keys")]
    control_slider: HashMap<Layer, bool>,
    transmit_port: TransmitPort,
    transmit_channel: MidiChannel,
    transmit_bank_select_msb: bool,
    #[validate(range(max = 127))]
    bank_select_msb: u8,
    transmit_bank_select_lsb: bool,
    #[validate(range(max = 127))]
    bank_select_lsb: u8,
    transmit_program_change: bool,
    #[validate(range(max = 127))]
    program_change: u8,
    transmit_level: bool,
    #[validate(range(max = 127))]
    level: u8,
    transmit_pan: bool,
    #[validate]
    pan: Pan,
    transmit_course_tune: bool,
    #[validate]
    course_tune_semitones: OffsetU8<64, 16, 112>, // 16-112 (-48 - +48)
    transmit_fine_tine: bool,
    #[validate]
    fine_tune_percent: OffsetU8<64, 14, 114>, // 14-114 (-50 - + 50)
    transmit_mono_poly: bool,
    mono_poly: MonoPolyOnly,
    transmit_portamento: bool,
    portamento_switch: bool,
    transmit_portamento_time: bool,
    #[validate(range(max = 127))]
    portamento_time: u8,
    transmit_cutoff: bool,
    #[validate]
    cutoff: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    transmit_resonance: bool,
    #[validate]
    resonance: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    transmit_attack_time: bool,
    #[validate]
    attack_time: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    transmit_decay_time: bool,
    #[validate]
    decay_time: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    transmit_release_time: bool,
    #[validate]
    release_time: OffsetU8<64, 0, 127>, // max 127 (-64 - +63)
    transmit_pitch_bend_range: bool,
    #[validate(range(max = 48))]
    pitch_bend_range_semitones: u8,
    transmit_modulation_depth: bool,
    #[validate(range(max = 127))]
    modulation_depth: u8,
    transmit_chorus_level: bool,
    #[validate(range(max = 127))]
    chorus_level: u8,
    transmit_reverb_level: bool,
    #[validate(range(max = 127))]
    reverb_level: u8,
    transmit_control_change_1: bool,
    #[validate(range(max = 127))]
    control_change_1_number: u8,
    #[validate(range(max = 127))]
    control_change_1_value: u8,
    transmit_control_change_2: bool,
    #[validate(range(max = 127))]
    control_change_2_number: u8,
    #[validate(range(max = 127))]
    control_change_2_value: u8,
    s1: bool,
    s2: bool,
    #[serde(skip_serializing_if="Bits::is_zero", default="Bits::<1>::zero")]
    unused: Bits<1>
}

impl Bytes<30> for ExternalLayer {
    fn to_bytes(&self) -> Result<Box<[u8; 30]>, BytesError> {
        BitStream::write_fixed(|bits| {
            bits.set_u8::<7>(self.range_lower.into(), 0, 87)?;
            bits.set_u8::<7>(self.range_upper.into(), self.range_lower.into(), 87)?;
            bits.set_u8::<7>(self.velocity_range_lower, 1, 127)?;
            bits.set_u8::<7>(self.velocity_range_upper, 1, 127)?;
            bits.set_u8::<7>(self.velocity_sensitivity.into(), 1, 127)?;
            bits.set_u8::<7>(self.velocity_max, 1, 127)?;
            bits.set_u8::<7>(self.transpose.into(), 16, 112)?;
            bits.set_bool(self.enable);
            bits.set_bool(self.damper);
            bits.set_bool(self.fc1);
            bits.set_bool(self.fc2);
            bits.set_bool(self.modulation);
            bits.set_bool(self.bender);
            bits.set_bool(self.control_mfx_switch);
            for key in Layer::iter() {
                bits.set_bool(*self.control_slider.get(&key).unwrap());
            }
            bits.set_u8::<3>(self.transmit_port.into(), 0, 4)?;
            bits.set_u8::<4>(self.transmit_channel.into(), 0, 15)?;
            bits.set_bool(self.transmit_bank_select_msb);
            bits.set_u8::<7>(self.bank_select_msb, 0, 127)?;
            bits.set_bool(self.transmit_bank_select_lsb);
            bits.set_u8::<7>(self.bank_select_lsb, 0, 127)?;
            bits.set_bool(self.transmit_program_change);
            bits.set_u8::<7>(self.program_change, 0, 127)?;
            bits.set_bool(self.transmit_level);
            bits.set_u8::<7>(self.level, 0, 127)?;
            bits.set_bool(self.transmit_pan);
            bits.set_u8::<7>(self.pan.into(), 0, 127)?;
            bits.set_bool(self.transmit_course_tune);
            bits.set_u8::<7>(self.course_tune_semitones.into(), 16, 112)?;
            bits.set_bool(self.transmit_fine_tine);
            bits.set_u8::<7>(self.fine_tune_percent.into(), 14, 114)?;
            bits.set_bool(self.transmit_mono_poly);
            bits.set_u8::<2>(self.mono_poly.into(), 0, 1)?;
            bits.set_bool(self.transmit_portamento);
            bits.set_bool(self.portamento_switch);
            bits.set_bool(self.transmit_portamento_time);
            bits.set_u8::<7>(self.portamento_time, 0, 127)?;
            bits.set_bool(self.transmit_cutoff);
            bits.set_u8::<7>(self.cutoff.into(), 0, 127)?;
            bits.set_bool(self.transmit_resonance);
            bits.set_u8::<7>(self.resonance.into(), 0, 127)?;
            bits.set_bool(self.transmit_attack_time);
            bits.set_u8::<7>(self.attack_time.into(), 0, 127)?;
            bits.set_bool(self.transmit_decay_time);
            bits.set_u8::<7>(self.decay_time.into(), 0, 127)?;
            bits.set_bool(self.transmit_release_time);
            bits.set_u8::<7>(self.release_time.into(), 0, 127)?;
            bits.set_bool(self.transmit_pitch_bend_range);
            bits.set_u8::<6>(self.pitch_bend_range_semitones, 0, 48)?;
            bits.set_bool(self.transmit_modulation_depth);
            bits.set_u8::<7>(self.modulation_depth, 0, 127)?;
            bits.set_bool(self.transmit_chorus_level);
            bits.set_u8::<7>(self.chorus_level, 0, 127)?;
            bits.set_bool(self.transmit_reverb_level);
            bits.set_u8::<7>(self.reverb_level, 0, 127)?;
            bits.set_bool(self.transmit_control_change_1);
            bits.set_u8::<7>(self.control_change_1_number, 0, 127)?;
            bits.set_u8::<7>(self.control_change_1_value, 0, 127)?;
            bits.set_bool(self.transmit_control_change_2);
            bits.set_u8::<7>(self.control_change_2_number, 0, 127)?;
            bits.set_u8::<7>(self.control_change_2_value, 0, 127)?;
            bits.set_bool(self.s1);
            bits.set_bool(self.s2);
            bits.set_bits(&self.unused);
            Ok(())
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let range_lower: PianoKey = data.get_u8::<7>(0, 87)?.into();
            let range_upper = data.get_u8::<7>(range_lower.into(), 87)?.into();
            let velocity_range_lower = data.get_u8::<7>(1, 127)?;
            let velocity_range_upper = data.get_u8::<7>(1, 127)?;
            let velocity_sensitivity = data.get_u8::<7>(1, 127)?.into();
            let velocity_max = data.get_u8::<7>(1, 127)?;
            let transpose = data.get_u8::<7>(16, 112)?.into();
            let enable = data.get_bool();
            let damper = data.get_bool();
            let fc1 = data.get_bool();
            let fc2 = data.get_bool();
            let modulation = data.get_bool();
            let bender = data.get_bool();
            let control_mfx_switch = data.get_bool();
            let mut control_slider = HashMap::new();
            for key in Layer::iter() {
                control_slider.insert(key, data.get_bool());
            }
            Ok(Self {
                range_lower,
                range_upper,
                velocity_range_lower,
                velocity_range_upper,
                velocity_sensitivity,
                velocity_max,
                transpose,
                enable,
                damper,
                fc1,
                fc2,
                modulation,
                bender,
                control_mfx_switch,
                control_slider,
                transmit_port: data.get_u8::<3>(0, 4)?.into(),
                transmit_channel: data.get_u8::<4>(0, 15)?.into(),
                transmit_bank_select_msb: data.get_bool(),
                bank_select_msb: data.get_u8::<7>(0, 127)?,
                transmit_bank_select_lsb: data.get_bool(),
                bank_select_lsb: data.get_u8::<7>(0, 127)?,
                transmit_program_change: data.get_bool(),
                program_change: data.get_u8::<7>(0, 127)?,
                transmit_level: data.get_bool(),
                level: data.get_u8::<7>(0, 127)?.into(),
                transmit_pan: data.get_bool(),
                pan: data.get_u8::<7>(0, 127)?.into(),
                transmit_course_tune: data.get_bool(),
                course_tune_semitones: data.get_u8::<7>(16, 112)?.into(),
                transmit_fine_tine: data.get_bool(),
                fine_tune_percent: data.get_u8::<7>(14, 114)?.into(),
                transmit_mono_poly: data.get_bool(),
                mono_poly: data.get_u8::<2>(0, 1)?.into(),
                transmit_portamento: data.get_bool(),
                portamento_switch: data.get_bool(),
                transmit_portamento_time: data.get_bool(),
                portamento_time: data.get_u8::<7>(0, 127)?,
                transmit_cutoff: data.get_bool(),
                cutoff: data.get_u8::<7>(0, 127)?.into(),
                transmit_resonance: data.get_bool(),
                resonance: data.get_u8::<7>(0, 127)?.into(),
                transmit_attack_time: data.get_bool(),
                attack_time: data.get_u8::<7>(0, 127)?.into(),
                transmit_decay_time: data.get_bool(),
                decay_time: data.get_u8::<7>(0, 127)?.into(),
                transmit_release_time: data.get_bool(),
                release_time: data.get_u8::<7>( 0, 127)?.into(),
                transmit_pitch_bend_range: data.get_bool(),
                pitch_bend_range_semitones: data.get_u8::<6>(0, 48)?,
                transmit_modulation_depth: data.get_bool(),
                modulation_depth: data.get_u8::<7>(0, 127)?,
                transmit_chorus_level: data.get_bool(),
                chorus_level: data.get_u8::<7>(0, 127)?,
                transmit_reverb_level: data.get_bool(),
                reverb_level: data.get_u8::<7>(0, 127)?,
                transmit_control_change_1: data.get_bool(),
                control_change_1_number: data.get_u8::<7>(0, 127)?,
                control_change_1_value: data.get_u8::<7>(0, 127)?,
                transmit_control_change_2: data.get_bool(),
                control_change_2_number: data.get_u8::<7>(0, 127)?,
                control_change_2_value: data.get_u8::<7>(0, 127)?,
                s1: data.get_bool(),
                s2: data.get_bool(),
                unused: data.get_bits()
            })
        })
    }
}

impl LayerRanges for ExternalLayer {
    fn get_range_upper(&self) -> PianoKey {
        self.range_upper
    }

    fn get_range_lower(&self) -> PianoKey {
        self.range_lower
    }

    fn get_velocity_upper(&self) -> u8 {
        self.velocity_range_upper
    }

    fn get_velocity_lower(&self) -> u8 {
        self.velocity_range_lower
    }
}
