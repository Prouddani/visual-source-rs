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

pub trait VSFieldType {
    fn into_vs(&self) -> String;
    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str>;

    fn get_type(&self) -> &'static str;
}

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