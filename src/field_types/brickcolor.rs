use crate::{field_types::{VSFieldType, VisualSourceParserError, number::VSNumber}, hex::Hex};

pub struct VSBrickColor(pub VSNumber);
impl VSBrickColor {
    pub fn new() -> Self {
        Self(Hex(1.0).into())
    }
}
impl From<usize> for VSBrickColor {
    fn from(value: usize) -> Self {
        Self(Hex(value as f64).into())
    }
}
impl VSFieldType for VSBrickColor {
    fn into_vs(&self) -> String {
        format!("{}", self.0.into_vs())
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        // we get the whole number, since BrickColors don't have fractional part
        let (whole, _) = vs.split_once(".").unwrap_or(("0", ""));

        self.0 = Hex(whole.parse::<f64>().or(Err("Unable to parse number from hexadecimal"))?).into();

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "BrickColor"
    }
}