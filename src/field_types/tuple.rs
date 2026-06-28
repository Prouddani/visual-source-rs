use std::any::{Any, TypeId};

use serde_json::Value;

use crate::{U_001A, U_001B, block::{BlockInput, BlockInputVisibility}, field_types::{VSFieldType, VisualSourceParserError, new_field_from_vs_type, number::VSNumber, string::VSString}};
pub struct VSTuple(pub Vec<VSString>);
impl VSTuple {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn get_from_input_vec<'a>(&self, inputs: &'a Vec<BlockInput>) -> Vec<&'a BlockInput> {
        inputs.iter().filter(|i| {
            self.0.iter().find(|input_name| i.name.to_string() == input_name.to_string()).is_some()
        }).collect::<Vec<&BlockInput>>()
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
    fn into_vs(&self) -> String {
        // entry order
        self.0.iter().map(VSFieldType::into_vs).collect::<Vec<String>>().join(",")
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        use std::mem;

        self.0 = vs.split(",").map(|s| { s.into() }).collect::<Vec<VSString>>();

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Tuple"
    }
}