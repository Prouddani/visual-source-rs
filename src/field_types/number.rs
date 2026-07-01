use std::fmt::Display;

use serde_json::json;

use crate::{field_types::{VSFieldType}, hex::Hex};

#[derive(Clone, Copy, Debug)]
pub struct VSNumber(pub Hex);
impl VSNumber {
    /// Creates a new Number instance
    pub fn new() -> Self {
        Self(Hex(0.0))
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut number = Self::new();
        number.from_json(json)?;

        Ok(number)
    }
}
impl Into<isize> for VSNumber {
    fn into(self) -> isize {
        self.0.into()
    }
}
impl<T> From<T> for VSNumber
where 
    T: Into<f64>
{
    fn from(value: T) -> Self {
        Self(Hex(value.into()))
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

    fn into_json(&self) -> serde_json::Value {
        json!(self.0.0)
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        if let serde_json::Value::Number(number) = json {
            self.0.0 = number.as_f64().ok_or("Given number cannot be transformed into an f64, and, therefore, cannot be converted into VSNumber")?;
        } else {
            return Err("Given serde_json::Value could not be converted into VSNumber, because it has a type other than Number");
        }

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Number"
    }
}
impl Display for VSNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs())
    }
}

#[macro_export]
macro_rules! vs_num {
    ($n:literal) => {
        $crate::field_types::number::VSNumber::from($n)
    }
}