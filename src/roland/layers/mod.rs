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

#[derive(Serialize, Deserialize, Debug)]
pub struct LogicalLayer {
    pub internal: InternalLayer,
    pub external: ExternalLayer,
    pub tone: ToneLayer,
    pub piano: PianoLayer,
    pub e_piano: EPianoLayer,
    pub tone_wheel: ToneWheelLayer
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
                e_piano: ep.remove(0),
                tone_wheel: tw.remove(0),
            });
        }
        layers.try_into().unwrap()
    }
}