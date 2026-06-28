use crate::{field_types::{VSFieldType, VisualSourceParserError}, hex::Hex};

#[derive(Clone, Copy, Debug)]
pub struct VSNumber(pub Hex);
impl VSNumber {
    pub fn new() -> Self {
        Self(Hex(0.0))
    }
}
impl From<f64> for VSNumber {
    fn from(value: f64) -> Self {
        Self(Hex(value))
    }
}
impl From<Hex> for VSNumber {
    fn from(value: Hex) -> Self {
        Self(value)
    }
}
impl VSFieldType for VSNumber {
    fn into_vs(&self) -> String {
        format!("{}", self.0)
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        self.0 = Hex::from_hex(vs).ok_or("Error parsing hexadecimal into decimal")?; // throws the result, in case the option is None

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Number"
    }
}