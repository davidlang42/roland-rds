use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;
use strum::IntoEnumIterator;
use validator::{ValidationError, Validate, ValidationErrors};
use std::hash::Hash;
use std::cmp::Eq;

use crate::roland::layers::LogicalLayer;
use crate::roland::types::enums::{ButtonFunction, PedalFunction};
use crate::roland::types::notes::PianoKey;

pub fn valid_chars<const N: usize>(chars: &[char; N]) -> Result<(), ValidationError> {
    const MIN: u8 = 32;
    const MAX: u8 = 127;
    let mut invalid = Vec::new();
    for (i, c) in chars.iter().enumerate() {
        let ascii = *c as u8;
        if ascii < MIN || ascii > MAX {
            invalid.push(i);
        }
    }
    if invalid.len() == 0 {
        Ok(())
    } else {
        let mut e = ValidationError::new("Characters out of range");
        e.add_param(Cow::from("Min"), &MIN);
        e.add_param(Cow::from("Max"), &MAX);
        e.add_param(Cow::from("CharIndicies"), &invalid);
        Err(e)
    }
}

pub fn validate_boxed_array<T: Validate, const N: usize>(boxed_array: &Box<[T; N]>) -> Vec<Result<(), ValidationErrors>> {
    boxed_array.as_ref().iter().map(|t| t.validate()).collect()
}

pub fn valid_boxed_elements<T: Validate, const N: usize>(boxed_array: &Box<[T; N]>) -> Result<(), ValidationError> {
    let mut errors = Vec::new();
    for (i, t) in boxed_array.iter().enumerate() {
        if let Err(error) = t.validate() {
            errors.push((i, error));
        }
    }
    if errors.len() == 0 {
        Ok(())
    } else {
        let mut e = ValidationError::new("Boxed array element(s) failed validation");
        let indicies: Vec<usize> = errors.iter().map(|(x, _)| *x).collect();
        e.add_param(Cow::from("ArrayIndicies"), &indicies);
        let error_objects: Vec<ValidationErrors> = errors.into_iter().map(|(_, x)| x).collect();
        e.add_param(Cow::from("Errors"), &error_objects);
        Err(e)
    }
}

pub fn validate_control_change(cc: &u8) -> Result<(), ValidationErrors> {
    if *cc > 127 {
        Err(out_of_range_err("ControlChange", &0, &127))
    } else {
        Ok(())
    }
}

pub fn contains_all_keys<K: IntoEnumIterator + Hash + Eq + Serialize, V>(map: &HashMap<K, V>) -> Result<(), ValidationError> {
    let mut missing = Vec::new();
    for key in K::iter() {
        if !map.contains_key(&key) {
            missing.push(key);
        }
    }
    if missing.len() == 0 {
        Ok(())
    } else {
        let mut e = ValidationError::new("HashMap missing required keys");
        e.add_param(Cow::from("MissingKeys"), &missing);
        Err(e)
    }
}

pub fn out_of_range_err<T: Serialize>(field: &'static str, min: &T, max: &T) -> ValidationErrors {
    let mut e = ValidationErrors::new();
    let mut range_err = ValidationError::new("Out of range");
    range_err.add_param(Cow::from("Min"), min);
    range_err.add_param(Cow::from("Max"), max);
    e.add(field, range_err);
    e
}

pub fn unused_by_rd300nx_err<T: Serialize>(field: &'static str, unused_value: &T) -> ValidationErrors {
    let mut e = ValidationErrors::new();
    let mut unused_err = ValidationError::new("Selected value not available on the RD300NX");
    unused_err.add_param(Cow::from("UnusedValue"), unused_value);
    e.add(field, unused_err);
    e
}

pub fn matching_piano_tone(layer: &LogicalLayer) -> Result<(), ValidationError> {
    if let Some(piano_tone) = layer.tone.tone_number.as_piano_tone() {
        if layer.piano.tone_number != piano_tone {
            let mut tone_err = ValidationError::new("Piano tone does not match tone number");
            tone_err.add_param(Cow::from("PianoToneNumber"), &layer.piano.tone_number);
            tone_err.add_param(Cow::from("ToneNumber"), &piano_tone);
            return Err(tone_err);
        }
    }
    Ok(())
}

// ValidationErrors::merge_all() is broken and will falsely return Ok(()) without the additional steps below
pub fn merge_all_fixed(parent: Result<(), ValidationErrors>, field: &'static str, children: Vec<Result<(), ValidationErrors>>) -> Result<(), ValidationErrors> {
    let results: Vec<_> = children.into_iter().map(|child| {
        let mut result = Ok(());
        result = ValidationErrors::merge(result, field, child);
        result
    }).collect();
    ValidationErrors::merge_all(parent, field, results)
}

pub trait LayerRanges {
    fn is_enabled(&self) -> bool;
    fn get_range_upper(&self) -> PianoKey;
    fn get_range_lower(&self) -> PianoKey;
    fn get_velocity_upper(&self) -> u8;
    fn get_velocity_lower(&self) -> u8;

    fn uses_full_range(&self) -> bool {
        self.get_range_lower() == PianoKey::A0 && self.get_range_upper() == PianoKey::C8
    }
}

pub fn valid_key_range<T: LayerRanges>(layer: &T) -> Result<(), ValidationError> {
    if layer.get_range_upper() < layer.get_range_lower() {
        let mut e = ValidationError::new("range_upper is less than range_lower");
        e.add_param(Cow::from("range_lower"), &layer.get_range_lower());
        e.add_param(Cow::from("range_upper"), &layer.get_range_upper());
        Err(e)
    } else {
        Ok(())
    }
}

pub fn valid_velocity_range<T: LayerRanges>(layer: &T) -> Result<(), ValidationError> {
    if layer.get_velocity_upper() < layer.get_velocity_lower() {
        let mut e = ValidationError::new("velocity_range_upper is less than velocity_range_lower");
        e.add_param(Cow::from("velocity_range_lower"), &layer.get_velocity_lower());
        e.add_param(Cow::from("velocity_range_upper"), &layer.get_velocity_upper());
        Err(e)
    } else {
        Ok(())
    }
}

pub fn not_system_only_button_function(value: &ButtonFunction) -> Result<(), ValidationError> {
    if value.is_system_only() {
        let mut e = ValidationError::new("System only ButtonFunction used for LiveSet");
        e.add_param(Cow::from("button_function"), value);
        Err(e)
    } else {
        Ok(())
    }
}

pub fn not_system_only_pedal_function(value: &PedalFunction) -> Result<(), ValidationError> {
    if value.is_system_only() {
        let mut e = ValidationError::new("System only PedalFunction used for LiveSet");
        e.add_param(Cow::from("pedal_function"), value);
        Err(e)
    } else {
        Ok(())
    }
}