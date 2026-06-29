use std::fmt::Display;

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