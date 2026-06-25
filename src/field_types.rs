use crate::field_types::{bool::VSBool, number::VSNumber, string::VSString, vector2::VSVector2, vector3::VSVector3};

pub mod string;
pub mod object;
pub mod number;
pub mod vector2;
pub mod vector3;
pub mod bool;
// pub mod UDim;
// pub mod UDim2;
// pub mod tuple;
pub mod brickcolor;

#[derive(Debug)]
pub enum VisualSourceParserError {
    IncorrectType,
}

pub trait VSFieldType {
    fn into_vs(&self) -> String;
    fn from_vs(&mut self, vs: &str) -> Result<(), VisualSourceParserError>;

    fn get_type(&self) -> &'static str;
}

pub fn new_field_from_vs_type(t: &str) -> Option<Box<dyn VSFieldType>> {
    Some(match t {
        "String" => Box::new(VSString::new()),
        "Number" => Box::new(VSNumber::new()),
        "Bool" => Box::new(VSBool::new()),
        "Vector2" => Box::new(VSVector2::new()),
        "Vector3" => Box::new(VSVector3::new()),
        _ => return None
    })
}