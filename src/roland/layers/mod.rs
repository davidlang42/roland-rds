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

//TODO (NEXT) combine layer infos into one Layer object, which makes more sense for a user eg. LogicalLayer { InternalLayer, ExternalLayer, Tone, Piano }