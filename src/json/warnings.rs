// The idea is that warnings are a very user focussed string, rather than complex nested validation.

// Where validation tells the user there is fundamentally a problem that means the RD300NX object is invalid,
// a warning only indicates that although valid, the configuration provided may not operate as expected
// in some scenarios.

use strum::IntoEnumIterator;

use crate::roland::{live_set::{LiveSet, mfx::Mfx, reverb::Reverb, chorus::Chorus}, layers::InternalLayer, types::enums::{Layer, PedalFunction}};

use super::validation::LayerRanges;

pub trait Warnings {
    fn warnings(&self) -> Vec<String>;
}

pub fn split_switch_warning<'a, L: LayerRanges + 'a, I: Iterator<Item = &'a L>>(name: &str, enabled: bool, layers: I) -> Option<String> {
    if !enabled {
        let layers: Vec<usize> = layers.enumerate()
            .filter(|(_, l)| l.is_enabled() && !l.uses_full_range())
            .map(|(i, _)| i).collect();
        if layers.len() > 0 {
            return Some(format!("{} layers {:?} have non-full ranges, but split switch is OFF", name, layers));
        }
    }
    None
}

pub fn tone_remain_warnings(a: &LiveSet, b: &LiveSet, fc1_from_system: Option<PedalFunction>, fc2_from_system: Option<PedalFunction>) -> Vec<String> {
    let mut reasons = Vec::new();
    if let Some(reason) = Mfx::tone_remain_warning(
        &a.mfx,
        &b.mfx,
        &a.layers[0].internal.active()
    ) {
        reasons.push(reason);
    }
    if let Some(reason) = Reverb::tone_remain_warning(
        &a.reverb,
        &b.reverb,
        a.layers.iter().map(|l| l.internal.reverb).max().unwrap(),
        b.layers.iter().map(|l| l.internal.reverb).max().unwrap()
    ) {
        reasons.push(reason);
    }
    if let Some(reason) = Chorus::tone_remain_warning(
        &a.chorus,
        &b.chorus,
        a.layers.iter().map(|l| l.internal.chorus).max().unwrap(),
        b.layers.iter().map(|l| l.internal.chorus).max().unwrap()
    ) {
        reasons.push(reason);
    }
    let layer_names: Vec<_> = Layer::iter().collect();
    for i in 0..a.layers.len() {
        if let Some(reason) = InternalLayer::tone_remain_warning(
            &layer_names[i],
            &a.layers[i].internal, 
            &b.layers[i].internal,
            !a.chorus.chorus_type.is_off(),
            a.reverb.reverb_type.is_off(),
            &fc1_from_system.unwrap_or(a.common.fc1_assign),
            &fc1_from_system.unwrap_or(b.common.fc1_assign),
            &fc2_from_system.unwrap_or(a.common.fc2_assign),
            &fc2_from_system.unwrap_or(b.common.fc2_assign)
        ) {
            reasons.push(reason);
        }
    }
    reasons
}
