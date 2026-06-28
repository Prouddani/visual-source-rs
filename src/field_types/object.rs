use crate::field_types::{VSFieldType, VisualSourceParserError};

pub struct VSObject(pub String);
impl VSObject {
    pub fn new() -> Self {
        Self(String::new())
    }
}
impl From<&str> for VSObject {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
impl From<String> for VSObject {
    fn from(value: String) -> Self {
        Self(value)
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

    fn get_type(&self) -> &'static str {
        "Object"
    }
}