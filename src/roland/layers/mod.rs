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

use crate::json::{Json, StructuredJson, StructuredJsonError};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct LogicalLayer {
    pub internal: InternalLayer,
    pub external: ExternalLayer,
    pub tone: ToneLayer,
    pub piano: PianoLayer,
    pub unused_e_piano: EPianoLayer,
    pub unused_tone_wheel: ToneWheelLayer
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