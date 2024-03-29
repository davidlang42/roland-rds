use schemars::{JsonSchema, schema::Schema};
use serde_json::Value;
use serde::{de, Serialize, Deserialize};
use validator::Validate;

use crate::json::{type_name_pretty, schema::{u16_schema, one_of_schema, enum_schema}, validation::out_of_range_err};

pub struct Tone {
    _number: u16,
    pub name: &'static str,
    pub msb: u8,
    pub lsb: u8,
    pub pc: u8
}

impl Tone {
    fn numbered_string(&self) -> String {
        format!("{}_{}", self._number, self.name)
    }
}

#[derive(Debug)]
pub struct ToneNumber(u16);

impl ToneNumber {
    pub fn details(&self) -> &Tone {
        let number = self.0 as usize;
        if number == 0 || number > TONE_LIST.len() {
            panic!("Invalid tone number")
        }
        &TONE_LIST[number - 1]
    }

    pub fn find(msb: u8, lsb: u8, pc: u8) -> Option<Self> {
        for (i, tone) in TONE_LIST.iter().enumerate() {
            if tone.msb == msb && tone.lsb == lsb && tone.pc == pc {
                return Some(Self(i as u16 + 1));
            }
        }
        None
    }

    pub fn as_piano_tone(&self) -> Option<PianoToneNumber> {
        let max = PianoToneNumber::piano_tones_list().len() as u16;
        if self.0 <= max {
            Some(PianoToneNumber::from(self.0 as u8 - 1))
        } else {
            None
        }
    }
}

impl From<u16> for ToneNumber {
    fn from(value: u16) -> Self {
        if value == 0 {
            panic!("Values for ToneNumber start at 1");
        }
        if value as usize > TONE_LIST.len() {
            panic!("Value ({}) exceeds maximum tone number ({})", value, TONE_LIST.len());
        }
        Self(value)
    }
}

impl Serialize for ToneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let s: String = self.details().numbered_string();
        s.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ToneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => {
                for (i, tone) in TONE_LIST.iter().enumerate() {
                    if s == tone.numbered_string() {
                        return Ok(Self(i as u16 + 1));
                    }
                }
                Err(de::Error::custom(format!("String is not a valid Tone: {}", s)))
            },
            Value::Number(number) => {
                if let Some(n) = number.as_u64() {
                    if n < 1 || n > TONE_LIST.len() as u64 {
                        Err(de::Error::custom(format!("Number is a out of valid range (1-{}): {}", TONE_LIST.len(), number)))
                    } else {
                        Ok(Self(n as u16))
                    }
                } else {
                    Err(de::Error::custom(format!("Number is a not a positive integer: {}", number)))
                }
            },
            _ => Err(de::Error::custom(format!("Expected string or number")))
        }
    }
}

impl Validate for ToneNumber {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let max = TONE_LIST.len() as u16;
        if self.0 < 1 || self.0 > max {
            Err(out_of_range_err("0", &1, &max))
        } else {
            Ok(())
        }
    }
}

impl JsonSchema for ToneNumber {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        tone_schema(TONE_LIST.iter())
    }
}

fn tone_schema<'a, I: Iterator<Item = &'a Tone>>(allowed_tones: I) -> Schema {
    let mut names = Vec::new();
    for tone in allowed_tones {
        names.push(tone.numbered_string());
    }
    let max = names.len() as u16;
    one_of_schema(vec![
        enum_schema(names),
        u16_schema(1, max)
    ])
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PianoToneNumber(u8); // 0-8 (1-9)

impl PianoToneNumber {
    const PIANO_TONE_MSB: u8 = 114; // the dedicated piano tones appear at the start of the tones list and use this MSB specifically

    pub fn details(&self) -> &Tone {
        let number = self.0 as usize;
        if number == 0 || number > TONE_LIST.len() {
            panic!("Invalid tone number")
        }
        let tone = &TONE_LIST[number - 1];
        if tone.msb != Self::PIANO_TONE_MSB {
            panic!("Tone number is not a piano tone")
        }
        tone
    }

    fn piano_tones_list() -> Vec<&'static Tone> {
        let mut tones = Vec::new();
        for tone in &TONE_LIST {
            if tone.msb != Self::PIANO_TONE_MSB {
                break;
            }
            tones.push(tone);
        }
        tones
    }
}

impl From<u8> for PianoToneNumber {
    fn from(value: u8) -> Self {
        Self(value + 1)
    }
}

impl Into<u8> for PianoToneNumber {
    fn into(self) -> u8 {
        self.0 - 1
    }
}

impl Default for PianoToneNumber {
    fn default() -> Self {
        Self::from(0)
    }
}

impl Serialize for PianoToneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let s: String = self.details().numbered_string();
        s.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PianoToneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value: Value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => {
                for (i, tone) in Self::piano_tones_list().into_iter().enumerate() {
                    if s == tone.numbered_string() {
                        return Ok(Self(i as u8 + 1));
                    }
                }
                Err(de::Error::custom(format!("String is not a valid Piano Tone: {}", s)))
            },
            Value::Number(number) => {
                if let Some(n) = number.as_u64() {
                    let piano_tones_list = Self::piano_tones_list();
                    if n < 1 || n > piano_tones_list.len() as u64 {
                        Err(de::Error::custom(format!("Number is a out of valid range (1-{}): {}", piano_tones_list.len(), number)))
                    } else {
                        Ok(Self(n as u8))
                    }
                } else {
                    Err(de::Error::custom(format!("Number is a not a positive integer: {}", number)))
                }
            },
            _ => Err(de::Error::custom(format!("Expected string or number")))
        }
    }
}

impl JsonSchema for PianoToneNumber {
    fn schema_name() -> String {
        type_name_pretty::<Self>().into()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        tone_schema(Self::piano_tones_list().into_iter())
    }
}

impl Validate for PianoToneNumber {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let max = Self::piano_tones_list().len() as u8;
        if self.0 < 1 || self.0 > max {
            Err(out_of_range_err("0", &1, &max))
        } else {
            Ok(())
        }
    }
}

static TONE_LIST: [Tone; 942] = [
    Tone { _number: 1, name: "ConcertGrand", msb: 114, lsb: 0, pc: 0 },
    Tone { _number: 2, name: "Honky-tonk1", msb: 114, lsb: 0, pc: 1 },
    Tone { _number: 3, name: "Concert Mono", msb: 114, lsb: 0, pc: 2 },
    Tone { _number: 4, name: "St.Piano 1", msb: 87, lsb: 0, pc: 0 },
    Tone { _number: 5, name: "St.Piano 2", msb: 87, lsb: 0, pc: 1 },
    Tone { _number: 6, name: "St.Piano 3", msb: 87, lsb: 0, pc: 2 },
    Tone { _number: 7, name: "St.Piano 4", msb: 87, lsb: 0, pc: 3 },
    Tone { _number: 8, name: "St.Piano 5", msb: 87, lsb: 0, pc: 4 },
    Tone { _number: 9, name: "Brite Piano", msb: 87, lsb: 0, pc: 5 },
    Tone { _number: 10, name: "Stage Piano", msb: 87, lsb: 0, pc: 6 },
    Tone { _number: 11, name: "Honky Tonk", msb: 87, lsb: 0, pc: 7 },
    Tone { _number: 12, name: "Pop Piano 1", msb: 87, lsb: 0, pc: 8 },
    Tone { _number: 13, name: "Pop Piano 2", msb: 87, lsb: 0, pc: 9 },
    Tone { _number: 14, name: "Pop Piano 3", msb: 87, lsb: 0, pc: 10 },
    Tone { _number: 15, name: "Piano 1", msb: 121, lsb: 0, pc: 0 },
    Tone { _number: 16, name: "Piano 1 w", msb: 121, lsb: 1, pc: 0 },
    Tone { _number: 17, name: "European Pf", msb: 121, lsb: 2, pc: 0 },
    Tone { _number: 18, name: "Piano 2", msb: 121, lsb: 0, pc: 1 },
    Tone { _number: 19, name: "Piano 2 w", msb: 121, lsb: 1, pc: 1 },
    Tone { _number: 20, name: "Piano 3", msb: 121, lsb: 0, pc: 2 },
    Tone { _number: 21, name: "Piano 3 w", msb: 121, lsb: 1, pc: 2 },
    Tone { _number: 22, name: "Honky-tonk", msb: 121, lsb: 0, pc: 3 },
    Tone { _number: 23, name: "Honky-tonk w", msb: 121, lsb: 1, pc: 3 },
    Tone { _number: 24, name: "TINE EP", msb: 89, lsb: 64, pc: 4 },
    Tone { _number: 25, name: "REED EP", msb: 89, lsb: 65, pc: 4 },
    Tone { _number: 26, name: "Stage EP 1", msb: 87, lsb: 0, pc: 11 },
    Tone { _number: 27, name: "Stage EP 2", msb: 87, lsb: 0, pc: 12 },
    Tone { _number: 28, name: "Stage EP Trm", msb: 87, lsb: 0, pc: 13 },
    Tone { _number: 29, name: "Tremolo EP 1", msb: 87, lsb: 0, pc: 14 },
    Tone { _number: 30, name: "E.Piano 3", msb: 87, lsb: 0, pc: 15 },
    Tone { _number: 31, name: "E.Piano 4", msb: 87, lsb: 0, pc: 16 },
    Tone { _number: 32, name: "E.Piano 5", msb: 87, lsb: 0, pc: 17 },
    Tone { _number: 33, name: "E.Piano 6", msb: 87, lsb: 0, pc: 18 },
    Tone { _number: 34, name: "E.Piano 7", msb: 87, lsb: 0, pc: 19 },
    Tone { _number: 35, name: "E.Piano 8", msb: 87, lsb: 0, pc: 20 },
    Tone { _number: 36, name: "Dyno EP", msb: 87, lsb: 0, pc: 21 },
    Tone { _number: 37, name: "Dyno EP Trm", msb: 87, lsb: 0, pc: 22 },
    Tone { _number: 38, name: "Tremolo EP 2", msb: 87, lsb: 0, pc: 23 },
    Tone { _number: 39, name: "Back2the60s", msb: 87, lsb: 0, pc: 24 },
    Tone { _number: 40, name: "Tine EP", msb: 87, lsb: 0, pc: 25 },
    Tone { _number: 41, name: "SA EP 1", msb: 87, lsb: 0, pc: 26 },
    Tone { _number: 42, name: "SA EP 2", msb: 87, lsb: 0, pc: 27 },
    Tone { _number: 43, name: "Psy EP", msb: 87, lsb: 0, pc: 28 },
    Tone { _number: 44, name: "Hit EP", msb: 87, lsb: 65, pc: 2 },
    Tone { _number: 45, name: "Wurly EP", msb: 87, lsb: 0, pc: 29 },
    Tone { _number: 46, name: "Wurly EP Trm", msb: 87, lsb: 0, pc: 30 },
    Tone { _number: 47, name: "Curly Wurly", msb: 87, lsb: 0, pc: 31 },
    Tone { _number: 48, name: "Super Wurly", msb: 87, lsb: 0, pc: 32 },
    Tone { _number: 49, name: "EP Legend 3", msb: 87, lsb: 0, pc: 33 },
    Tone { _number: 50, name: "EP Belle", msb: 87, lsb: 0, pc: 34 },
    Tone { _number: 51, name: "80's EP", msb: 87, lsb: 0, pc: 35 },
    Tone { _number: 52, name: "FM EP 1", msb: 87, lsb: 0, pc: 36 },
    Tone { _number: 53, name: "FM EP 2", msb: 87, lsb: 0, pc: 37 },
    Tone { _number: 54, name: "Sinus EP", msb: 87, lsb: 0, pc: 38 },
    Tone { _number: 55, name: "Spirit Tines", msb: 87, lsb: 0, pc: 39 },
    Tone { _number: 56, name: "E.Piano 1", msb: 121, lsb: 0, pc: 4 },
    Tone { _number: 57, name: "St.Soft EP", msb: 121, lsb: 1, pc: 4 },
    Tone { _number: 58, name: "EP Legend 1", msb: 121, lsb: 2, pc: 4 },
    Tone { _number: 59, name: "Wurly", msb: 121, lsb: 3, pc: 4 },
    Tone { _number: 60, name: "E.Piano 2", msb: 121, lsb: 0, pc: 5 },
    Tone { _number: 61, name: "Detuned EP", msb: 121, lsb: 1, pc: 5 },
    Tone { _number: 62, name: "St.FM EP", msb: 121, lsb: 2, pc: 5 },
    Tone { _number: 63, name: "EP Legend 2", msb: 121, lsb: 3, pc: 5 },
    Tone { _number: 64, name: "EP Phase", msb: 121, lsb: 4, pc: 5 },
    Tone { _number: 65, name: "E.Grand 1", msb: 87, lsb: 65, pc: 21 },
    Tone { _number: 66, name: "E.Grand 2", msb: 87, lsb: 65, pc: 22 },
    Tone { _number: 67, name: "E.Grand 3", msb: 87, lsb: 65, pc: 23 },
    Tone { _number: 68, name: "Clav", msb: 121, lsb: 0, pc: 7 },
    Tone { _number: 69, name: "Clav 2", msb: 87, lsb: 0, pc: 41 },
    Tone { _number: 70, name: "Pulse Clav", msb: 121, lsb: 1, pc: 7 },
    Tone { _number: 71, name: "Pulse Clav 2", msb: 87, lsb: 0, pc: 42 },
    Tone { _number: 72, name: "Sweepin Clav", msb: 87, lsb: 0, pc: 43 },
    Tone { _number: 73, name: "Analog Clav", msb: 87, lsb: 0, pc: 44 },
    Tone { _number: 74, name: "Biting Clav", msb: 87, lsb: 0, pc: 45 },
    Tone { _number: 75, name: "Pulse Clv St", msb: 87, lsb: 0, pc: 46 },
    Tone { _number: 76, name: "Natural Hps.", msb: 87, lsb: 66, pc: 15 },
    Tone { _number: 77, name: "Harpsichord", msb: 121, lsb: 0, pc: 6 },
    Tone { _number: 78, name: "Harpsichord2", msb: 87, lsb: 0, pc: 40 },
    Tone { _number: 79, name: "Coupled Hps", msb: 121, lsb: 1, pc: 6 },
    Tone { _number: 80, name: "Harpsi w", msb: 121, lsb: 2, pc: 6 },
    Tone { _number: 81, name: "Harpsi o", msb: 121, lsb: 3, pc: 6 },
    Tone { _number: 82, name: "Vibraphone", msb: 121, lsb: 0, pc: 11 },
    Tone { _number: 83, name: "Vibraphone 2", msb: 87, lsb: 0, pc: 67 },
    Tone { _number: 84, name: "VibraphoneTr", msb: 87, lsb: 0, pc: 68 },
    Tone { _number: 85, name: "Vibraphone w", msb: 121, lsb: 1, pc: 11 },
    Tone { _number: 86, name: "Tremolo Vib", msb: 87, lsb: 0, pc: 69 },
    Tone { _number: 87, name: "Jazz Vib", msb: 87, lsb: 4, pc: 2 },
    Tone { _number: 88, name: "Marimba", msb: 121, lsb: 0, pc: 12 },
    Tone { _number: 89, name: "Marimba 2", msb: 87, lsb: 0, pc: 70 },
    Tone { _number: 90, name: "Marimba 3", msb: 87, lsb: 0, pc: 71 },
    Tone { _number: 91, name: "Marimba w", msb: 121, lsb: 1, pc: 12 },
    Tone { _number: 92, name: "BsMarimba 1", msb: 87, lsb: 4, pc: 3 },
    Tone { _number: 93, name: "BsMarimba 2", msb: 87, lsb: 4, pc: 4 },
    Tone { _number: 94, name: "Xylophone", msb: 121, lsb: 0, pc: 13 },
    Tone { _number: 95, name: "Xylophone 2", msb: 87, lsb: 0, pc: 73 },
    Tone { _number: 96, name: "Xylophone 3", msb: 87, lsb: 0, pc: 74 },
    Tone { _number: 97, name: "Ethno Keys", msb: 87, lsb: 0, pc: 75 },
    Tone { _number: 98, name: "Celesta", msb: 121, lsb: 0, pc: 8 },
    Tone { _number: 99, name: "Glockenspiel", msb: 121, lsb: 0, pc: 9 },
    Tone { _number: 100, name: "Music Box", msb: 121, lsb: 0, pc: 10 },
    Tone { _number: 101, name: "Music Box 2", msb: 87, lsb: 0, pc: 54 },
    Tone { _number: 102, name: "Kalimba", msb: 121, lsb: 0, pc: 108 },
    Tone { _number: 103, name: "Kalimbells", msb: 87, lsb: 0, pc: 56 },
    Tone { _number: 104, name: "Steel Drums", msb: 121, lsb: 0, pc: 114 },
    Tone { _number: 105, name: "Steel Drums2", msb: 87, lsb: 0, pc: 72 },
    Tone { _number: 106, name: "Soft StlDrm", msb: 87, lsb: 4, pc: 1 },
    Tone { _number: 107, name: "FM Sparkles", msb: 87, lsb: 0, pc: 47 },
    Tone { _number: 108, name: "FM Syn Bell", msb: 87, lsb: 0, pc: 48 },
    Tone { _number: 109, name: "FM Heaven", msb: 87, lsb: 0, pc: 49 },
    Tone { _number: 110, name: "D50 Fantasy", msb: 87, lsb: 0, pc: 50 },
    Tone { _number: 111, name: "D50 Bell", msb: 87, lsb: 0, pc: 51 },
    Tone { _number: 112, name: "Dreaming Bel", msb: 87, lsb: 0, pc: 52 },
    Tone { _number: 113, name: "Analog Bell", msb: 87, lsb: 0, pc: 53 },
    Tone { _number: 114, name: "Music Bells", msb: 87, lsb: 0, pc: 55 },
    Tone { _number: 115, name: "Bell 1", msb: 87, lsb: 0, pc: 57 },
    Tone { _number: 116, name: "Bell 2", msb: 87, lsb: 0, pc: 58 },
    Tone { _number: 117, name: "Org Bell", msb: 87, lsb: 4, pc: 0 },
    Tone { _number: 118, name: "Crystal", msb: 121, lsb: 0, pc: 98 },
    Tone { _number: 119, name: "Tinkle Bell", msb: 121, lsb: 0, pc: 112 },
    Tone { _number: 120, name: "Icy Keys", msb: 87, lsb: 0, pc: 59 },
    Tone { _number: 121, name: "Toy Box", msb: 87, lsb: 0, pc: 60 },
    Tone { _number: 122, name: "Dreambell", msb: 87, lsb: 0, pc: 65 },
    Tone { _number: 123, name: "Sine Mallet", msb: 87, lsb: 4, pc: 5 },
    Tone { _number: 124, name: "Syn Mallet", msb: 121, lsb: 1, pc: 98 },
    Tone { _number: 125, name: "TubularBells", msb: 121, lsb: 0, pc: 14 },
    Tone { _number: 126, name: "TubularBell2", msb: 87, lsb: 0, pc: 63 },
    Tone { _number: 127, name: "Church Bell", msb: 121, lsb: 1, pc: 14 },
    Tone { _number: 128, name: "Carillon", msb: 121, lsb: 2, pc: 14 },
    Tone { _number: 129, name: "Carillon 2", msb: 87, lsb: 0, pc: 61 },
    Tone { _number: 130, name: "Tower Bell", msb: 87, lsb: 0, pc: 62 },
    Tone { _number: 131, name: "Bell Ring", msb: 87, lsb: 0, pc: 64 },
    Tone { _number: 132, name: "GX Strings", msb: 87, lsb: 68, pc: 0 },
    Tone { _number: 133, name: "Mood Strings", msb: 87, lsb: 1, pc: 69 },
    Tone { _number: 134, name: "Slow Strings", msb: 121, lsb: 0, pc: 49 },
    Tone { _number: 135, name: "DecayStrings", msb: 87, lsb: 68, pc: 26 },
    Tone { _number: 136, name: "Strings", msb: 121, lsb: 0, pc: 48 },
    Tone { _number: 137, name: "Strings 2", msb: 87, lsb: 1, pc: 70 },
    Tone { _number: 138, name: "Strings 3", msb: 87, lsb: 1, pc: 71 },
    Tone { _number: 139, name: "Strings 4", msb: 87, lsb: 1, pc: 72 },
    Tone { _number: 140, name: "Strings 5", msb: 87, lsb: 1, pc: 73 },
    Tone { _number: 141, name: "Strings 6", msb: 87, lsb: 4, pc: 32 },
    Tone { _number: 142, name: "Stage Str 1", msb: 87, lsb: 1, pc: 74 },
    Tone { _number: 143, name: "Stage Str 2", msb: 87, lsb: 1, pc: 75 },
    Tone { _number: 144, name: "Pop Str", msb: 87, lsb: 1, pc: 76 },
    Tone { _number: 145, name: "Hall Strings", msb: 87, lsb: 4, pc: 33 },
    Tone { _number: 146, name: "Marc.Str", msb: 87, lsb: 1, pc: 77 },
    Tone { _number: 147, name: "StringsStacc", msb: 87, lsb: 1, pc: 78 },
    Tone { _number: 148, name: "Orchestra", msb: 121, lsb: 1, pc: 48 },
    Tone { _number: 149, name: "Oct Strings", msb: 121, lsb: 2, pc: 48 },
    Tone { _number: 150, name: "Orc.Unison 1", msb: 87, lsb: 1, pc: 85 },
    Tone { _number: 151, name: "Orc.Unison 2", msb: 87, lsb: 1, pc: 86 },
    Tone { _number: 152, name: "Full Orc", msb: 87, lsb: 4, pc: 34 },
    Tone { _number: 153, name: "Tremolo Str", msb: 121, lsb: 0, pc: 44 },
    Tone { _number: 154, name: "TapeStrings1", msb: 87, lsb: 1, pc: 81 },
    Tone { _number: 155, name: "TapeStrings2", msb: 87, lsb: 1, pc: 82 },
    Tone { _number: 156, name: "Hybrid Str", msb: 87, lsb: 1, pc: 83 },
    Tone { _number: 157, name: "Violin", msb: 121, lsb: 0, pc: 40 },
    Tone { _number: 158, name: "Violin 2", msb: 87, lsb: 1, pc: 84 },
    Tone { _number: 159, name: "Slow Violin", msb: 121, lsb: 1, pc: 40 },
    Tone { _number: 160, name: "Bright Vln", msb: 87, lsb: 4, pc: 30 },
    Tone { _number: 161, name: "Viola", msb: 121, lsb: 0, pc: 41 },
    Tone { _number: 162, name: "Cello", msb: 121, lsb: 0, pc: 42 },
    Tone { _number: 163, name: "Bright Vc", msb: 87, lsb: 4, pc: 31 },
    Tone { _number: 164, name: "Contrabass", msb: 121, lsb: 0, pc: 43 },
    Tone { _number: 165, name: "PizzicatoStr", msb: 121, lsb: 0, pc: 45 },
    Tone { _number: 166, name: "Pizz 1", msb: 87, lsb: 1, pc: 79 },
    Tone { _number: 167, name: "Pizz 2", msb: 87, lsb: 1, pc: 80 },
    Tone { _number: 168, name: "Harp", msb: 121, lsb: 0, pc: 46 },
    Tone { _number: 169, name: "Yang Qin", msb: 121, lsb: 1, pc: 46 },
    Tone { _number: 170, name: "Timpani", msb: 121, lsb: 0, pc: 47 },
    Tone { _number: 171, name: "Fiddle", msb: 121, lsb: 0, pc: 110 },
    Tone { _number: 172, name: "Cheezy Movie", msb: 87, lsb: 4, pc: 48 },
    Tone { _number: 173, name: "CalmChoirPad", msb: 87, lsb: 69, pc: 0 },
    Tone { _number: 174, name: "Soft Pad 1", msb: 87, lsb: 3, pc: 27 },
    Tone { _number: 175, name: "Soft Pad 2", msb: 87, lsb: 3, pc: 28 },
    Tone { _number: 176, name: "Soft Pad 3", msb: 87, lsb: 3, pc: 29 },
    Tone { _number: 177, name: "Soft Pad 4", msb: 87, lsb: 3, pc: 30 },
    Tone { _number: 178, name: "Soft Pad 5", msb: 87, lsb: 3, pc: 31 },
    Tone { _number: 179, name: "Soft Pad 6", msb: 87, lsb: 3, pc: 32 },
    Tone { _number: 180, name: "Soft Pad 7", msb: 87, lsb: 3, pc: 33 },
    Tone { _number: 181, name: "Soft Pad 8", msb: 87, lsb: 3, pc: 34 },
    Tone { _number: 182, name: "Soft Pad 9", msb: 87, lsb: 3, pc: 35 },
    Tone { _number: 183, name: "Soft Pad 10", msb: 87, lsb: 3, pc: 36 },
    Tone { _number: 184, name: "Dreamheaven", msb: 87, lsb: 3, pc: 15 },
    Tone { _number: 185, name: "Air Key 1", msb: 87, lsb: 3, pc: 16 },
    Tone { _number: 186, name: "Air Key 2", msb: 87, lsb: 3, pc: 20 },
    Tone { _number: 187, name: "Sweet Keys", msb: 87, lsb: 3, pc: 17 },
    Tone { _number: 188, name: "Soft Bell", msb: 87, lsb: 3, pc: 19 },
    Tone { _number: 189, name: "Oct Heaven", msb: 87, lsb: 3, pc: 21 },
    Tone { _number: 190, name: "Stacc Heaven", msb: 87, lsb: 3, pc: 22 },
    Tone { _number: 191, name: "DigitalDream", msb: 87, lsb: 3, pc: 23 },
    Tone { _number: 192, name: "Analog Dream", msb: 87, lsb: 3, pc: 24 },
    Tone { _number: 193, name: "Harp Pad", msb: 87, lsb: 3, pc: 25 },
    Tone { _number: 194, name: "Sitar Pad", msb: 87, lsb: 3, pc: 26 },
    Tone { _number: 195, name: "VintageStr 1", msb: 87, lsb: 3, pc: 37 },
    Tone { _number: 196, name: "VintageStr 2", msb: 87, lsb: 3, pc: 38 },
    Tone { _number: 197, name: "VintageStr 3", msb: 87, lsb: 3, pc: 39 },
    Tone { _number: 198, name: "VintageStr 4", msb: 87, lsb: 3, pc: 40 },
    Tone { _number: 199, name: "VintageStr 5", msb: 87, lsb: 3, pc: 41 },
    Tone { _number: 200, name: "VintageStr 6", msb: 87, lsb: 3, pc: 42 },
    Tone { _number: 201, name: "VintageStr 7", msb: 87, lsb: 3, pc: 43 },
    Tone { _number: 202, name: "JX Strings", msb: 87, lsb: 3, pc: 44 },
    Tone { _number: 203, name: "JP Strings 1", msb: 87, lsb: 3, pc: 45 },
    Tone { _number: 204, name: "JP Strings 2", msb: 87, lsb: 3, pc: 46 },
    Tone { _number: 205, name: "106 Strings", msb: 87, lsb: 3, pc: 47 },
    Tone { _number: 206, name: "PWM Str 1", msb: 87, lsb: 3, pc: 48 },
    Tone { _number: 207, name: "PWM Str 2", msb: 87, lsb: 3, pc: 49 },
    Tone { _number: 208, name: "PWM Str 3", msb: 87, lsb: 4, pc: 80 },
    Tone { _number: 209, name: "Fading Str", msb: 87, lsb: 3, pc: 50 },
    Tone { _number: 210, name: "ParadisePad", msb: 87, lsb: 3, pc: 51 },
    Tone { _number: 211, name: "80s Strings", msb: 87, lsb: 3, pc: 52 },
    Tone { _number: 212, name: "Stringship", msb: 87, lsb: 3, pc: 53 },
    Tone { _number: 213, name: "Airy Pad", msb: 87, lsb: 3, pc: 54 },
    Tone { _number: 214, name: "Neo RS-202", msb: 87, lsb: 3, pc: 55 },
    Tone { _number: 215, name: "Sawtooth Str", msb: 87, lsb: 3, pc: 56 },
    Tone { _number: 216, name: "Pulse Pad", msb: 87, lsb: 3, pc: 57 },
    Tone { _number: 217, name: "Hollow Pad 1", msb: 87, lsb: 3, pc: 58 },
    Tone { _number: 218, name: "WarmHeaven 1", msb: 87, lsb: 3, pc: 59 },
    Tone { _number: 219, name: "WarmHeaven 2", msb: 87, lsb: 3, pc: 60 },
    Tone { _number: 220, name: "Heaven Key", msb: 87, lsb: 3, pc: 10 },
    Tone { _number: 221, name: "Heaven Pad 1", msb: 87, lsb: 3, pc: 11 },
    Tone { _number: 222, name: "Heaven Pad 2", msb: 87, lsb: 3, pc: 12 },
    Tone { _number: 223, name: "Heaven Pad 3", msb: 87, lsb: 3, pc: 61 },
    Tone { _number: 224, name: "Heaven Pad 4", msb: 87, lsb: 3, pc: 62 },
    Tone { _number: 225, name: "FineWinePad1", msb: 87, lsb: 3, pc: 63 },
    Tone { _number: 226, name: "FineWinePad2", msb: 87, lsb: 3, pc: 64 },
    Tone { _number: 227, name: "5th Pad 1", msb: 87, lsb: 3, pc: 65 },
    Tone { _number: 228, name: "5th Pad 2", msb: 87, lsb: 3, pc: 66 },
    Tone { _number: 229, name: "Nu Epic Pad", msb: 87, lsb: 3, pc: 67 },
    Tone { _number: 230, name: "Angelis Pad", msb: 87, lsb: 3, pc: 68 },
    Tone { _number: 231, name: "TrnsSweepPad", msb: 87, lsb: 3, pc: 69 },
    Tone { _number: 232, name: "Giant Sweep", msb: 87, lsb: 3, pc: 70 },
    Tone { _number: 233, name: "Voyager", msb: 87, lsb: 3, pc: 71 },
    Tone { _number: 234, name: "Digital Pad", msb: 87, lsb: 3, pc: 72 },
    Tone { _number: 235, name: "NuSoundtrack", msb: 87, lsb: 3, pc: 73 },
    Tone { _number: 236, name: "Xadecimal", msb: 87, lsb: 3, pc: 74 },
    Tone { _number: 237, name: "PanninFormnt", msb: 87, lsb: 3, pc: 75 },
    Tone { _number: 238, name: "Fairy's Song", msb: 87, lsb: 3, pc: 76 },
    Tone { _number: 239, name: "Atmospherics", msb: 87, lsb: 3, pc: 77 },
    Tone { _number: 240, name: "Strobe Pad", msb: 87, lsb: 3, pc: 78 },
    Tone { _number: 241, name: "StrobeBell 1", msb: 87, lsb: 3, pc: 79 },
    Tone { _number: 242, name: "StrobeBell 2", msb: 87, lsb: 3, pc: 80 },
    Tone { _number: 243, name: "Flying Pad 1", msb: 87, lsb: 3, pc: 81 },
    Tone { _number: 244, name: "Flying Pad 2", msb: 87, lsb: 3, pc: 82 },
    Tone { _number: 245, name: "Flying Pad 3", msb: 87, lsb: 3, pc: 83 },
    Tone { _number: 246, name: "Flying Pad 4", msb: 87, lsb: 3, pc: 84 },
    Tone { _number: 247, name: "Flying Pad 5", msb: 87, lsb: 3, pc: 85 },
    Tone { _number: 248, name: "Shimmer Pad", msb: 87, lsb: 3, pc: 106 },
    Tone { _number: 249, name: "BUBBLE 1", msb: 87, lsb: 4, pc: 64 },
    Tone { _number: 250, name: "BUBBLE 2", msb: 87, lsb: 4, pc: 65 },
    Tone { _number: 251, name: "BUBBLE 3", msb: 87, lsb: 4, pc: 66 },
    Tone { _number: 252, name: "Soft PWM Pad", msb: 87, lsb: 4, pc: 74 },
    Tone { _number: 253, name: "Org Pad", msb: 87, lsb: 4, pc: 75 },
    Tone { _number: 254, name: "Hollow Pad 2", msb: 87, lsb: 4, pc: 76 },
    Tone { _number: 255, name: "SavannaPad 1", msb: 87, lsb: 4, pc: 77 },
    Tone { _number: 256, name: "SavannaPad 2", msb: 87, lsb: 4, pc: 78 },
    Tone { _number: 257, name: "SavannaPad 3", msb: 87, lsb: 4, pc: 79 },
    Tone { _number: 258, name: "PWM Pad 1", msb: 87, lsb: 3, pc: 3 },
    Tone { _number: 259, name: "PWM Pad 2", msb: 87, lsb: 4, pc: 81 },
    Tone { _number: 260, name: "Str Machine", msb: 87, lsb: 4, pc: 82 },
    Tone { _number: 261, name: "Reso Pad", msb: 87, lsb: 4, pc: 83 },
    Tone { _number: 262, name: "BPF Pad", msb: 87, lsb: 4, pc: 84 },
    Tone { _number: 263, name: "Sweep Pad", msb: 121, lsb: 0, pc: 95 },
    Tone { _number: 264, name: "Sweep Pad 2", msb: 87, lsb: 4, pc: 86 },
    Tone { _number: 265, name: "Sweep Pad 3", msb: 87, lsb: 4, pc: 87 },
    Tone { _number: 266, name: "Sweep Pad 4", msb: 87, lsb: 4, pc: 88 },
    Tone { _number: 267, name: "Scoop Pad 1", msb: 87, lsb: 4, pc: 89 },
    Tone { _number: 268, name: "Scoop Pad 2", msb: 87, lsb: 4, pc: 90 },
    Tone { _number: 269, name: "Brite Wine", msb: 87, lsb: 4, pc: 91 },
    Tone { _number: 270, name: "Wine Pad", msb: 87, lsb: 4, pc: 92 },
    Tone { _number: 271, name: "Sine Magic", msb: 87, lsb: 4, pc: 94 },
    Tone { _number: 272, name: "Syn.Strings1", msb: 121, lsb: 0, pc: 50 },
    Tone { _number: 273, name: "Syn.Strings2", msb: 121, lsb: 0, pc: 51 },
    Tone { _number: 274, name: "Syn.Strings3", msb: 121, lsb: 1, pc: 50 },
    Tone { _number: 275, name: "Fantasia", msb: 121, lsb: 0, pc: 88 },
    Tone { _number: 276, name: "Warm Pad", msb: 121, lsb: 0, pc: 89 },
    Tone { _number: 277, name: "Sine Pad", msb: 121, lsb: 1, pc: 89 },
    Tone { _number: 278, name: "Poly Synth", msb: 121, lsb: 0, pc: 90 },
    Tone { _number: 279, name: "Bowed Glass", msb: 121, lsb: 0, pc: 92 },
    Tone { _number: 280, name: "Metal Pad", msb: 121, lsb: 0, pc: 93 },
    Tone { _number: 281, name: "Halo Pad", msb: 121, lsb: 0, pc: 94 },
    Tone { _number: 282, name: "Ice Rain", msb: 121, lsb: 0, pc: 96 },
    Tone { _number: 283, name: "Soundtrack", msb: 121, lsb: 0, pc: 97 },
    Tone { _number: 284, name: "Atmosphere", msb: 121, lsb: 0, pc: 99 },
    Tone { _number: 285, name: "Brightness", msb: 121, lsb: 0, pc: 100 },
    Tone { _number: 286, name: "Rock Organ 1", msb: 87, lsb: 0, pc: 76 },
    Tone { _number: 287, name: "Rock Organ 2", msb: 87, lsb: 0, pc: 77 },
    Tone { _number: 288, name: "Rock Organ 3", msb: 87, lsb: 0, pc: 78 },
    Tone { _number: 289, name: "Rock Organ 4", msb: 87, lsb: 0, pc: 79 },
    Tone { _number: 290, name: "Rock Organ 5", msb: 87, lsb: 0, pc: 80 },
    Tone { _number: 291, name: "RotaryOrgan1", msb: 87, lsb: 0, pc: 81 },
    Tone { _number: 292, name: "RotaryOrgan2", msb: 87, lsb: 0, pc: 82 },
    Tone { _number: 293, name: "Perc. Organ", msb: 121, lsb: 2, pc: 17 },
    Tone { _number: 294, name: "Perc.Organ 2", msb: 87, lsb: 0, pc: 83 },
    Tone { _number: 295, name: "Perc.Organ 3", msb: 87, lsb: 0, pc: 84 },
    Tone { _number: 296, name: "Perc.Organ 4", msb: 87, lsb: 0, pc: 85 },
    Tone { _number: 297, name: "E.Organ 1", msb: 87, lsb: 0, pc: 86 },
    Tone { _number: 298, name: "E.Organ 2", msb: 87, lsb: 0, pc: 87 },
    Tone { _number: 299, name: "E.Organ 3", msb: 87, lsb: 0, pc: 88 },
    Tone { _number: 300, name: "E.Organ 4", msb: 87, lsb: 0, pc: 89 },
    Tone { _number: 301, name: "E.Organ 5", msb: 87, lsb: 0, pc: 90 },
    Tone { _number: 302, name: "E.Organ 6", msb: 87, lsb: 0, pc: 91 },
    Tone { _number: 303, name: "E.Organ 7", msb: 87, lsb: 0, pc: 92 },
    Tone { _number: 304, name: "Puff Organ", msb: 121, lsb: 1, pc: 20 },
    Tone { _number: 305, name: "Nason Flute", msb: 87, lsb: 67, pc: 18 },
    Tone { _number: 306, name: "Massive Pipe", msb: 87, lsb: 67, pc: 16 },
    Tone { _number: 307, name: "Mid Pipe Org", msb: 87, lsb: 67, pc: 17 },
    Tone { _number: 308, name: "Grand Pipes", msb: 87, lsb: 0, pc: 95 },
    Tone { _number: 309, name: "Church Org 1", msb: 121, lsb: 0, pc: 19 },
    Tone { _number: 310, name: "Church Org 2", msb: 121, lsb: 1, pc: 19 },
    Tone { _number: 311, name: "Church Org 3", msb: 121, lsb: 2, pc: 19 },
    Tone { _number: 312, name: "Theater Org", msb: 87, lsb: 67, pc: 19 },
    Tone { _number: 313, name: "Accordion Fr", msb: 121, lsb: 0, pc: 21 },
    Tone { _number: 314, name: "Accordion It", msb: 121, lsb: 1, pc: 21 },
    Tone { _number: 315, name: "AccordionIt2", msb: 87, lsb: 0, pc: 96 },
    Tone { _number: 316, name: "Musette", msb: 87, lsb: 0, pc: 97 },
    Tone { _number: 317, name: "Vodkakordion", msb: 87, lsb: 0, pc: 98 },
    Tone { _number: 318, name: "Bandoneon", msb: 121, lsb: 0, pc: 23 },
    Tone { _number: 319, name: "Harmonica", msb: 121, lsb: 0, pc: 22 },
    Tone { _number: 320, name: "Harmonica 2", msb: 87, lsb: 0, pc: 99 },
    Tone { _number: 321, name: "70's E.Org 1", msb: 87, lsb: 0, pc: 93 },
    Tone { _number: 322, name: "70's E.Org 2", msb: 87, lsb: 0, pc: 94 },
    Tone { _number: 323, name: "Ana Organ 1", msb: 87, lsb: 4, pc: 6 },
    Tone { _number: 324, name: "Ana Organ 2", msb: 87, lsb: 4, pc: 7 },
    Tone { _number: 325, name: "Ana Organ 3", msb: 87, lsb: 4, pc: 8 },
    Tone { _number: 326, name: "Ana Organ 4", msb: 87, lsb: 4, pc: 9 },
    Tone { _number: 327, name: "Ana Organ 5", msb: 87, lsb: 4, pc: 10 },
    Tone { _number: 328, name: "Organ 1", msb: 121, lsb: 0, pc: 16 },
    Tone { _number: 329, name: "Trem. Organ", msb: 121, lsb: 1, pc: 16 },
    Tone { _number: 330, name: "60's Organ", msb: 121, lsb: 2, pc: 16 },
    Tone { _number: 331, name: "70's E.Organ", msb: 121, lsb: 3, pc: 16 },
    Tone { _number: 332, name: "Organ 2", msb: 121, lsb: 0, pc: 17 },
    Tone { _number: 333, name: "Chorus Organ", msb: 121, lsb: 1, pc: 17 },
    Tone { _number: 334, name: "Organ 3", msb: 121, lsb: 0, pc: 18 },
    Tone { _number: 335, name: "Reed Organ", msb: 121, lsb: 0, pc: 20 },
    Tone { _number: 336, name: "Nylon Gtr 1", msb: 121, lsb: 0, pc: 24 },
    Tone { _number: 337, name: "Nylon Gtr 2", msb: 121, lsb: 3, pc: 24 },
    Tone { _number: 338, name: "Nylon Gtr 3", msb: 87, lsb: 0, pc: 100 },
    Tone { _number: 339, name: "Nylon Gtr 4", msb: 87, lsb: 0, pc: 101 },
    Tone { _number: 340, name: "Nylon Gtr 5", msb: 87, lsb: 0, pc: 102 },
    Tone { _number: 341, name: "Nylon Gtr 6", msb: 87, lsb: 0, pc: 103 },
    Tone { _number: 342, name: "Wet Nyln Gtr", msb: 87, lsb: 0, pc: 104 },
    Tone { _number: 343, name: "Folk Gtr 1", msb: 87, lsb: 0, pc: 106 },
    Tone { _number: 344, name: "Folk Gtr 2", msb: 87, lsb: 0, pc: 107 },
    Tone { _number: 345, name: "Folk Gtr 3", msb: 87, lsb: 0, pc: 108 },
    Tone { _number: 346, name: "Latin Gtr", msb: 87, lsb: 0, pc: 109 },
    Tone { _number: 347, name: "Clean Gtr 1", msb: 87, lsb: 0, pc: 116 },
    Tone { _number: 348, name: "Clean Gtr 2", msb: 87, lsb: 0, pc: 117 },
    Tone { _number: 349, name: "Clean Gtr 3", msb: 87, lsb: 0, pc: 118 },
    Tone { _number: 350, name: "Jazz Guitar", msb: 121, lsb: 0, pc: 26 },
    Tone { _number: 351, name: "Jazz Guitar2", msb: 87, lsb: 0, pc: 119 },
    Tone { _number: 352, name: "Pick E.Gtr", msb: 87, lsb: 0, pc: 120 },
    Tone { _number: 353, name: "Funk Guitar", msb: 121, lsb: 2, pc: 28 },
    Tone { _number: 354, name: "Funk Guitar2", msb: 87, lsb: 0, pc: 121 },
    Tone { _number: 355, name: "Wet E.Gtr", msb: 87, lsb: 0, pc: 122 },
    Tone { _number: 356, name: "Overdrive Gt", msb: 121, lsb: 0, pc: 29 },
    Tone { _number: 357, name: "OverdriveGt2", msb: 87, lsb: 0, pc: 123 },
    Tone { _number: 358, name: "Guitar Pinch", msb: 121, lsb: 1, pc: 29 },
    Tone { _number: 359, name: "Dist Gtr 1", msb: 87, lsb: 0, pc: 124 },
    Tone { _number: 360, name: "Dist Gtr 2", msb: 87, lsb: 0, pc: 125 },
    Tone { _number: 361, name: "Dist Gtr 3", msb: 87, lsb: 0, pc: 126 },
    Tone { _number: 362, name: "DistortionGt", msb: 121, lsb: 0, pc: 30 },
    Tone { _number: 363, name: "Gt Feedback1", msb: 121, lsb: 1, pc: 30 },
    Tone { _number: 364, name: "Dist Rtm Gtr", msb: 121, lsb: 2, pc: 30 },
    Tone { _number: 365, name: "Ukulele", msb: 121, lsb: 1, pc: 24 },
    Tone { _number: 366, name: "Ukulele 2", msb: 87, lsb: 0, pc: 105 },
    Tone { _number: 367, name: "Nylon Gtr 1o", msb: 121, lsb: 2, pc: 24 },
    Tone { _number: 368, name: "Steel-str.Gt", msb: 121, lsb: 0, pc: 25 },
    Tone { _number: 369, name: "12-str. Gtr", msb: 121, lsb: 1, pc: 25 },
    Tone { _number: 370, name: "Mandolin", msb: 121, lsb: 2, pc: 25 },
    Tone { _number: 371, name: "Steel + Body", msb: 121, lsb: 3, pc: 25 },
    Tone { _number: 372, name: "Pedal Steel", msb: 121, lsb: 1, pc: 26 },
    Tone { _number: 373, name: "Pedal Steel2", msb: 87, lsb: 4, pc: 11 },
    Tone { _number: 374, name: "Clean Guitar", msb: 121, lsb: 0, pc: 27 },
    Tone { _number: 375, name: "Chorus Gtr", msb: 121, lsb: 1, pc: 27 },
    Tone { _number: 376, name: "Mid Tone Gtr", msb: 121, lsb: 2, pc: 27 },
    Tone { _number: 377, name: "Muted Guitar", msb: 121, lsb: 0, pc: 28 },
    Tone { _number: 378, name: "Funk Pop", msb: 121, lsb: 1, pc: 28 },
    Tone { _number: 379, name: "Jazz Man", msb: 121, lsb: 3, pc: 28 },
    Tone { _number: 380, name: "Gt Harmonics", msb: 121, lsb: 0, pc: 31 },
    Tone { _number: 381, name: "Gt Feedback2", msb: 121, lsb: 1, pc: 31 },
    Tone { _number: 382, name: "Sitar 1", msb: 121, lsb: 0, pc: 104 },
    Tone { _number: 383, name: "Sitar 2", msb: 121, lsb: 1, pc: 104 },
    Tone { _number: 384, name: "Sitar 3", msb: 87, lsb: 0, pc: 114 },
    Tone { _number: 385, name: "Banjo", msb: 121, lsb: 0, pc: 105 },
    Tone { _number: 386, name: "Shamisen", msb: 121, lsb: 0, pc: 106 },
    Tone { _number: 387, name: "Koto", msb: 121, lsb: 0, pc: 107 },
    Tone { _number: 388, name: "Taisho Koto", msb: 121, lsb: 1, pc: 107 },
    Tone { _number: 389, name: "Aerial Harp", msb: 87, lsb: 0, pc: 112 },
    Tone { _number: 390, name: "LostParadise", msb: 87, lsb: 0, pc: 113 },
    Tone { _number: 391, name: "Indian Frtls", msb: 87, lsb: 0, pc: 115 },
    Tone { _number: 392, name: "Santur", msb: 121, lsb: 0, pc: 15 },
    Tone { _number: 393, name: "Santur 2", msb: 87, lsb: 0, pc: 110 },
    Tone { _number: 394, name: "Santur 3", msb: 87, lsb: 0, pc: 111 },
    Tone { _number: 395, name: "Acoustic Bs", msb: 121, lsb: 0, pc: 32 },
    Tone { _number: 396, name: "Acoustic Bs2", msb: 87, lsb: 0, pc: 127 },
    Tone { _number: 397, name: "Acoustic Bs3", msb: 87, lsb: 1, pc: 0 },
    Tone { _number: 398, name: "Fingered Bs", msb: 121, lsb: 0, pc: 33 },
    Tone { _number: 399, name: "Fingered Bs2", msb: 87, lsb: 1, pc: 1 },
    Tone { _number: 400, name: "Fingered Bs3", msb: 87, lsb: 1, pc: 2 },
    Tone { _number: 401, name: "Fingered Bs4", msb: 87, lsb: 1, pc: 3 },
    Tone { _number: 402, name: "Pick Bass", msb: 87, lsb: 1, pc: 4 },
    Tone { _number: 403, name: "Picked Bass", msb: 121, lsb: 0, pc: 34 },
    Tone { _number: 404, name: "Fretless Bs", msb: 121, lsb: 0, pc: 35 },
    Tone { _number: 405, name: "FretlessBs 2", msb: 87, lsb: 1, pc: 5 },
    Tone { _number: 406, name: "FretlessBs 3", msb: 87, lsb: 1, pc: 6 },
    Tone { _number: 407, name: "Finger Slap", msb: 121, lsb: 1, pc: 33 },
    Tone { _number: 408, name: "Finger Slap2", msb: 87, lsb: 1, pc: 7 },
    Tone { _number: 409, name: "Slap Bass 1", msb: 121, lsb: 0, pc: 36 },
    Tone { _number: 410, name: "Slap Bass 2", msb: 121, lsb: 0, pc: 37 },
    Tone { _number: 411, name: "Return2Base!", msb: 87, lsb: 1, pc: 8 },
    Tone { _number: 412, name: "MG Bass 1", msb: 87, lsb: 1, pc: 9 },
    Tone { _number: 413, name: "MG Bass 2", msb: 87, lsb: 1, pc: 10 },
    Tone { _number: 414, name: "MG Bass 3", msb: 87, lsb: 1, pc: 11 },
    Tone { _number: 415, name: "Modular Bs 1", msb: 87, lsb: 1, pc: 12 },
    Tone { _number: 416, name: "Modular Bs 2", msb: 87, lsb: 1, pc: 13 },
    Tone { _number: 417, name: "PWM Bass 1", msb: 87, lsb: 1, pc: 14 },
    Tone { _number: 418, name: "PWM Bass 2", msb: 87, lsb: 1, pc: 15 },
    Tone { _number: 419, name: "Big Mini", msb: 87, lsb: 1, pc: 16 },
    Tone { _number: 420, name: "Fat Analog", msb: 87, lsb: 1, pc: 17 },
    Tone { _number: 421, name: "Spike Bass", msb: 87, lsb: 1, pc: 18 },
    Tone { _number: 422, name: "SH Bass", msb: 87, lsb: 1, pc: 19 },
    Tone { _number: 423, name: "Intrusive Bs", msb: 87, lsb: 1, pc: 20 },
    Tone { _number: 424, name: "Synth Bass 1", msb: 121, lsb: 0, pc: 38 },
    Tone { _number: 425, name: "Synth Bass 2", msb: 121, lsb: 0, pc: 39 },
    Tone { _number: 426, name: "Synth Bass 3", msb: 87, lsb: 1, pc: 21 },
    Tone { _number: 427, name: "Synth Bass 4", msb: 87, lsb: 1, pc: 22 },
    Tone { _number: 428, name: "Synth Bass 5", msb: 87, lsb: 1, pc: 23 },
    Tone { _number: 429, name: "Synth Bass 6", msb: 87, lsb: 1, pc: 24 },
    Tone { _number: 430, name: "Synth Bass 7", msb: 87, lsb: 1, pc: 25 },
    Tone { _number: 431, name: "Synth Bass 8", msb: 87, lsb: 1, pc: 26 },
    Tone { _number: 432, name: "Synth Bass 9", msb: 87, lsb: 1, pc: 27 },
    Tone { _number: 433, name: "Synth Bass10", msb: 87, lsb: 1, pc: 28 },
    Tone { _number: 434, name: "Synth Bass11", msb: 87, lsb: 1, pc: 29 },
    Tone { _number: 435, name: "Synth Bass12", msb: 87, lsb: 1, pc: 30 },
    Tone { _number: 436, name: "Synth Bass13", msb: 87, lsb: 1, pc: 31 },
    Tone { _number: 437, name: "Synth Bass14", msb: 87, lsb: 1, pc: 32 },
    Tone { _number: 438, name: "Reso Bass 1", msb: 87, lsb: 1, pc: 33 },
    Tone { _number: 439, name: "Reso Bass 2", msb: 87, lsb: 1, pc: 34 },
    Tone { _number: 440, name: "Reso Bass 3", msb: 87, lsb: 1, pc: 35 },
    Tone { _number: 441, name: "Reso Bass 4", msb: 87, lsb: 1, pc: 36 },
    Tone { _number: 442, name: "Reso Bass 5", msb: 87, lsb: 1, pc: 37 },
    Tone { _number: 443, name: "Reso Bass 6", msb: 87, lsb: 1, pc: 38 },
    Tone { _number: 444, name: "Reso Bass 7", msb: 87, lsb: 1, pc: 39 },
    Tone { _number: 445, name: "Reso Bass 8", msb: 87, lsb: 1, pc: 40 },
    Tone { _number: 446, name: "Reso Bass 9", msb: 87, lsb: 4, pc: 20 },
    Tone { _number: 447, name: "Reso Bass 10", msb: 87, lsb: 4, pc: 21 },
    Tone { _number: 448, name: "Acid Bass", msb: 121, lsb: 2, pc: 38 },
    Tone { _number: 449, name: "Acid Bass 2", msb: 87, lsb: 1, pc: 41 },
    Tone { _number: 450, name: "Acid Bass 3", msb: 87, lsb: 1, pc: 42 },
    Tone { _number: 451, name: "Acid Bass 4", msb: 87, lsb: 1, pc: 43 },
    Tone { _number: 452, name: "Acid Bass 5", msb: 87, lsb: 4, pc: 14 },
    Tone { _number: 453, name: "Acid Bass 6", msb: 87, lsb: 4, pc: 15 },
    Tone { _number: 454, name: "Acid Bass 7", msb: 87, lsb: 4, pc: 16 },
    Tone { _number: 455, name: "TB Bass 1", msb: 87, lsb: 1, pc: 45 },
    Tone { _number: 456, name: "TB Bass 2", msb: 87, lsb: 1, pc: 46 },
    Tone { _number: 457, name: "TB Bass 3", msb: 87, lsb: 4, pc: 12 },
    Tone { _number: 458, name: "TB Bass 4", msb: 87, lsb: 4, pc: 13 },
    Tone { _number: 459, name: "Alpha Bass 1", msb: 87, lsb: 1, pc: 44 },
    Tone { _number: 460, name: "Alpha Bass 2", msb: 87, lsb: 1, pc: 47 },
    Tone { _number: 461, name: "Alpha ResoBs", msb: 87, lsb: 1, pc: 48 },
    Tone { _number: 462, name: "Nu Saw Bass", msb: 87, lsb: 1, pc: 49 },
    Tone { _number: 463, name: "Nu RnB SawBs", msb: 87, lsb: 1, pc: 50 },
    Tone { _number: 464, name: "Storm Bass", msb: 87, lsb: 1, pc: 51 },
    Tone { _number: 465, name: "Detune Bass", msb: 87, lsb: 1, pc: 52 },
    Tone { _number: 466, name: "Gashed Bass", msb: 87, lsb: 1, pc: 53 },
    Tone { _number: 467, name: "Hi-Energy Bs", msb: 87, lsb: 1, pc: 54 },
    Tone { _number: 468, name: "Pedal Bass 1", msb: 87, lsb: 1, pc: 55 },
    Tone { _number: 469, name: "Pedal Bass 2", msb: 87, lsb: 4, pc: 18 },
    Tone { _number: 470, name: "Monster Bass", msb: 87, lsb: 1, pc: 56 },
    Tone { _number: 471, name: "JunoSqr Bs 1", msb: 87, lsb: 1, pc: 57 },
    Tone { _number: 472, name: "JunoSqr Bs 2", msb: 87, lsb: 1, pc: 58 },
    Tone { _number: 473, name: "101 Bass", msb: 87, lsb: 1, pc: 59 },
    Tone { _number: 474, name: "106 Bass 1", msb: 87, lsb: 1, pc: 60 },
    Tone { _number: 475, name: "106 Bass 2", msb: 87, lsb: 1, pc: 61 },
    Tone { _number: 476, name: "Compu Bass 1", msb: 87, lsb: 1, pc: 62 },
    Tone { _number: 477, name: "Compu Bass 2", msb: 87, lsb: 1, pc: 63 },
    Tone { _number: 478, name: "Triangle Bs", msb: 87, lsb: 1, pc: 64 },
    Tone { _number: 479, name: "Muffled Bass", msb: 87, lsb: 1, pc: 65 },
    Tone { _number: 480, name: "Garage Bass", msb: 87, lsb: 1, pc: 66 },
    Tone { _number: 481, name: "TransistorBs", msb: 87, lsb: 1, pc: 67 },
    Tone { _number: 482, name: "Fazee Bass", msb: 87, lsb: 1, pc: 68 },
    Tone { _number: 483, name: "Brite Bass", msb: 87, lsb: 4, pc: 17 },
    Tone { _number: 484, name: "Saw Bass", msb: 87, lsb: 4, pc: 19 },
    Tone { _number: 485, name: "Sub Bass", msb: 87, lsb: 4, pc: 22 },
    Tone { _number: 486, name: "Ramp Bass", msb: 87, lsb: 4, pc: 23 },
    Tone { _number: 487, name: "Fat Bass 1", msb: 87, lsb: 4, pc: 24 },
    Tone { _number: 488, name: "Fat Bass 2", msb: 87, lsb: 4, pc: 25 },
    Tone { _number: 489, name: "Fat Bass 3", msb: 87, lsb: 4, pc: 26 },
    Tone { _number: 490, name: "Flat Bass", msb: 87, lsb: 4, pc: 27 },
    Tone { _number: 491, name: "Electro Rubb", msb: 87, lsb: 4, pc: 28 },
    Tone { _number: 492, name: "80s Bass", msb: 87, lsb: 4, pc: 29 },
    Tone { _number: 493, name: "SynthBass101", msb: 121, lsb: 1, pc: 38 },
    Tone { _number: 494, name: "Clav Bass", msb: 121, lsb: 3, pc: 38 },
    Tone { _number: 495, name: "Hammer Bass", msb: 121, lsb: 4, pc: 38 },
    Tone { _number: 496, name: "SynSlap Bass", msb: 121, lsb: 1, pc: 39 },
    Tone { _number: 497, name: "Rubber Bass", msb: 121, lsb: 2, pc: 39 },
    Tone { _number: 498, name: "Attack Pulse", msb: 121, lsb: 3, pc: 39 },
    Tone { _number: 499, name: "Jazz Scat 1", msb: 87, lsb: 3, pc: 86 },
    Tone { _number: 500, name: "Jazz Scat 2", msb: 87, lsb: 3, pc: 87 },
    Tone { _number: 501, name: "GX Choir", msb: 87, lsb: 72, pc: 0 },
    Tone { _number: 502, name: "Choir Aahs", msb: 121, lsb: 0, pc: 52 },
    Tone { _number: 503, name: "Chorus Aahs", msb: 121, lsb: 1, pc: 52 },
    Tone { _number: 504, name: "Choir Pad", msb: 87, lsb: 3, pc: 88 },
    Tone { _number: 505, name: "Angels Choir", msb: 87, lsb: 3, pc: 89 },
    Tone { _number: 506, name: "Aerial Choir", msb: 87, lsb: 3, pc: 90 },
    Tone { _number: 507, name: "Voice Oohs", msb: 121, lsb: 0, pc: 53 },
    Tone { _number: 508, name: "Doo Pad", msb: 87, lsb: 3, pc: 91 },
    Tone { _number: 509, name: "Humming", msb: 121, lsb: 1, pc: 53 },
    Tone { _number: 510, name: "Humming 2", msb: 87, lsb: 3, pc: 92 },
    Tone { _number: 511, name: "Humming 3", msb: 87, lsb: 3, pc: 93 },
    Tone { _number: 512, name: "Gospel Hum", msb: 87, lsb: 3, pc: 94 },
    Tone { _number: 513, name: "Decay Choir", msb: 87, lsb: 72, pc: 26 },
    Tone { _number: 514, name: "Dcy ChoirPad", msb: 87, lsb: 69, pc: 24 },
    Tone { _number: 515, name: "Vox Pad 1", msb: 87, lsb: 3, pc: 95 },
    Tone { _number: 516, name: "Vox Pad 2", msb: 87, lsb: 3, pc: 96 },
    Tone { _number: 517, name: "Dreamvox 1", msb: 87, lsb: 3, pc: 13 },
    Tone { _number: 518, name: "Dreamvox 2", msb: 87, lsb: 3, pc: 14 },
    Tone { _number: 519, name: "80s Vox", msb: 87, lsb: 3, pc: 97 },
    Tone { _number: 520, name: "SynVox", msb: 121, lsb: 0, pc: 54 },
    Tone { _number: 521, name: "SynVox 2", msb: 87, lsb: 3, pc: 98 },
    Tone { _number: 522, name: "SynVox 3", msb: 87, lsb: 3, pc: 99 },
    Tone { _number: 523, name: "Mini Vox", msb: 87, lsb: 3, pc: 100 },
    Tone { _number: 524, name: "Chipmunk", msb: 87, lsb: 3, pc: 101 },
    Tone { _number: 525, name: "Sample Opera", msb: 87, lsb: 3, pc: 102 },
    Tone { _number: 526, name: "Sad Ceremony", msb: 87, lsb: 3, pc: 103 },
    Tone { _number: 527, name: "5th Voice", msb: 87, lsb: 4, pc: 51 },
    Tone { _number: 528, name: "Sop Vox", msb: 87, lsb: 4, pc: 93 },
    Tone { _number: 529, name: "Analog Voice", msb: 121, lsb: 1, pc: 54 },
    Tone { _number: 530, name: "Space Voice", msb: 121, lsb: 0, pc: 91 },
    Tone { _number: 531, name: "Itopia", msb: 121, lsb: 1, pc: 91 },
    Tone { _number: 532, name: "Dreaming Box", msb: 87, lsb: 0, pc: 66 },
    Tone { _number: 533, name: "Brass 1", msb: 121, lsb: 0, pc: 61 },
    Tone { _number: 534, name: "Brass 2", msb: 121, lsb: 1, pc: 61 },
    Tone { _number: 535, name: "Brass 3", msb: 87, lsb: 1, pc: 95 },
    Tone { _number: 536, name: "Brass 4", msb: 87, lsb: 1, pc: 96 },
    Tone { _number: 537, name: "Brass 5", msb: 87, lsb: 1, pc: 97 },
    Tone { _number: 538, name: "Brass 6", msb: 87, lsb: 1, pc: 98 },
    Tone { _number: 539, name: "80s Brass 1", msb: 87, lsb: 1, pc: 99 },
    Tone { _number: 540, name: "80s Brass 2", msb: 87, lsb: 1, pc: 100 },
    Tone { _number: 541, name: "80s Brass 3", msb: 87, lsb: 1, pc: 101 },
    Tone { _number: 542, name: "80s Brass 4", msb: 87, lsb: 1, pc: 102 },
    Tone { _number: 543, name: "80s Brass 5", msb: 87, lsb: 1, pc: 103 },
    Tone { _number: 544, name: "80s Brass 6", msb: 87, lsb: 1, pc: 104 },
    Tone { _number: 545, name: "80s Brass 7", msb: 87, lsb: 1, pc: 105 },
    Tone { _number: 546, name: "80s Brass 8", msb: 87, lsb: 1, pc: 106 },
    Tone { _number: 547, name: "Soft SynBrs1", msb: 87, lsb: 1, pc: 107 },
    Tone { _number: 548, name: "Soft SynBrs2", msb: 87, lsb: 1, pc: 117 },
    Tone { _number: 549, name: "Warm SynBrs", msb: 87, lsb: 1, pc: 108 },
    Tone { _number: 550, name: "Brite SynBrs", msb: 87, lsb: 1, pc: 109 },
    Tone { _number: 551, name: "Express Brs", msb: 87, lsb: 1, pc: 110 },
    Tone { _number: 552, name: "EuroExpress1", msb: 87, lsb: 1, pc: 111 },
    Tone { _number: 553, name: "JP Brass 1", msb: 87, lsb: 1, pc: 112 },
    Tone { _number: 554, name: "JP Brass 2", msb: 87, lsb: 1, pc: 118 },
    Tone { _number: 555, name: "Juno Brass", msb: 87, lsb: 1, pc: 113 },
    Tone { _number: 556, name: "Ox Brass", msb: 87, lsb: 1, pc: 114 },
    Tone { _number: 557, name: "Reso Brass", msb: 87, lsb: 1, pc: 115 },
    Tone { _number: 558, name: "Wide SynBrs", msb: 87, lsb: 1, pc: 116 },
    Tone { _number: 559, name: "106 Brass", msb: 87, lsb: 1, pc: 119 },
    Tone { _number: 560, name: "Octa Brass", msb: 87, lsb: 1, pc: 120 },
    Tone { _number: 561, name: "Poly Brass 1", msb: 87, lsb: 1, pc: 121 },
    Tone { _number: 562, name: "Poly Brass 2", msb: 87, lsb: 4, pc: 40 },
    Tone { _number: 563, name: "Dual Saw Brs", msb: 87, lsb: 1, pc: 122 },
    Tone { _number: 564, name: "Jump Poly", msb: 87, lsb: 2, pc: 108 },
    Tone { _number: 565, name: "Reso Key 1", msb: 87, lsb: 2, pc: 123 },
    Tone { _number: 566, name: "EuroExpress2", msb: 87, lsb: 2, pc: 126 },
    Tone { _number: 567, name: "Ox Synth", msb: 87, lsb: 3, pc: 4 },
    Tone { _number: 568, name: "VintageBrs 1", msb: 87, lsb: 4, pc: 36 },
    Tone { _number: 569, name: "VintageBrs 2", msb: 87, lsb: 4, pc: 37 },
    Tone { _number: 570, name: "VintageBrs 3", msb: 87, lsb: 4, pc: 38 },
    Tone { _number: 571, name: "VintageBrs 4", msb: 87, lsb: 4, pc: 39 },
    Tone { _number: 572, name: "JP Brass", msb: 121, lsb: 1, pc: 62 },
    Tone { _number: 573, name: "Oct SynBrass", msb: 121, lsb: 2, pc: 62 },
    Tone { _number: 574, name: "Jump Brass", msb: 121, lsb: 3, pc: 62 },
    Tone { _number: 575, name: "Synth Brass1", msb: 121, lsb: 0, pc: 62 },
    Tone { _number: 576, name: "Synth Brass2", msb: 121, lsb: 0, pc: 63 },
    Tone { _number: 577, name: "SynBrass sfz", msb: 121, lsb: 1, pc: 63 },
    Tone { _number: 578, name: "Velo Brass", msb: 121, lsb: 2, pc: 63 },
    Tone { _number: 579, name: "Trumpet", msb: 121, lsb: 0, pc: 56 },
    Tone { _number: 580, name: "Trumpet 2", msb: 87, lsb: 1, pc: 93 },
    Tone { _number: 581, name: "Dark Trumpet", msb: 121, lsb: 1, pc: 56 },
    Tone { _number: 582, name: "MuteTrumpet1", msb: 121, lsb: 0, pc: 59 },
    Tone { _number: 583, name: "MuteTrumpet2", msb: 121, lsb: 1, pc: 59 },
    Tone { _number: 584, name: "Trombone 1", msb: 121, lsb: 0, pc: 57 },
    Tone { _number: 585, name: "Trombone 2", msb: 121, lsb: 1, pc: 57 },
    Tone { _number: 586, name: "Bright Tb", msb: 121, lsb: 2, pc: 57 },
    Tone { _number: 587, name: "Tuba", msb: 121, lsb: 0, pc: 58 },
    Tone { _number: 588, name: "Fr.Horn", msb: 87, lsb: 1, pc: 94 },
    Tone { _number: 589, name: "French Horn", msb: 121, lsb: 1, pc: 60 },
    Tone { _number: 590, name: "F.Horn Sect", msb: 121, lsb: 0, pc: 60 },
    Tone { _number: 591, name: "Soprano Sax", msb: 121, lsb: 0, pc: 64 },
    Tone { _number: 592, name: "Soprano Sax2", msb: 87, lsb: 1, pc: 123 },
    Tone { _number: 593, name: "Alto Sax", msb: 121, lsb: 0, pc: 65 },
    Tone { _number: 594, name: "Tenor Sax", msb: 121, lsb: 0, pc: 66 },
    Tone { _number: 595, name: "Tenor Sax 2", msb: 87, lsb: 1, pc: 125 },
    Tone { _number: 596, name: "BreathyTenor", msb: 87, lsb: 1, pc: 124 },
    Tone { _number: 597, name: "Baritone Sax", msb: 121, lsb: 0, pc: 67 },
    Tone { _number: 598, name: "Oboe", msb: 121, lsb: 0, pc: 68 },
    Tone { _number: 599, name: "English Horn", msb: 121, lsb: 0, pc: 69 },
    Tone { _number: 600, name: "Bassoon", msb: 121, lsb: 0, pc: 70 },
    Tone { _number: 601, name: "Bassoon 2", msb: 87, lsb: 1, pc: 87 },
    Tone { _number: 602, name: "Clarinet", msb: 121, lsb: 0, pc: 71 },
    Tone { _number: 603, name: "Piccolo", msb: 121, lsb: 0, pc: 72 },
    Tone { _number: 604, name: "Flute", msb: 121, lsb: 0, pc: 73 },
    Tone { _number: 605, name: "Flute 2", msb: 87, lsb: 1, pc: 88 },
    Tone { _number: 606, name: "Recorder", msb: 121, lsb: 0, pc: 74 },
    Tone { _number: 607, name: "Pan Flute", msb: 121, lsb: 0, pc: 75 },
    Tone { _number: 608, name: "Pan Flute 2", msb: 87, lsb: 1, pc: 89 },
    Tone { _number: 609, name: "Pan Pipes 1", msb: 87, lsb: 1, pc: 90 },
    Tone { _number: 610, name: "Pan Pipes 2", msb: 87, lsb: 4, pc: 35 },
    Tone { _number: 611, name: "Bottle Blow", msb: 121, lsb: 0, pc: 76 },
    Tone { _number: 612, name: "Shakuhachi", msb: 121, lsb: 0, pc: 77 },
    Tone { _number: 613, name: "Shakuhachi 2", msb: 87, lsb: 1, pc: 91 },
    Tone { _number: 614, name: "Whistle", msb: 121, lsb: 0, pc: 78 },
    Tone { _number: 615, name: "Ocarina", msb: 121, lsb: 0, pc: 79 },
    Tone { _number: 616, name: "Ocarina 2", msb: 87, lsb: 1, pc: 92 },
    Tone { _number: 617, name: "Bagpipe", msb: 121, lsb: 0, pc: 109 },
    Tone { _number: 618, name: "Shanai", msb: 121, lsb: 0, pc: 111 },
    Tone { _number: 619, name: "Dream Trance", msb: 87, lsb: 2, pc: 95 },
    Tone { _number: 620, name: "Dream Saws", msb: 87, lsb: 2, pc: 96 },
    Tone { _number: 621, name: "Dream Pulse", msb: 87, lsb: 2, pc: 97 },
    Tone { _number: 622, name: "Trance Synth", msb: 87, lsb: 2, pc: 98 },
    Tone { _number: 623, name: "Trancy", msb: 87, lsb: 2, pc: 99 },
    Tone { _number: 624, name: "Trance Keys", msb: 87, lsb: 2, pc: 100 },
    Tone { _number: 625, name: "Trance Saws", msb: 87, lsb: 2, pc: 101 },
    Tone { _number: 626, name: "Auto Trance1", msb: 87, lsb: 2, pc: 102 },
    Tone { _number: 627, name: "Super Saws 1", msb: 87, lsb: 2, pc: 103 },
    Tone { _number: 628, name: "Analog Saws", msb: 87, lsb: 2, pc: 104 },
    Tone { _number: 629, name: "Uni-G", msb: 87, lsb: 2, pc: 105 },
    Tone { _number: 630, name: "Digitaless", msb: 87, lsb: 2, pc: 106 },
    Tone { _number: 631, name: "Bustranza", msb: 87, lsb: 2, pc: 107 },
    Tone { _number: 632, name: "Super Saws 2", msb: 87, lsb: 2, pc: 109 },
    Tone { _number: 633, name: "Poly Synth 2", msb: 87, lsb: 2, pc: 110 },
    Tone { _number: 634, name: "Poly Synth 3", msb: 87, lsb: 2, pc: 111 },
    Tone { _number: 635, name: "Poly Synth 4", msb: 87, lsb: 2, pc: 112 },
    Tone { _number: 636, name: "Poly Synth 5", msb: 87, lsb: 2, pc: 113 },
    Tone { _number: 637, name: "Poly Synth 6", msb: 87, lsb: 2, pc: 114 },
    Tone { _number: 638, name: "Poly Synth 7", msb: 87, lsb: 2, pc: 115 },
    Tone { _number: 639, name: "Juno Saw Key", msb: 87, lsb: 2, pc: 116 },
    Tone { _number: 640, name: "Saw Key 1", msb: 87, lsb: 2, pc: 117 },
    Tone { _number: 641, name: "Saw Key 2", msb: 87, lsb: 2, pc: 118 },
    Tone { _number: 642, name: "Waspy Synth", msb: 87, lsb: 2, pc: 119 },
    Tone { _number: 643, name: "Juno SQR", msb: 87, lsb: 2, pc: 120 },
    Tone { _number: 644, name: "Vintage Key", msb: 87, lsb: 2, pc: 121 },
    Tone { _number: 645, name: "Ju-D Fifths", msb: 87, lsb: 2, pc: 122 },
    Tone { _number: 646, name: "Reso Key 2", msb: 87, lsb: 2, pc: 124 },
    Tone { _number: 647, name: "Fat Synth", msb: 87, lsb: 2, pc: 125 },
    Tone { _number: 648, name: "DOC Stack", msb: 87, lsb: 2, pc: 127 },
    Tone { _number: 649, name: "2 Saws", msb: 87, lsb: 3, pc: 0 },
    Tone { _number: 650, name: "Hi Saw Band", msb: 87, lsb: 3, pc: 1 },
    Tone { _number: 651, name: "Brite Synth", msb: 87, lsb: 3, pc: 2 },
    Tone { _number: 652, name: "RAVtune", msb: 87, lsb: 3, pc: 5 },
    Tone { _number: 653, name: "Saw Lead 1", msb: 87, lsb: 1, pc: 126 },
    Tone { _number: 654, name: "Saw Lead 2", msb: 87, lsb: 1, pc: 127 },
    Tone { _number: 655, name: "Saw Lead 3", msb: 87, lsb: 2, pc: 0 },
    Tone { _number: 656, name: "Saw Lead 4", msb: 87, lsb: 2, pc: 1 },
    Tone { _number: 657, name: "Saw Lead 5", msb: 87, lsb: 2, pc: 2 },
    Tone { _number: 658, name: "Saw Lead 6", msb: 87, lsb: 2, pc: 3 },
    Tone { _number: 659, name: "Saw Lead 7", msb: 87, lsb: 2, pc: 4 },
    Tone { _number: 660, name: "Saw Lead 8", msb: 87, lsb: 2, pc: 5 },
    Tone { _number: 661, name: "Saw Lead 9", msb: 87, lsb: 2, pc: 6 },
    Tone { _number: 662, name: "Saw Lead 10", msb: 87, lsb: 4, pc: 45 },
    Tone { _number: 663, name: "GR300 Lead 1", msb: 87, lsb: 2, pc: 7 },
    Tone { _number: 664, name: "GR300 Lead 2", msb: 87, lsb: 2, pc: 8 },
    Tone { _number: 665, name: "Classic GR", msb: 87, lsb: 2, pc: 9 },
    Tone { _number: 666, name: "Bright GR", msb: 87, lsb: 2, pc: 10 },
    Tone { _number: 667, name: "Fat GR Lead", msb: 87, lsb: 2, pc: 11 },
    Tone { _number: 668, name: "MODified Ld", msb: 87, lsb: 2, pc: 12 },
    Tone { _number: 669, name: "Syn Lead 1", msb: 87, lsb: 2, pc: 13 },
    Tone { _number: 670, name: "Syn Lead 2", msb: 87, lsb: 2, pc: 14 },
    Tone { _number: 671, name: "Syn Lead 3", msb: 87, lsb: 2, pc: 15 },
    Tone { _number: 672, name: "Syn Lead 4", msb: 87, lsb: 2, pc: 16 },
    Tone { _number: 673, name: "Syn Lead 5", msb: 87, lsb: 2, pc: 17 },
    Tone { _number: 674, name: "Syn Lead 6", msb: 87, lsb: 2, pc: 18 },
    Tone { _number: 675, name: "Syn Lead 7", msb: 87, lsb: 2, pc: 19 },
    Tone { _number: 676, name: "Pro Fat Ld 1", msb: 87, lsb: 2, pc: 20 },
    Tone { _number: 677, name: "Pro Fat Ld 2", msb: 87, lsb: 2, pc: 26 },
    Tone { _number: 678, name: "JupiterLead1", msb: 87, lsb: 2, pc: 21 },
    Tone { _number: 679, name: "JupiterLead2", msb: 87, lsb: 2, pc: 22 },
    Tone { _number: 680, name: "Porta Lead", msb: 87, lsb: 2, pc: 23 },
    Tone { _number: 681, name: "Classic Lead", msb: 87, lsb: 2, pc: 24 },
    Tone { _number: 682, name: "On Air", msb: 87, lsb: 2, pc: 25 },
    Tone { _number: 683, name: "Wormy Lead", msb: 87, lsb: 2, pc: 27 },
    Tone { _number: 684, name: "Waspy Lead", msb: 87, lsb: 2, pc: 28 },
    Tone { _number: 685, name: "Brite ResoLd", msb: 87, lsb: 2, pc: 29 },
    Tone { _number: 686, name: "Brass Lead", msb: 87, lsb: 2, pc: 30 },
    Tone { _number: 687, name: "Legato Tkno", msb: 87, lsb: 2, pc: 31 },
    Tone { _number: 688, name: "Follow Me", msb: 87, lsb: 2, pc: 32 },
    Tone { _number: 689, name: "Octa Juice", msb: 87, lsb: 2, pc: 33 },
    Tone { _number: 690, name: "Juicy Jupe", msb: 87, lsb: 2, pc: 34 },
    Tone { _number: 691, name: "Octa Saw", msb: 87, lsb: 2, pc: 35 },
    Tone { _number: 692, name: "Vintager 1", msb: 87, lsb: 2, pc: 36 },
    Tone { _number: 693, name: "Vintager 2", msb: 87, lsb: 2, pc: 37 },
    Tone { _number: 694, name: "Sync Lead", msb: 87, lsb: 2, pc: 38 },
    Tone { _number: 695, name: "Octa Sync", msb: 87, lsb: 2, pc: 39 },
    Tone { _number: 696, name: "Leading Sync", msb: 87, lsb: 2, pc: 40 },
    Tone { _number: 697, name: "A Leader", msb: 87, lsb: 2, pc: 41 },
    Tone { _number: 698, name: "Hot Coffee", msb: 87, lsb: 2, pc: 42 },
    Tone { _number: 699, name: "Hot Sync", msb: 87, lsb: 2, pc: 43 },
    Tone { _number: 700, name: "Synchro Lead", msb: 87, lsb: 2, pc: 44 },
    Tone { _number: 701, name: "Space Solo", msb: 87, lsb: 2, pc: 45 },
    Tone { _number: 702, name: "Squareheads", msb: 87, lsb: 2, pc: 46 },
    Tone { _number: 703, name: "Mod Lead", msb: 87, lsb: 2, pc: 47 },
    Tone { _number: 704, name: "Alpha Spit", msb: 87, lsb: 2, pc: 48 },
    Tone { _number: 705, name: "Air Lead", msb: 87, lsb: 2, pc: 49 },
    Tone { _number: 706, name: "Pulstar Lead", msb: 87, lsb: 2, pc: 50 },
    Tone { _number: 707, name: "Therasaw", msb: 87, lsb: 2, pc: 51 },
    Tone { _number: 708, name: "Warmy Lead", msb: 87, lsb: 2, pc: 52 },
    Tone { _number: 709, name: "ResoSawLead", msb: 87, lsb: 2, pc: 53 },
    Tone { _number: 710, name: "Soft Reso Ld", msb: 87, lsb: 2, pc: 54 },
    Tone { _number: 711, name: "Reso Lead 1", msb: 87, lsb: 2, pc: 55 },
    Tone { _number: 712, name: "Reso Lead 2", msb: 87, lsb: 2, pc: 56 },
    Tone { _number: 713, name: "Reso Lead 3", msb: 87, lsb: 2, pc: 57 },
    Tone { _number: 714, name: "Reso Lead 4", msb: 87, lsb: 2, pc: 58 },
    Tone { _number: 715, name: "Reso Lead 5", msb: 87, lsb: 2, pc: 59 },
    Tone { _number: 716, name: "Juicy Lead", msb: 87, lsb: 2, pc: 60 },
    Tone { _number: 717, name: "DC Triangle", msb: 87, lsb: 2, pc: 61 },
    Tone { _number: 718, name: "Soft Lead 1", msb: 87, lsb: 2, pc: 62 },
    Tone { _number: 719, name: "Soft Lead 2", msb: 87, lsb: 2, pc: 63 },
    Tone { _number: 720, name: "Soft Lead 3", msb: 87, lsb: 2, pc: 64 },
    Tone { _number: 721, name: "Soft Lead 4", msb: 87, lsb: 2, pc: 65 },
    Tone { _number: 722, name: "Soft Lead 5", msb: 87, lsb: 2, pc: 66 },
    Tone { _number: 723, name: "Soft Lead 6", msb: 87, lsb: 2, pc: 67 },
    Tone { _number: 724, name: "Soft Lead 7", msb: 87, lsb: 2, pc: 68 },
    Tone { _number: 725, name: "Soft Lead 8", msb: 87, lsb: 2, pc: 69 },
    Tone { _number: 726, name: "Soft Lead 9", msb: 87, lsb: 2, pc: 70 },
    Tone { _number: 727, name: "Soft Lead 10", msb: 87, lsb: 2, pc: 71 },
    Tone { _number: 728, name: "Tri Lead", msb: 87, lsb: 2, pc: 72 },
    Tone { _number: 729, name: "Pulse Lead 1", msb: 87, lsb: 2, pc: 73 },
    Tone { _number: 730, name: "Pulse Lead 2", msb: 87, lsb: 2, pc: 74 },
    Tone { _number: 731, name: "Pulse Lead 3", msb: 87, lsb: 4, pc: 41 },
    Tone { _number: 732, name: "Pulse Lead 4", msb: 87, lsb: 4, pc: 42 },
    Tone { _number: 733, name: "Simple Tri", msb: 87, lsb: 2, pc: 75 },
    Tone { _number: 734, name: "Simple Sine", msb: 87, lsb: 2, pc: 76 },
    Tone { _number: 735, name: "Whistle Ld 1", msb: 87, lsb: 2, pc: 77 },
    Tone { _number: 736, name: "Whistle Ld 2", msb: 87, lsb: 2, pc: 78 },
    Tone { _number: 737, name: "Square Pipe", msb: 87, lsb: 2, pc: 79 },
    Tone { _number: 738, name: "CosmicDrops1", msb: 87, lsb: 2, pc: 80 },
    Tone { _number: 739, name: "CosmicDrops2", msb: 87, lsb: 3, pc: 116 },
    Tone { _number: 740, name: "Spooky Lead", msb: 87, lsb: 2, pc: 81 },
    Tone { _number: 741, name: "Pure Lead", msb: 87, lsb: 2, pc: 82 },
    Tone { _number: 742, name: "303 NRG", msb: 87, lsb: 2, pc: 83 },
    Tone { _number: 743, name: "Round SQR", msb: 87, lsb: 2, pc: 84 },
    Tone { _number: 744, name: "Brite SQR", msb: 87, lsb: 2, pc: 85 },
    Tone { _number: 745, name: "Square SAW", msb: 87, lsb: 2, pc: 86 },
    Tone { _number: 746, name: "Simple SQR", msb: 87, lsb: 2, pc: 87 },
    Tone { _number: 747, name: "Sqr Lead", msb: 87, lsb: 2, pc: 88 },
    Tone { _number: 748, name: "Atk Lead", msb: 87, lsb: 2, pc: 89 },
    Tone { _number: 749, name: "Octa Square", msb: 87, lsb: 2, pc: 90 },
    Tone { _number: 750, name: "CS Lead", msb: 87, lsb: 2, pc: 91 },
    Tone { _number: 751, name: "Mini Growl", msb: 87, lsb: 2, pc: 92 },
    Tone { _number: 752, name: "Hoover Again", msb: 87, lsb: 2, pc: 93 },
    Tone { _number: 753, name: "Tranceformer", msb: 87, lsb: 2, pc: 94 },
    Tone { _number: 754, name: "Analog Seq", msb: 87, lsb: 3, pc: 6 },
    Tone { _number: 755, name: "Seq Pop", msb: 87, lsb: 3, pc: 7 },
    Tone { _number: 756, name: "Periscope", msb: 87, lsb: 3, pc: 8 },
    Tone { _number: 757, name: "Major 7", msb: 87, lsb: 3, pc: 9 },
    Tone { _number: 758, name: "Pipe Key", msb: 87, lsb: 3, pc: 18 },
    Tone { _number: 759, name: "Enigmatic", msb: 87, lsb: 3, pc: 104 },
    Tone { _number: 760, name: "Planetz", msb: 87, lsb: 3, pc: 105 },
    Tone { _number: 761, name: "Sci-Fi", msb: 87, lsb: 3, pc: 107 },
    Tone { _number: 762, name: "ResoSweep Dn", msb: 87, lsb: 3, pc: 108 },
    Tone { _number: 763, name: "Jet Noise", msb: 87, lsb: 3, pc: 109 },
    Tone { _number: 764, name: "Brandish", msb: 87, lsb: 3, pc: 110 },
    Tone { _number: 765, name: "909 Fx", msb: 87, lsb: 3, pc: 111 },
    Tone { _number: 766, name: "Zap", msb: 87, lsb: 3, pc: 112 },
    Tone { _number: 767, name: "PolySweep Nz", msb: 87, lsb: 3, pc: 113 },
    Tone { _number: 768, name: "Passing By", msb: 87, lsb: 3, pc: 114 },
    Tone { _number: 769, name: "Lazer Points", msb: 87, lsb: 3, pc: 115 },
    Tone { _number: 770, name: "Crystal Fx", msb: 87, lsb: 3, pc: 117 },
    Tone { _number: 771, name: "Crystal Ice", msb: 87, lsb: 3, pc: 118 },
    Tone { _number: 772, name: "Mad Noise", msb: 87, lsb: 3, pc: 119 },
    Tone { _number: 773, name: "Robot Sci-Fi", msb: 87, lsb: 3, pc: 120 },
    Tone { _number: 774, name: "Computer 1", msb: 87, lsb: 3, pc: 121 },
    Tone { _number: 775, name: "Computer 2", msb: 87, lsb: 3, pc: 122 },
    Tone { _number: 776, name: "S&H Noise", msb: 87, lsb: 3, pc: 123 },
    Tone { _number: 777, name: "S&H Ramp", msb: 87, lsb: 3, pc: 124 },
    Tone { _number: 778, name: "S&H PWM", msb: 87, lsb: 3, pc: 125 },
    Tone { _number: 779, name: "S&H Saw 1", msb: 87, lsb: 3, pc: 126 },
    Tone { _number: 780, name: "S&H Saw 2", msb: 87, lsb: 3, pc: 127 },
    Tone { _number: 781, name: "Ramp Lead 1", msb: 87, lsb: 4, pc: 43 },
    Tone { _number: 782, name: "Ramp Lead 2", msb: 87, lsb: 4, pc: 44 },
    Tone { _number: 783, name: "Sine Lead 1", msb: 87, lsb: 4, pc: 46 },
    Tone { _number: 784, name: "Sine Lead 2", msb: 87, lsb: 4, pc: 47 },
    Tone { _number: 785, name: "Mod Chord", msb: 87, lsb: 4, pc: 49 },
    Tone { _number: 786, name: "Housechord", msb: 87, lsb: 4, pc: 50 },
    Tone { _number: 787, name: "Juno-D Maj7", msb: 87, lsb: 4, pc: 52 },
    Tone { _number: 788, name: "Sweet House", msb: 87, lsb: 4, pc: 53 },
    Tone { _number: 789, name: "Detune Saws", msb: 87, lsb: 4, pc: 54 },
    Tone { _number: 790, name: "Electrostar", msb: 87, lsb: 4, pc: 55 },
    Tone { _number: 791, name: "Dance Saws1", msb: 87, lsb: 4, pc: 56 },
    Tone { _number: 792, name: "Resoform", msb: 87, lsb: 4, pc: 57 },
    Tone { _number: 793, name: "Melodic Drum", msb: 87, lsb: 4, pc: 58 },
    Tone { _number: 794, name: "Alpha Said", msb: 87, lsb: 4, pc: 59 },
    Tone { _number: 795, name: "Shroomy", msb: 87, lsb: 4, pc: 60 },
    Tone { _number: 796, name: "Detune Seq", msb: 87, lsb: 4, pc: 61 },
    Tone { _number: 797, name: "LoFi Piano", msb: 87, lsb: 4, pc: 62 },
    Tone { _number: 798, name: "FX Ramp", msb: 87, lsb: 4, pc: 63 },
    Tone { _number: 799, name: "Scratch 2", msb: 87, lsb: 4, pc: 67 },
    Tone { _number: 800, name: "AnalogDays 1", msb: 87, lsb: 4, pc: 68 },
    Tone { _number: 801, name: "Dance Saws 2", msb: 87, lsb: 4, pc: 69 },
    Tone { _number: 802, name: "Sync Key", msb: 87, lsb: 4, pc: 70 },
    Tone { _number: 803, name: "Detune Ramp", msb: 87, lsb: 4, pc: 71 },
    Tone { _number: 804, name: "Reso Saw", msb: 87, lsb: 4, pc: 72 },
    Tone { _number: 805, name: "EuroExpress3", msb: 87, lsb: 4, pc: 73 },
    Tone { _number: 806, name: "Sweep Saw", msb: 87, lsb: 4, pc: 85 },
    Tone { _number: 807, name: "Pulsatron", msb: 87, lsb: 4, pc: 95 },
    Tone { _number: 808, name: "Motion Bass", msb: 87, lsb: 4, pc: 96 },
    Tone { _number: 809, name: "Trance Splt", msb: 87, lsb: 4, pc: 97 },
    Tone { _number: 810, name: "Rhythmic 5th", msb: 87, lsb: 4, pc: 98 },
    Tone { _number: 811, name: "Rhythmic 1", msb: 87, lsb: 4, pc: 99 },
    Tone { _number: 812, name: "Rhythmic 2", msb: 87, lsb: 4, pc: 100 },
    Tone { _number: 813, name: "Mega Sync 1", msb: 87, lsb: 4, pc: 101 },
    Tone { _number: 814, name: "StrobeBell 3", msb: 87, lsb: 4, pc: 102 },
    Tone { _number: 815, name: "Strobe 1", msb: 87, lsb: 4, pc: 103 },
    Tone { _number: 816, name: "Strobe 2", msb: 87, lsb: 4, pc: 104 },
    Tone { _number: 817, name: "Strobe 3", msb: 87, lsb: 4, pc: 105 },
    Tone { _number: 818, name: "Strobe 4", msb: 87, lsb: 4, pc: 106 },
    Tone { _number: 819, name: "LFO Saw", msb: 87, lsb: 4, pc: 107 },
    Tone { _number: 820, name: "Keep Going", msb: 87, lsb: 4, pc: 108 },
    Tone { _number: 821, name: "Keep Running", msb: 87, lsb: 4, pc: 109 },
    Tone { _number: 822, name: "Electrons", msb: 87, lsb: 4, pc: 110 },
    Tone { _number: 823, name: "BriskVortex", msb: 87, lsb: 4, pc: 111 },
    Tone { _number: 824, name: "LFO Vox", msb: 87, lsb: 4, pc: 112 },
    Tone { _number: 825, name: "Pulsasaw", msb: 87, lsb: 4, pc: 113 },
    Tone { _number: 826, name: "Arposphere", msb: 87, lsb: 4, pc: 114 },
    Tone { _number: 827, name: "Mega Sync 2", msb: 87, lsb: 4, pc: 115 },
    Tone { _number: 828, name: "Compusonic 1", msb: 87, lsb: 4, pc: 116 },
    Tone { _number: 829, name: "Compusonic 2", msb: 87, lsb: 4, pc: 117 },
    Tone { _number: 830, name: "Compusonic 3", msb: 87, lsb: 4, pc: 118 },
    Tone { _number: 831, name: "Compusonic 4", msb: 87, lsb: 4, pc: 119 },
    Tone { _number: 832, name: "Compusonic 5", msb: 87, lsb: 4, pc: 120 },
    Tone { _number: 833, name: "AnalogDays 2", msb: 87, lsb: 4, pc: 121 },
    Tone { _number: 834, name: "Groove 7", msb: 87, lsb: 4, pc: 122 },
    Tone { _number: 835, name: "Juno Pop", msb: 87, lsb: 4, pc: 123 },
    Tone { _number: 836, name: "Auto Trance2", msb: 87, lsb: 4, pc: 124 },
    Tone { _number: 837, name: "In Da Groove", msb: 87, lsb: 4, pc: 125 },
    Tone { _number: 838, name: "80s Beat", msb: 87, lsb: 4, pc: 126 },
    Tone { _number: 839, name: "Ride Cymbal", msb: 87, lsb: 4, pc: 127 },
    Tone { _number: 840, name: "OrchestraHit", msb: 121, lsb: 0, pc: 55 },
    Tone { _number: 841, name: "Bass Hit", msb: 121, lsb: 1, pc: 55 },
    Tone { _number: 842, name: "6th Hit", msb: 121, lsb: 2, pc: 55 },
    Tone { _number: 843, name: "Euro Hit", msb: 121, lsb: 3, pc: 55 },
    Tone { _number: 844, name: "Square Wave", msb: 121, lsb: 0, pc: 80 },
    Tone { _number: 845, name: "MG Square", msb: 121, lsb: 1, pc: 80 },
    Tone { _number: 846, name: "2600 Sine", msb: 121, lsb: 2, pc: 80 },
    Tone { _number: 847, name: "Saw Wave", msb: 121, lsb: 0, pc: 81 },
    Tone { _number: 848, name: "OB2 Saw", msb: 121, lsb: 1, pc: 81 },
    Tone { _number: 849, name: "Doctor Solo", msb: 121, lsb: 2, pc: 81 },
    Tone { _number: 850, name: "Natural Lead", msb: 121, lsb: 3, pc: 81 },
    Tone { _number: 851, name: "SequencedSaw", msb: 121, lsb: 4, pc: 81 },
    Tone { _number: 852, name: "Syn.Calliope", msb: 121, lsb: 0, pc: 82 },
    Tone { _number: 853, name: "Chiffer Lead", msb: 121, lsb: 0, pc: 83 },
    Tone { _number: 854, name: "Charang", msb: 121, lsb: 0, pc: 84 },
    Tone { _number: 855, name: "Wire Lead", msb: 121, lsb: 1, pc: 84 },
    Tone { _number: 856, name: "Solo Vox", msb: 121, lsb: 0, pc: 85 },
    Tone { _number: 857, name: "5th Saw Wave", msb: 121, lsb: 0, pc: 86 },
    Tone { _number: 858, name: "Bass & Lead", msb: 121, lsb: 0, pc: 87 },
    Tone { _number: 859, name: "Delayed Lead", msb: 121, lsb: 1, pc: 87 },
    Tone { _number: 860, name: "Goblin", msb: 121, lsb: 0, pc: 101 },
    Tone { _number: 861, name: "Echo Drops", msb: 121, lsb: 0, pc: 102 },
    Tone { _number: 862, name: "Echo Bell", msb: 121, lsb: 1, pc: 102 },
    Tone { _number: 863, name: "Echo Pan", msb: 121, lsb: 2, pc: 102 },
    Tone { _number: 864, name: "Star Theme", msb: 121, lsb: 0, pc: 103 },
    Tone { _number: 865, name: "Castanets", msb: 121, lsb: 1, pc: 115 },
    Tone { _number: 866, name: "Taiko", msb: 121, lsb: 0, pc: 116 },
    Tone { _number: 867, name: "Concert BD", msb: 121, lsb: 1, pc: 116 },
    Tone { _number: 868, name: "Melo. Tom 1", msb: 121, lsb: 0, pc: 117 },
    Tone { _number: 869, name: "Melo. Tom 2", msb: 121, lsb: 1, pc: 117 },
    Tone { _number: 870, name: "Synth Drum", msb: 121, lsb: 0, pc: 118 },
    Tone { _number: 871, name: "808 Tom", msb: 121, lsb: 1, pc: 118 },
    Tone { _number: 872, name: "Elec Perc", msb: 121, lsb: 2, pc: 118 },
    Tone { _number: 873, name: "Reverse Cymb", msb: 121, lsb: 0, pc: 119 },
    Tone { _number: 874, name: "Agogo", msb: 121, lsb: 0, pc: 113 },
    Tone { _number: 875, name: "Woodblock", msb: 121, lsb: 0, pc: 115 },
    Tone { _number: 876, name: "Gt FretNoise", msb: 121, lsb: 0, pc: 120 },
    Tone { _number: 877, name: "Gt Cut Noise", msb: 121, lsb: 1, pc: 120 },
    Tone { _number: 878, name: "String Slap", msb: 121, lsb: 2, pc: 120 },
    Tone { _number: 879, name: "Breath Noise", msb: 121, lsb: 0, pc: 121 },
    Tone { _number: 880, name: "Fl.Key Click", msb: 121, lsb: 1, pc: 121 },
    Tone { _number: 881, name: "Seashore", msb: 121, lsb: 0, pc: 122 },
    Tone { _number: 882, name: "Rain", msb: 121, lsb: 1, pc: 122 },
    Tone { _number: 883, name: "Thunder", msb: 121, lsb: 2, pc: 122 },
    Tone { _number: 884, name: "Wind", msb: 121, lsb: 3, pc: 122 },
    Tone { _number: 885, name: "Stream", msb: 121, lsb: 4, pc: 122 },
    Tone { _number: 886, name: "Bubble", msb: 121, lsb: 5, pc: 122 },
    Tone { _number: 887, name: "Bird 1", msb: 121, lsb: 0, pc: 123 },
    Tone { _number: 888, name: "Dog", msb: 121, lsb: 1, pc: 123 },
    Tone { _number: 889, name: "Horse Gallop", msb: 121, lsb: 2, pc: 123 },
    Tone { _number: 890, name: "Bird 2", msb: 121, lsb: 3, pc: 123 },
    Tone { _number: 891, name: "Telephone 1", msb: 121, lsb: 0, pc: 124 },
    Tone { _number: 892, name: "Telephone 2", msb: 121, lsb: 1, pc: 124 },
    Tone { _number: 893, name: "DoorCreaking", msb: 121, lsb: 2, pc: 124 },
    Tone { _number: 894, name: "Door", msb: 121, lsb: 3, pc: 124 },
    Tone { _number: 895, name: "Scratch", msb: 121, lsb: 4, pc: 124 },
    Tone { _number: 896, name: "Wind Chimes", msb: 121, lsb: 5, pc: 124 },
    Tone { _number: 897, name: "Helicopter", msb: 121, lsb: 0, pc: 125 },
    Tone { _number: 898, name: "Car Engine", msb: 121, lsb: 1, pc: 125 },
    Tone { _number: 899, name: "Car Stop", msb: 121, lsb: 2, pc: 125 },
    Tone { _number: 900, name: "Car Pass", msb: 121, lsb: 3, pc: 125 },
    Tone { _number: 901, name: "Car Crash", msb: 121, lsb: 4, pc: 125 },
    Tone { _number: 902, name: "Siren", msb: 121, lsb: 5, pc: 125 },
    Tone { _number: 903, name: "Train", msb: 121, lsb: 6, pc: 125 },
    Tone { _number: 904, name: "Jetplane", msb: 121, lsb: 7, pc: 125 },
    Tone { _number: 905, name: "Starship", msb: 121, lsb: 8, pc: 125 },
    Tone { _number: 906, name: "Burst Noise", msb: 121, lsb: 9, pc: 125 },
    Tone { _number: 907, name: "Applause", msb: 121, lsb: 0, pc: 126 },
    Tone { _number: 908, name: "Laughing", msb: 121, lsb: 1, pc: 126 },
    Tone { _number: 909, name: "Screaming", msb: 121, lsb: 2, pc: 126 },
    Tone { _number: 910, name: "Punch", msb: 121, lsb: 3, pc: 126 },
    Tone { _number: 911, name: "Heart Beat", msb: 121, lsb: 4, pc: 126 },
    Tone { _number: 912, name: "Footsteps", msb: 121, lsb: 5, pc: 126 },
    Tone { _number: 913, name: "Gun Shot", msb: 121, lsb: 0, pc: 127 },
    Tone { _number: 914, name: "Machine Gun", msb: 121, lsb: 1, pc: 127 },
    Tone { _number: 915, name: "Laser Gun", msb: 121, lsb: 2, pc: 127 },
    Tone { _number: 916, name: "Explosion", msb: 121, lsb: 3, pc: 127 },
    Tone { _number: 917, name: "Standard 1", msb: 86, lsb: 0, pc: 0 },
    Tone { _number: 918, name: "Standard 2", msb: 86, lsb: 0, pc: 1 },
    Tone { _number: 919, name: "Standard 3", msb: 86, lsb: 0, pc: 2 },
    Tone { _number: 920, name: "Rock Kit", msb: 86, lsb: 0, pc: 3 },
    Tone { _number: 921, name: "Jazz Kit", msb: 86, lsb: 0, pc: 4 },
    Tone { _number: 922, name: "Brush Kit", msb: 86, lsb: 0, pc: 5 },
    Tone { _number: 923, name: "Machine Kit", msb: 86, lsb: 0, pc: 6 },
    Tone { _number: 924, name: "R&B T-Analog", msb: 86, lsb: 0, pc: 7 },
    Tone { _number: 925, name: "R&B Mini Kit", msb: 86, lsb: 0, pc: 8 },
    Tone { _number: 926, name: "HipHop Kit", msb: 86, lsb: 0, pc: 9 },
    Tone { _number: 927, name: "R&B Kit", msb: 86, lsb: 0, pc: 10 },
    Tone { _number: 928, name: "Dance Kit 1", msb: 86, lsb: 0, pc: 11 },
    Tone { _number: 929, name: "Dance Kit 2", msb: 86, lsb: 0, pc: 12 },
    Tone { _number: 930, name: "Dance Kit 3", msb: 86, lsb: 0, pc: 13 },
    Tone { _number: 931, name: "GM2 STANDARD", msb: 120, lsb: 0, pc: 0 },
    Tone { _number: 932, name: "GM2 ROOM", msb: 120, lsb: 0, pc: 8 },
    Tone { _number: 933, name: "GM2 POWER", msb: 120, lsb: 0, pc: 16 },
    Tone { _number: 934, name: "GM2 ELECTRIC", msb: 120, lsb: 0, pc: 24 },
    Tone { _number: 935, name: "GM2 ANALOG", msb: 120, lsb: 0, pc: 25 },
    Tone { _number: 936, name: "GM2 JAZZ", msb: 120, lsb: 0, pc: 32 },
    Tone { _number: 937, name: "GM2 BRUSH", msb: 120, lsb: 0, pc: 40 },
    Tone { _number: 938, name: "GM2 ORCHSTRA", msb: 120, lsb: 0, pc: 48 },
    Tone { _number: 939, name: "GM2 SFX", msb: 120, lsb: 0, pc: 56 },
    // The extra tones below are only available on RD700NX, but inadvertedly end up in the 4th (unused) layer of the preset data in RD300NX.
    // I don't believe they can be used in the RD300NX, but at the moment the file can't be parsed without them being listed.
    // For reference, the RD700NX tone list is ~50 longer than RD300NX but the tone number mapping doesn't match.
    Tone { _number: 0, name: "Honky-tonk", msb: 112, lsb: 0, pc: 5 },
    Tone { _number: 0, name: "Reed E.Piano", msb: 112, lsb: 0, pc: 6 },
    Tone { _number: 0, name: "Comp Reed E.P", msb: 112, lsb: 0, pc: 7 }
];