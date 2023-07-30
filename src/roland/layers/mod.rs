use schemars::JsonSchema;

mod internal;
mod external;
mod tone;
mod piano;
mod e_piano;
mod tone_wheel;

pub use internal::InternalLayer;
pub use external::ExternalLayer;
pub use tone::ToneLayer;
pub use piano::PianoLayer;
pub use e_piano::EPianoLayer;
pub use tone_wheel::ToneWheelLayer;
use validator::{Validate, ValidationErrors};

use crate::json::{Json, StructuredJson, StructuredJsonError, validation::matching_piano_tone};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct LogicalLayer {
    pub internal: InternalLayer,
    pub external: ExternalLayer,
    pub tone: ToneLayer,
    pub piano: PianoLayer,
    pub unused_e_piano: EPianoLayer,
    pub unused_tone_wheel: ToneWheelLayer
}

impl Validate for LogicalLayer {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        let mut r = Ok(());
        r = ValidationErrors::merge(r, "internal", self.internal.validate());
        r = ValidationErrors::merge(r, "external", self.external.validate());
        r = ValidationErrors::merge(r, "tone", self.tone.validate());
        r = ValidationErrors::merge(r, "piano", self.piano.validate());
        r = ValidationErrors::merge(r, "tone/piano", matching_piano_tone(&self.tone, &self.piano));
        //r = ValidationErrors::merge(r, "unused_e_piano", self.unused_e_piano.validate());
        //r = ValidationErrors::merge(r, "unused_tone_wheel", self.unused_tone_wheel.validate());
        r
    }
}

impl LogicalLayer {
    pub fn from_layers<const N: usize>(internal: Box<[InternalLayer; N]>, external: Box<[ExternalLayer; N]>, tone: Box<[ToneLayer; N]>, piano: Box<[PianoLayer; N]>, e_piano: Box<[EPianoLayer; N]>, tone_wheel: Box<[ToneWheelLayer; N]>) -> Box<[Self; N]> {
        let mut int: Vec<InternalLayer> = internal.into_iter().collect();
        let mut ext: Vec<ExternalLayer> = external.into_iter().collect();
        let mut tone: Vec<ToneLayer> = tone.into_iter().collect();
        let mut pno: Vec<PianoLayer> = piano.into_iter().collect();
        let mut ep: Vec<EPianoLayer> = e_piano.into_iter().collect();
        let mut tw: Vec<ToneWheelLayer> = tone_wheel.into_iter().collect();
        let mut layers = Vec::new();
        for _ in 0..N {
            layers.push(Self {
                internal: int.remove(0),
                external: ext.remove(0),
                tone: tone.remove(0),
                piano: pno.remove(0),
                unused_e_piano: ep.remove(0),
                unused_tone_wheel: tw.remove(0),
            });
        }
        Box::new(layers.try_into().unwrap())
    }
}

impl Json for LogicalLayer {
    fn to_structured_json(&self) -> StructuredJson {
        StructuredJson::SingleJson(self.to_json())
    }

    fn from_structured_json(structured_json: StructuredJson) -> Result<Self, StructuredJsonError> {
        Self::from_json(structured_json.to_single_json()?).map_err(|e| e.into())
    }

    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    fn from_json(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}