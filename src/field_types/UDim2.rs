use std::fmt::Display;

use crate::field_types::{VSFieldType, number::VSNumber};

pub struct VSUDim2 {
    pub xscale: VSNumber,
    pub xoffset: VSNumber,
    pub yscale: VSNumber,
    pub yoffset: VSNumber
}
impl VSUDim2 {
    pub fn new() -> Self {
        Self {
            xscale: 0.into(),
            xoffset: 0.into(),
            yscale: 0.into(),
            yoffset: 0.into()
        }
    }
}
impl<T> From<(T, T, T, T)> for VSUDim2
where 
    T: Into<VSNumber>
{
    fn from(value: (T, T, T, T)) -> Self {
        Self {
            xscale: value.0.into(),
            xoffset: value.1.into(),
            yscale: value.2.into(),
            yoffset: value.3.into()
        }
    }
}
impl VSFieldType for VSUDim2 {
    fn into_vs(&self) -> String {
        format!("{},{},{},{}", self.xscale, self.xoffset, self.yscale, self.yoffset)
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), &'static str> {
        let split = vs.split(",").collect::<Vec<_>>();

        let xscale_str = split.get(0).ok_or("Error getting XScale property of UDim2")?;
        self.xscale.from_vs(xscale_str)?;

        let xoffset_str = split.get(0).ok_or("Error getting XOffset property of UDim2")?;
        self.xoffset.from_vs(xoffset_str)?;
        
        let yscale_str = split.get(0).ok_or("Error getting YScale property of UDim2")?;
        self.yscale.from_vs(yscale_str)?;
        
        let xoffset_str = split.get(0).ok_or("Error getting XOffset property of UDim2")?;
        self.xoffset.from_vs(xoffset_str)?;

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
        VSUDim2::from(($xscale, $xoffset, $yscale, $yoffset))
    }
}