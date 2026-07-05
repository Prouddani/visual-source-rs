use std::{fmt::Display};
use serde_json::json;

use crate::{block::{BlockInput}, field_types::{VSFieldType, string::VSString}};

pub struct VSTuple;
impl VSTuple {
    /// Creates a new Tuple instance
    pub fn new() -> Self {
        Self
    }

    /// Returns the inputs that camoflage as tuple parameters (each input inside a tuple)
    pub fn get_from_input_vec<'a>(self_input: &BlockInput, inputs: &'a Vec<BlockInput>) -> Vec<&'a BlockInput> {
        inputs.iter().filter(|i| {
            i.is_of_tuple(self_input.name.to_string())
        }).collect::<Vec<&BlockInput>>()
    }

    /// Returns the inputs that camoflage as tuple parameters (each input inside a tuple)
    pub fn get_mut_from_input_vec<'a>(self_input: &BlockInput, inputs: &'a mut Vec<BlockInput>) -> Vec<&'a mut BlockInput> {
        inputs.iter_mut().filter(|i| {
            i.is_of_tuple(self_input.name.to_string())
        }).collect::<Vec<&mut BlockInput>>()
    }
}
impl VSFieldType for VSTuple {
    fn to_vs(&self) -> String {
        // entry order
        String::new()
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        Ok(())
    }

    fn to_json(&self) -> serde_json::Value {
        json!(String::new())
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Tuple"
    }
}
impl Display for VSTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_vs())
    }
}