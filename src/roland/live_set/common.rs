use std::fmt::Debug;

use crate::bits::{Bits, BitStream};
use crate::bytes::{Bytes, BytesError, StructuredJson};
use crate::json::serialize_chars_as_string;
use crate::roland::{validate, in_range};

use super::super::{max, in_range_u16};

#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    #[serde(with = "serialize_chars_as_string")]
    name: [char; 16], // 32-127 (ascii)
    voice_reserve: [u8; 16], // index=channel, each max 64 (0-63, full)
    live_set_tempo: u16, // 10-500
    fc1_assign: u8, // max 144 (OFF, CC00 - CC127, BEND-UP, BEND-DOWN, AFTERTOUCH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, RHY PLY/STP, SONG PLY/STP, SONG RESET, MFX1 SW, MFX2 SW, MFX1 CONTROL, MFX2 CONTROL, ROTARY SPEED, SOUND FOCUS VALUE)
    fc2_assign: u8, // max 144 (OFF, CC00 - CC127, BEND-UP, BEND-DOWN, AFTERTOUCH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, RHY PLY/STP, SONG PLY/STP, SONG RESET, MFX1 SW, MFX2 SW, MFX1 CONTROL, MFX2 CONTROL, ROTARY SPEED, SOUND FOCUS VALUE)
    sound_focus_switch: bool,
    sound_focus_type: u8, // max 31 (OFF, PIANO TYPE1, PIANO TYPE2, E.PIANO TYPE, SOUND LIFT, ENHANCER, MID BOOST)
    sound_focus_value: u8, // max 127
    s1_assign: u8, // max 17 (OFF, COUPLE+1OCT, COUPLE-1OCT, COUPLE+2OCT, COUPLE-2OCT, COUPLE+5TH, COUPLE-4TH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, SONG PLY/STP, SONG RESET, SONG BWD, SONG FWD, MFX1 SW, MFX2 SW, ROTARY SPEED)
    s2_assign: u8, // max 17 (OFF, COUPLE+1OCT, COUPLE-1OCT, COUPLE+2OCT, COUPLE-2OCT, COUPLE+5TH, COUPLE-4TH, OCT-UP, OCT-DOWN, START/STOP, TAP-TEMPO, SONG PLY/STP, SONG RESET, SONG BWD, SONG FWD, MFX1 SW, MFX2 SW, ROTARY SPEED)
    s1_state: bool,
    s2_state: bool,
    eq_settings: Bits<68>, //TODO these are well defined by the 700NX midi implementation, but CBF doing the boilerplate rn
    key_touch_velocity: u8, // max 127 (REAL, 1-127)
    key_touch_curve_type: u8, // 1-5 (SUPER LIGHT, LIGHT, MEDIUM, HEAVY, SUPER HEAVY)
    key_touch_curve_offset: u8, //TODO spec is wrong: 54-73 (-10 - +9)
    key_touch_velocity_delay_sense: u8, // 1-127 (-63 - +63)
    key_touch_velocity_key_follow: u8, // 1-127 (-63 - +63)
    key_off_position: bool, // (STANDARD, DEEP)
    slider_select: bool, // (LAYER LEVEL, CONTROL)
    slider_assign: [u8; 4], // index=layer (UPPER1, UPPER2, LOWER1, LOWER2) each 0-133 (OFF, CC00 - CC127, BEND-UP, BEND-DOWN, AFTERTOUCH, MFX1 CONTROL, MFX2 CONTROL)
    split_switch_internal: bool,
    split_switch_external: bool,
    harmonic_bar_assign: [u8; 4*2], // index=reverse layer & on/off (LOWER2:ON, LOWER2:OFF, LOWER1:ON, LOWER1:OFF, UPPER2:ON, UPPER2:OFF, UPPER1:ON, UPPER1:OFF) each 1-9 (16',5-1/3',8',4',2-2/3',1-3/5',2',1-1/3',1')
    mfx_control_destination: u8, // max 3 (UPPER1, UPPER2, LOWER1, LOWER2)
    unused: Bits<7>
}

impl Bytes<56> for Common {
    fn to_bytes(&self) -> Box<[u8; Self::BYTE_SIZE]> {
        BitStream::write_fixed(|bits| {
            for value in self.name {
                bits.set_char(value);//TODO validate?
            }
            for value in self.voice_reserve {
                bits.set_u8::<7>(max(value, 64));
            }
            bits.set_u16::<9>(in_range_u16(self.live_set_tempo, 10, 500));
            bits.set_u8::<8>(max(self.fc1_assign, 144));
            bits.set_u8::<8>(max(self.fc2_assign, 144));
            bits.set_bool(self.sound_focus_switch);
            bits.set_u8::<5>(max(self.sound_focus_type, 31));
            bits.set_u8::<7>(self.sound_focus_value);
            bits.set_u8::<5>(max(self.s1_assign, 17));
            bits.set_u8::<5>(max(self.s2_assign, 17));
            bits.set_bool(self.s1_state);
            bits.set_bool(self.s2_state);
            bits.set_bits(&self.eq_settings);
            bits.set_u8::<7>(self.key_touch_velocity);
            bits.set_u8::<3>(in_range(self.key_touch_curve_type, 1, 5));
            bits.set_u8::<5>(self.key_touch_curve_offset); //TODO constrain with correct spec
            bits.set_u8::<7>(in_range(self.key_touch_velocity_delay_sense, 1, 127));
            bits.set_u8::<7>(in_range(self.key_touch_velocity_key_follow, 1, 127));
            bits.set_bool(self.key_off_position);
            bits.set_bool(self.slider_select);
            for value in self.slider_assign {
                bits.set_u8::<8>(max(value, 133));
            }
            bits.set_bool(self.split_switch_internal);
            bits.set_bool(self.split_switch_external);
            for value in self.harmonic_bar_assign {
                bits.set_u8::<4>(in_range(value, 1, 9));
            }
            bits.set_u8::<2>(self.mfx_control_destination);
            bits.set_bits(&self.unused);
        })
    }

    fn from_bytes(bytes: Box<[u8; Self::BYTE_SIZE]>) -> Result<Self, BytesError> where Self: Sized {
        BitStream::read_fixed(bytes, |data| {
            let mut name = [char::default(); 16];
            for i in 0..name.len() {
                name[i] = validate(data.get_char())?;
            }
            let mut voice_reserve = [0; 16];
            for i in 0..voice_reserve.len() {
                voice_reserve[i] = data.get_u8::<7>();
            }
            let live_set_tempo = data.get_u16::<9>();
            let fc1_assign = data.get_u8::<8>();
            let fc2_assign = data.get_u8::<8>();
            let sound_focus_switch = data.get_bool();
            let sound_focus_type = data.get_u8::<5>();
            let sound_focus_value = data.get_u8::<7>();
            let s1_assign = data.get_u8::<5>();
            let s2_assign = data.get_u8::<5>();
            let s1_state = data.get_bool();
            let s2_state = data.get_bool();
            let eq_settings = data.get_bits();
            let key_touch_velocity = data.get_u8::<7>();
            let key_touch_curve_type = data.get_u8::<3>();
            let key_touch_curve_offset = data.get_u8::<5>();
            let key_touch_velocity_delay_sense = data.get_u8::<7>();
            let key_touch_velocity_key_follow = data.get_u8::<7>();
            let key_off_position = data.get_bool();
            let slider_select = data.get_bool();
            let mut slider_assign = [0; 4];
            for i in 0..slider_assign.len() {
                slider_assign[i] = data.get_u8::<8>();
            }
            let split_switch_internal = data.get_bool();
            let split_switch_external = data.get_bool();
            let mut harmonic_bar_assign = [0; 4 * 2];
            for i in 0..harmonic_bar_assign.len() {
                harmonic_bar_assign[i] = data.get_u8::<4>();
            }
            Ok(Self {
                name,
                voice_reserve,
                live_set_tempo,
                fc1_assign,
                fc2_assign,
                sound_focus_switch,
                sound_focus_type,
                sound_focus_value,
                s1_assign,
                s2_assign,
                s1_state,
                s2_state,
                eq_settings,
                key_touch_velocity,
                key_touch_curve_type,
                key_touch_curve_offset,
                key_touch_velocity_delay_sense,
                key_touch_velocity_key_follow,
                key_off_position,
                slider_select,
                slider_assign,
                split_switch_internal,
                split_switch_external,
                harmonic_bar_assign,
                mfx_control_destination: data.get_u8::<2>(),
                unused: data.get_bits()
            })
        })
    }

    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Self {
        Self::from_json(structured_json.to_single_json())
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Error serializing JSON")
    }

    fn from_json(json: String) -> Self {
        serde_json::from_str(&json).expect("Error deserializing JSON")
    }
}

impl Common {
    pub fn name_string(&self) -> String {
        self.name.iter().collect()
    }
}