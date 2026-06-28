use std::fmt::{Debug, Display, write};

use crate::field_types::VSFieldType;

pub mod field_types;
pub mod editor;
pub mod block;

pub mod hex;

pub const U_001A: &str = "\u{001A}";
pub const U_001B: &str = "\u{001B}";

pub trait VSObjectType {
    fn into_vs(&self) -> String;
    fn from_vs<'a>(&mut self, vs: &'a str) -> Result<&'a str, &'static str>;
}

pub struct VisualSource {
    pub root_objects: Vec<Box<dyn VSObjectType>>,
}
impl Display for VisualSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vs_root_objects = self.root_objects.iter().map(|o| { o.into_vs() }).collect::<Vec<String>>();
        
        write!(f, "{}", vs_root_objects.join(""))
    }
}
impl Debug for VisualSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().escape_debug())
    }
}

impl Display for Box<dyn VSFieldType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs().escape_debug())
    }
}
impl Debug for Box<dyn VSFieldType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs().escape_debug())
    }
}