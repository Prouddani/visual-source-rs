use crate::{U_001B, field_types::{VSFieldType, VisualSourceParserError, new_field_from_vs_type, string::VSString}};

pub struct VSTuple(Vec<Box<dyn VSFieldType>>);
impl VSFieldType for VSTuple {
    fn into_vs(&self) -> String {
        format!(
            "{}",
            self.0.iter().map(|v| {
                v.into_vs()
            }).collect::<Vec<String>>().join(U_001B)
        )
    }

    fn from_vs(&mut self, vs: &str) -> Result<(), VisualSourceParserError> {
        self.0.clear();
        let mut data_buffer = String::new();

        for c in vs.chars() {
            if c.to_string() == U_001B {
                if !data_buffer.is_empty() {
                    let result = new_matching_vs_type(&data_buffer).ok_or(VisualSourceParserError::IncorrectType)?;
                    self.0.push(result);
                }

                data_buffer.clear();
            }
        }

        Ok(())
    }
}