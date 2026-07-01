use crate::field_types::{bool::VSBool, brickcolor::VSBrickColor, number::VSNumber, object::VSObject, string::VSString, tuple::VSTuple, vector2::VSVector2, vector3::VSVector3};

pub mod string;
pub mod object;
pub mod number;
pub mod vector2;
pub mod vector3;
pub mod bool;
pub mod udim2;
pub mod tuple;
pub mod brickcolor;
pub mod color3;

/// Trait used for all Visual Source values.
pub trait VSFieldType {
    /// Converts the Visual Source value into an actual Visual Source string
    fn into_vs(&self) -> String;

    /// Parses Visual Source into a Visual Source value
    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str>;

    /// Converts the Visual Source value into json
    fn into_json(&self) -> serde_json::Value;

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str>;

    /// Returns a string depicting the Visual Source type
    fn get_type(&self) -> &'static str;
}

/// Given a Visual Source type, let's call it VS_t, from string, it'll return a initialized Visual Source value with VS_t as a type from VSFieldType::get_type
pub fn new_field_from_vs_type(t: &str) -> Option<Box<dyn VSFieldType>> {
    Some(match t {
        "String" | "String?" | "Function" | "Table" | "CFrame" => Box::new(VSString::new()),
        "Number" | "Number?" => Box::new(VSNumber::new()),
        "Bool" | "Bool?" => Box::new(VSBool::new()),
        "Vector2" | "Vector2?" => Box::new(VSVector2::new()),
        "Vector3" | "Vector3?" => Box::new(VSVector3::new()),
        "Object" | "Object?" => Box::new(VSObject::new()),
        "BrickColor" | "BrickColor?" => Box::new(VSBrickColor::new()),
        "Tuple" => Box::new(VSTuple::new()),
        _ => return None
    })
}