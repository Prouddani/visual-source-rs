use crate::{field_types::{VSFieldType, VisualSourceParserError, number::VSNumber}, hex::Hex};

pub struct VSBrickColor(pub VSNumber);
impl From<usize> for VSBrickColor {
    fn from(value: usize) -> Self {
        Self(Hex(value as f64).into())
    }
}
impl VSFieldType for VSBrickColor {
    fn into_vs(&self) -> String {
        format!("{}", self.0.into_vs())
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), VisualSourceParserError> {
        // we get the whole number, since BrickColors don't have fractional part
        let (whole, _) = vs.split_once(".").ok_or(VisualSourceParserError::IncorrectType)?;

        self.0 = Hex(whole.parse::<f64>().or(Err(VisualSourceParserError::IncorrectType))?).into();

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "BrickColor"
    }
}