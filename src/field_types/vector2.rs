use std::fmt::Display;

use serde_json::{json, value::Serializer};

use crate::{field_types::{VSFieldType, number::VSNumber}};

#[derive(Clone, Copy, Debug)]
pub struct VSVector2 {
    pub x: VSNumber,
    pub y: VSNumber,
}
impl VSVector2 {
    /// Creates a new Vector2 instance
    pub fn new() -> Self {
        Self {
            x: 0.0.into(),
            y: 0.0.into()
        }
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut vec2 = Self::new();
        vec2.from_json(json)?;

        Ok(vec2)
    }
}
impl<T> From<(T, T)> for VSVector2
where
    T: Into<VSNumber>
{
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
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

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        let split = vs.split(',');
        for (i, sub) in split.enumerate() {
            let field;
            match i {
                0 => field = &mut self.x,
                1 => field = &mut self.y,
                _ => return Err("There is more than two axis for Vector2 input")
            }
            
            let _ = field.from_vs(sub)?;
        }

        Ok(())
    }

    fn into_json(&self) -> serde_json::Value {
        json!({
            "x": self.x.into_json(),
            "y": self.y.into_json(),
            "_ValueType": self.get_type(),
        })
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        let x = match json.get("x").ok_or("Error getting X axis in json for VSVector2")? {
            serde_json::Value::Number(x) => x.as_f64().ok_or("Error converting json number of x axis of VSVector2 into built-in type f64")?,
            _ => return Err("Given value for x axis of VSVector2 is not a number")
        };

        let y = match json.get("y").ok_or("Error getting y axis in json for VSVector2")? {
            serde_json::Value::Number(y) => y.as_f64().ok_or("Error converting json number of y axis of VSVector2 into built-in type f64")?,
            _ => return Err("Given value for y axis of VSVector2 is not a number")
        };

        self.x.0.0 = x;
        self.y.0.0 = y;

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Vector2"
    }
}
impl Display for VSVector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs())
    }
}

#[macro_export]
macro_rules! vs_vec2 {
    ($x:literal, $y:literal) => {
        $crate::field_types::vector2::VSVector2::from(($x, $y))
    }
}