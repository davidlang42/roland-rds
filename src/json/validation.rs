use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;
use strum::IntoEnumIterator;
use validator::{ValidationError, Validate, ValidationErrors};

pub fn valid_chars<const N: usize>(chars: &[char; N]) -> Result<(), ValidationError> {
    const MIN: u8 = 32;
    const MAX: u8 = 127;
    for (i, c) in chars.iter().enumerate() {
        let ascii = *c as u8;
        if ascii < MIN || ascii > MAX {
            let mut e = ValidationError::new("Chracter out of range");
            e.add_param(Cow::from("Min"), &MIN);
            e.add_param(Cow::from("Max"), &MAX);
            e.add_param(Cow::from("CharIndex"), &i);
            return Err(e);
        }
    }
    Ok(())
}

pub fn validate_boxed_array<T: Validate, const N: usize>(boxed_array: &Box<[T; N]>) -> Vec<Result<(), ValidationErrors>> {
    boxed_array.as_ref().iter().map(|t| t.validate()).collect()
}

pub fn valid_boxed_elements<T: Validate, const N: usize>(_boxed_array: &Box<[T; N]>) -> Result<(), ValidationError> {
    //boxed_array.as_ref().iter().map(|t| t.validate()).collect()
    //TODO implement
    todo!()
}

pub fn validate_control_change(cc: &u8) -> Result<(), ValidationErrors> {
    if *cc > 127 {
        Err(out_of_range_err("ControlChange", &0, &127))
    } else {
        Ok(())
    }
}

pub fn contains_all_keys<K: IntoEnumIterator, V>(_map: &HashMap<K, V>) -> Result<(), ValidationError> {
    //TODO implement
    todo!()
}

pub fn out_of_range_err<T: Serialize>(field: &'static str, min: &T, max: &T) -> ValidationErrors {
    let mut e = ValidationErrors::new();
    let mut range_err = ValidationError::new("Out of range");
    range_err.add_param(Cow::from("Min"), min);
    range_err.add_param(Cow::from("Max"), max);
    e.add(field, range_err);
    e
}