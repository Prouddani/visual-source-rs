use crate::{U_001A, U_001B, field_types::{VSFieldType, VisualSourceParserError, number::VSNumber}, hex::Hex};

#[derive(Clone, Copy, Debug)]
pub struct VSVector3 {
    pub x: VSNumber,
    pub y: VSNumber,
    pub z: VSNumber,
}
impl VSVector3 {
    pub fn new() -> Self {
        Self {
            x: Hex(0.0).into(),
            y: Hex(0.0).into(),
            z: Hex(0.0).into()
        }
    }
}
impl From<(f64, f64, f64)> for VSVector3 {
    fn from(value: (f64, f64, f64)) -> Self {
        Self {
            x: Hex(value.0).into(),
            y: Hex(value.1).into(),
            z: Hex(value.2).into(),
        }
    }
}
impl VSFieldType for VSVector3 {
    fn into_vs(&self) -> String {
        format!(
            "{},{},{}",
            self.x.into_vs(),
            self.y.into_vs(),
            self.z.into_vs()
        )
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), VisualSourceParserError> {
        let split = vs.split(',');
        for (i, sub) in split.enumerate() {
            let field;
            match i {
                0 => field = &mut self.x,
                1 => field = &mut self.y,
                2 => field = &mut self.z,
                _ => return Err(VisualSourceParserError::IncorrectType)
            }
            
            let _ = field.from_vs(sub)?;
        }

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Vector3"
    }
}