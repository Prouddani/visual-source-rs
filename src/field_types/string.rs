use std::{fmt::Display, ops::{Deref, DerefMut}};

use crate::{field_types::VSFieldType};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VSString(pub String);
impl VSString {
    /// Creates a new String instance
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut string = Self::new();
        string.from_json(json)?;

        Ok(string)
    }
}
impl From<String> for VSString {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl From<&str> for VSString {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl Into<String> for VSString {
    fn into(self) -> String {
        self.to_string()
    }
}
impl Deref for VSString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VSString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl VSFieldType for VSString {
    fn to_vs(&self) -> String {
        format!("{}", &self.0)
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        self.0 = vs.to_string();

        Ok(())
    }

    fn to_json(&self) -> serde_json::Value {
        serde_json::Value::String(self.to_vs())
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
        "String"
    }
}
impl Display for VSString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}