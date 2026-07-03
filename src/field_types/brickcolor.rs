use std::fmt::Display;

use serde_json::json;

use crate::field_types::{VSFieldType, color3::VSColor3, number::VSNumber};

pub struct VSBrickColor(pub VSNumber);
impl VSBrickColor {
    /// Creates a new BrickColor instance
    pub fn new() -> Self {
        Self(1.into())
    }

    /// Returns the nearest BrickColor from a rgb value
    pub fn from_rgb(target: (u8, u8, u8)) -> Self {
        let mut brick_color_json: serde_json::Value = serde_json::from_slice(include_bytes!("BrickColor.json")).unwrap();
        let brick_colors = brick_color_json.get_mut("BrickColors").unwrap().as_array_mut().unwrap();

        let mut nearest = 0;
        let mut nearest_distance = f64::MAX;

        for (i, brick_color) in brick_colors.iter().enumerate() {
            let color = brick_color["Color8"].as_array().unwrap();

            let r = color[0].as_u64().unwrap() as u8;
            let g = color[1].as_u64().unwrap() as u8;
            let b = color[2].as_u64().unwrap() as u8;

            let dr = target.0 as f64 - r as f64;
            let dg = target.1 as f64 - g as f64;
            let db = target.2 as f64 - b as f64;

            let dist = dr * dr + dg * dg + db * db;

            if dist < nearest_distance {
                nearest_distance = dist;
                nearest = i;
            }
        }

        Self((nearest as f64).into())
    }
}
impl From<usize> for VSBrickColor {
    fn from(value: usize) -> Self {
        Self((value as u32).into())
    }
}
impl VSFieldType for VSBrickColor {
    fn to_vs(&self) -> String {
        format!("{}", self.0.to_vs())
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        // we get the whole number, since BrickColors don't have fractional part
        let (whole, _) = vs.split_once(".").unwrap_or(("0", ""));

        self.0 = whole.parse::<f64>().or(Err("Unable to parse number from hexadecimal")).unwrap().into();

        Ok(())
    }

    fn to_json(&self) -> serde_json::Value {
        let brick_color_json: serde_json::Value = serde_json::from_slice(include_bytes!("BrickColor.json")).unwrap();
        let brick_colors = brick_color_json.get("BrickColors").unwrap();
        let brick_color = brick_colors.get(self.0.0.0 as usize - 1).unwrap();
        let color = brick_color.get("Color8").unwrap();

        json!({
            "r": color.get(0),
            "g": color.get(1),
            "b": color.get(2),
            "_ValueType": self.get_type()
        })
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        if let serde_json::Value::Number(number) = json {
            self.0.0.0 = number.as_f64().ok_or("Given number cannot be transformed into an f64, and, therefore, cannot be converted into VSBrickColor").unwrap();
        } else {
            return Err("Given serde_json::Value could not be converted into VSBrickColor, because it has a type other than Number");
        }

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "BrickColor"
    }
}
impl Display for VSBrickColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_vs())
    }
}