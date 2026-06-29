use std::fmt::Display;

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