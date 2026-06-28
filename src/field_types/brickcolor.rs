use std::fmt::Display;

use crate::{field_types::{VSFieldType, number::VSNumber}, hex::Hex};

pub struct VSBrickColor(pub VSNumber);
impl VSBrickColor {
    pub fn new() -> Self {
        Self(1.into())
    }
}
impl From<usize> for VSBrickColor {
    fn from(value: usize) -> Self {
        Self(1.into())
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
impl Display for VSBrickColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs())
    }
}

#[macro_export]
macro_rules! vs_brickcolor {
    ($i:literal) => {
        VSBrickColor::from($i)
    }
}