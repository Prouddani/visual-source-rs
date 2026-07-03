use serde_json::json;

use crate::field_types::VSFieldType;

pub struct VSNil;
impl VSFieldType for VSNil {
    fn into_vs(&self) -> String {
        "".to_string()
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        Ok(())
    }

    fn into_json(&self) -> serde_json::Value {
        json!("")
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Nil"
    }
}