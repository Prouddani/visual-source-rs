use crate::field_types::{VSFieldType, VisualSourceParserError};

#[derive(Clone, Copy, Debug)]
pub struct VSBool(pub bool);
impl VSBool {
    pub fn new() -> Self {
        Self(false)
    }
}
impl From<bool> for VSBool {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl VSFieldType for VSBool {
    fn into_vs(&self) -> String {
        format!(
            "{}", match self.0 {
                false => 0,
                true => 1,
            }
        )
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        match vs {
            "0" => self.0 = false,
            "1" => self.0 = true,
            _ => return Err("Bool has incorrect value. Must be 0 or 1 (true or false)")
        }
        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Bool"
    }
}