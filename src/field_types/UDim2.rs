use crate::field_types::{UDim::VSUDim, VSFieldType};
use crate::field_types::{VSFieldType, VisualSourceParserError};

pub struct VSUDim2 {
    pub x: VSUDim,
    pub y: VSUDim,
}
impl VSFieldType for VSUDim2 {}