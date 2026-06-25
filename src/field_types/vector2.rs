use crate::{U_001A, U_001B, field_types::{VSFieldType, VisualSourceParserError, number::VSNumber}, hex::Hex};

#[derive(Clone, Copy, Debug)]
pub struct VSVector2 {
    pub x: VSNumber,
    pub y: VSNumber,
}
impl VSVector2 {
    pub fn new() -> Self {
        Self {
            x: Hex(0.0).into(),
            y: Hex(0.0).into()
        }
    }
}
impl From<(f64, f64)> for VSVector2 {
    fn from(value: (f64, f64)) -> Self {
        Self {
            x: Hex(value.0).into(),
            y: Hex(value.1).into(),
        }
    }
}
impl VSFieldType for VSVector2 {
    fn into_vs(&self) -> String {
        format!(
            "{},{}",
            self.x.into_vs(),
            self.y.into_vs()
        )
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), VisualSourceParserError> {
        let split = vs.split(',');
        for (i, sub) in split.enumerate() {
            let field;
            match i {
                0 => field = &mut self.x,
                1 => field = &mut self.y,
                _ => return Err(VisualSourceParserError::IncorrectType)
            }
            
            let _ = field.from_vs(sub)?;
        }

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Vector2"
    }
}