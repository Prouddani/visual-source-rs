use std::{fmt::Display};
use serde_json::json;

use crate::{block::{BlockInput}, field_types::{VSFieldType, string::VSString}};

pub struct VSTuple(pub Vec<VSString>);
impl VSTuple {
    /// Creates a new Tuple instance
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Returns the inputs that camoflage as tuple parameters (each input inside a tuple)
    pub fn get_from_input_vec<'a>(&self, inputs: &'a Vec<BlockInput>) -> Vec<&'a BlockInput> {
        inputs.iter().filter(|i| {
            self.0.iter().find(|input_name| i.name.to_string() == input_name.to_string()).is_some()
        }).collect::<Vec<&BlockInput>>()
    }

    /// Returns the inputs that camoflage as tuple parameters (each input inside a tuple)
    pub fn get_mut_from_input_vec<'a>(&self, inputs: &'a mut Vec<BlockInput>) -> Vec<&'a mut BlockInput> {
        inputs.iter_mut().filter(|i| {
            self.0.iter().find(|input_name| i.name.to_string() == input_name.to_string()).is_some()
        }).collect::<Vec<&mut BlockInput>>()
    }
}
impl From<Vec<&str>> for VSTuple {
    fn from(value: Vec<&str>) -> Self {
        Self(value.into_iter().map(Into::into).collect::<Vec<VSString>>())
    }
}
impl From<Vec<String>> for VSTuple {
    fn from(value: Vec<String>) -> Self {
        Self(value.into_iter().map(Into::into).collect::<Vec<VSString>>())
    }
}
impl From<Vec<VSString>> for VSTuple {
    fn from(value: Vec<VSString>) -> Self {
        Self(value)
    }
}
impl VSFieldType for VSTuple {
    fn to_vs(&self) -> String {
        // entry order
        self.0.iter().map(VSFieldType::to_vs).collect::<Vec<String>>().join(",")
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        self.0 = vs.split(",").map(|s| { s.into() }).collect::<Vec<VSString>>();

        Ok(())
    }

    fn to_json(&self) -> serde_json::Value {
        json!(self.0.iter().map(VSFieldType::to_vs).collect::<Vec<String>>())
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        self.0.clear();
        if let serde_json::Value::Array(array) = json {
            for value in array {
                if let serde_json::Value::String(_) = value {
                    self.0.push({
                        let mut vsstring = VSString::new();
                        vsstring.from_json(value)?;

                        vsstring
                    })
                } else {
                    return Err("A tuple parameter input name is not a string. idk how but, somehow you broke the easiest thing to do ;-;");
                }
            }
        } else {
            return Err("Given json is not an array. Therefore, cannot be converted into a VSString")
        }

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