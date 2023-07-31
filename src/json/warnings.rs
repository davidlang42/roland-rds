// The idea is that warnings are a very user focussed string, rather than complex nested validation.

// Where validation tells the user there is fundamentally a problem that means the RD300NX object is invalid,
// a warning only indicates that although valid, the configuration provided may not operate as expected
// in some scenarios.

use crate::roland::live_set::LiveSet;

use super::validation::LayerRanges;

pub trait Warnings {
    //TODO (possibly future feature) interactive fixes where possible
    fn warnings(&self) -> Vec<String>;
}

pub fn split_switch_warning<'a, L: LayerRanges + 'a, I: Iterator<Item = &'a L>>(name: &str, enabled: bool, layers: I) -> Option<String> {
    if !enabled {
        let layers: Vec<usize> = layers.enumerate()
            .filter(|(_, l)| !l.uses_full_range())
            .map(|(i, _)| i).collect();
        if layers.len() > 0 {
            return Some(format!("{} layers {:?} have non-full ranges, but split switch is OFF", name, layers));
        }
    }
    None
}

pub fn tone_remain_warnings(_a: &LiveSet, _b: &LiveSet) -> Vec<String> {
    //TODO warn if patches won't tone_remain properly (and maybe with a secondary classification for "almost" where it should be practically fine even though it isnt technically perfect)
    Vec::new()
}