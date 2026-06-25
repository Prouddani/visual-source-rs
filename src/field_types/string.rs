use std::fmt::Display;

use crate::field_types::{VSFieldType, VisualSourceParserError};

#[derive(Clone, Debug)]
pub struct VSString(pub String);
impl VSString {
    pub fn new() -> Self {
        Self(String::new())
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
impl VSFieldType for VSString {
    fn into_vs(&self) -> String {
        format!("{}", &self.0)
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), VisualSourceParserError> {
        self.0 = vs.to_string();

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