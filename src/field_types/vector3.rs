use std::fmt::Display;

use serde_json::json;

use crate::{field_types::{VSFieldType, number::VSNumber}};

#[derive(Clone, Copy, Debug)]
pub struct VSVector3 {
    pub x: VSNumber,
    pub y: VSNumber,
    pub z: VSNumber,
}
impl VSVector3 {
    /// Creates a new Vector3 instance
    pub fn new() -> Self {
        Self {
            x: 0.0.into(),
            y: 0.0.into(),
            z: 0.0.into()
        }
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut vec3 = Self::new();
        vec3.from_json(json)?;

        Ok(vec3)
    }
}
impl<T> From<(T, T, T)> for VSVector3
where
    T: Into<VSNumber>
{
    fn from(value: (T, T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
            z: value.2.into()
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

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        let split = vs.split(',');
        for (i, sub) in split.enumerate() {
            let field;
            match i {
                0 => field = &mut self.x,
                1 => field = &mut self.y,
                2 => field = &mut self.z,
                _ => return Err("There are more than three axis for Vector3 input")
            }
            
            let _ = field.from_vs(sub)?;
        }

        Ok(())
    }

    fn into_json(&self) -> serde_json::Value {
        json!({
            "x": self.x.into_json(),
            "y": self.y.into_json(),
            "z": self.z.into_json(),
            "_ValueType": self.get_type(),
        })
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        let x = match json.get("x").ok_or("Error getting X axis in json for VSVector3")? {
            serde_json::Value::Number(x) => x.as_f64().ok_or("Error converting json number of x axis of VSVector3 into built-in type f64")?,
            _ => return Err("Given value for x axis of VSVector3 is not a number")
        };

        let y = match json.get("y").ok_or("Error getting y axis in json for VSVector3")? {
            serde_json::Value::Number(y) => y.as_f64().ok_or("Error converting json number of y axis of VSVector3 into built-in type f64")?,
            _ => return Err("Given value for y axis of VSVector3 is not a number")
        };

        let z = match json.get("z").ok_or("Error getting z axis in json for VSVector3")? {
            serde_json::Value::Number(z) => z.as_f64().ok_or("Error converting json number of z axis of VSVector3 into built-in type f64")?,
            _ => return Err("Given value for z axis of VSVector3 is not a number")
        };

        self.x.0.0 = x;
        self.y.0.0 = y;
        self.z.0.0 = y;

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Vector3"
    }
}
impl Display for VSVector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs())
    }
}

#[macro_export]
macro_rules! vs_vec3 {
    ($x:literal, $y:literal, $z:literal) => {
        $crate::field_types::vector3::VSVector3::from(($x, $y, $z))
    }
}