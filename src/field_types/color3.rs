use serde_json::json;

use crate::field_types::{VSFieldType, number::VSNumber};

pub struct VSColor3(pub VSNumber, pub VSNumber, pub VSNumber);
impl VSColor3 {
    pub fn new() -> Self {
        Self(VSNumber::new(), VSNumber::new(), VSNumber::new())
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut color = Self::new();
        color.from_json(json)?;

        Ok(color)
    }
}
impl<T> From<(T, T, T)> for VSColor3
where 
    T: Into<VSNumber>
{
    fn from(value: (T, T, T)) -> Self {
        Self(value.0.into(), value.1.into(), value.2.into())
    }
}
impl VSFieldType for VSColor3 {
    fn into_vs(&self) -> String {
        format!("{},{},{}", self.0.into_vs(), self.1.into_vs(), self.2.into_vs())
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        let split = vs.split(",").collect::<Vec<_>>();

        self.0 = VSNumber::from(split.get(0).ok_or("Unable to get Red property of Visual Source for Color3")?.parse::<f64>().or(Err("Error parsing color channel into f64 for Color3"))?);
        self.1 = VSNumber::from(split.get(1).ok_or("Unable to get Green property of Visual Source for Color3")?.parse::<f64>().or(Err("Error parsing color channel into f64 for Color3"))?);
        self.2 = VSNumber::from(split.get(2).ok_or("Unable to get Blue property of Visual Source for Color3")?.parse::<f64>().or(Err("Error parsing color channel into f64 for Color3"))?);

        Ok(())
    }

    fn into_json(&self) -> serde_json::Value {
        json!({
            "r": self.0.into_json(),
            "g": self.1.into_json(),
            "b": self.2.into_json(),
            "_ValueType": "Color3"
        })
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        self.0 = VSNumber::from(json.get("r").ok_or("Red channel of Color3 json does not exist")?.as_f64().ok_or("Error parsing Red channel of Color3 json into f64")?);
        self.1 = VSNumber::from(json.get("g").ok_or("Green channel of Color3 json does not exist")?.as_f64().ok_or("Error parsing Green channel of Color3 json into f64")?);
        self.2 = VSNumber::from(json.get("b").ok_or("Blue channel of Color3 json does not exist")?.as_f64().ok_or("Error parsing Blue channel of Color3 json into f64")?);

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Color3"
    }
}