use std::{borrow::Cow, collections::HashMap};

use serde::Serialize;
use strum::IntoEnumIterator;
use validator::{ValidationError, Validate, ValidationErrors};

pub fn valid_chars<const N: usize>(_chars: &[char; N]) -> Result<(), ValidationError> {
    // if username == "xXxShad0wxXx" {
    //     // the value of the username will automatically be added later
    //     return Err(ValidationError::new("terrible_username"));
    // }
    //Ok(())
    //TODO implement
    todo!()
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