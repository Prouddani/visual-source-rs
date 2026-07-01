use std::fmt::Display;

use serde_json::json;

use crate::field_types::{VSFieldType, number::VSNumber};

pub struct VSUDim2 {
    pub xscale: VSNumber,
    pub xoffset: VSNumber,
    pub yscale: VSNumber,
    pub yoffset: VSNumber
}
impl VSUDim2 {
    /// Creates a new UDim2 instance
    pub fn new() -> Self {
        Self {
            xscale: 0.0.into(),
            xoffset: 0.0.into(),
            yscale: 0.0.into(),
            yoffset: 0.0.into()
        }
    }
}
impl<T> From<(T, T, T, T)> for VSUDim2
where 
    T: Into<VSNumber>
{
    fn from(value: (T, T, T, T)) -> Self {
        let (xs, xo, ys, yo) = value;
        Self {
            xscale: xs.into(),
            xoffset: xo.into(),
            yscale: ys.into(),
            yoffset: yo.into()
        }
    }
}
impl VSFieldType for VSUDim2 {
    fn into_vs(&self) -> String {
        format!("{},{},{},{}", self.xscale.into_vs(), self.xoffset.into_vs(), self.yscale.into_vs(), self.yoffset.into_vs())
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        let split = vs.split(",").collect::<Vec<_>>();

        let xscale_str = split.get(0).ok_or("Error getting XScale of UDim2")?;
        self.xscale.from_vs(xscale_str)?;

        let xoffset_str = split.get(1).ok_or("Error getting XOffset of UDim2")?;
        self.xoffset.from_vs(xoffset_str)?;

        let yscale_str = split.get(2).ok_or("Error getting YScale of UDim2")?;
        self.yscale.from_vs(yscale_str)?;

        let yoffset_str = split.get(3).ok_or("Error getting YOffset of UDim2")?;
        self.yoffset.from_vs(yoffset_str)?;

        Ok(())
    }

    fn into_json(&self) -> serde_json::Value {
        json!({
            "xS": self.xscale.into_json(),
            "xO": self.xoffset.into_json(),
            "yS": self.yscale.into_json(),
            "yO": self.yoffset.into_json(),
            "_ValueType": self.get_type()
        })
    }

    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        let xscale = json.get("xS").ok_or("XScale property wasn't found in UDim2 json")?.as_number().ok_or("XScale property is not a number")?.as_f64().ok_or("Couldn't convert XScale into f64")?;
        let xoffset = json.get("xO").ok_or("XOffset property wasn't found in UDim2 json")?.as_number().ok_or("XOffset property is not a number")?.as_f64().ok_or("Couldn't convert XOffset into f64")?;
        let yscale = json.get("yS").ok_or("YScale property wasn't found in UDim2 json")?.as_number().ok_or("YScale property is not a number")?.as_f64().ok_or("Couldn't convert YScale into f64")?;
        let yoffset = json.get("yO").ok_or("YOffset property wasn't found in UDim2 json")?.as_number().ok_or("YOffset property is not a number")?.as_f64().ok_or("Couldn't convert YOffset into f64")?;

        self.xscale = xscale.into();
        self.xoffset = xoffset.into();
        self.yscale = yscale.into();
        self.yoffset = yoffset.into();

        Ok(())
    }
    
    fn get_type(&self) -> &'static str {
        "UDim2"
    }
}
impl Display for VSUDim2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs())
    }
}

#[macro_export]
macro_rules! vs_udim2 {
    ($xscale:literal, $xoffset:literal, $yscale:literal, $yoffset:literal) => {
        $crate::field_types::udim2::VSUDim2::from(($xscale, $xoffset, $yscale, $yoffset))
    }
}