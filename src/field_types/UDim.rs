use crate::field_types::{VSFieldType, VisualSourceParserError};

pub struct VSUDim {
    pub v: (f32, f32),
}
impl VSFieldType for VSUDim {}