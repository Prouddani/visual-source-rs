use std::fmt::Display;

use crate::field_types::{VSFieldType};

pub struct VSObject(pub String);
impl VSObject {
    /// Creates a new Object instance
    pub fn new() -> Self {
        Self(String::new())
    }

    /// Sets the value of the Object as the path
    pub fn from_path(path: impl Into<String>) -> Self {
        Self(path.into())
    }
}
impl VSFieldType for VSObject {
    fn into_vs(&self) -> String {
        format!("{}", self.0)
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        self.0 = vs.to_string();

        Ok(())
    }

    fn into_json(&self) -> serde_json::Value {
        serde_json::Value::String(self.into_vs())
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        if let serde_json::Value::String(string) = json {
            self.0 = string
        } else {
            return Err("Given json is not a string. Therefore, cannot be converted into a VSString")
        }

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Object"
    }
}
impl Display for VSObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs())
    }
}

#[macro_export]
macro_rules! vs_obj {
    ($path:literal) => {
        $crate::field_types::object::VSObject::from_path($path)
    }
}