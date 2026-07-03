use serde_json::json;

use crate::{U_001A, U_001B, VSObjectType, field_types::{VSFieldType, color3::VSColor3, string::VSString, vector2::VSVector2}};

/// Comment instance (not the actual Comment block)
pub struct Comment {
    pub name: VSString,
    pub visual_position: VSVector2,
    pub color: VSColor3,
    pub size: VSVector2
}
impl Comment {
    pub fn new() -> Self {
        Self {
            name: VSString::new(),
            visual_position: VSVector2::new(),
            color: VSColor3::new(),
            size: VSVector2::new()
        }
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut comment = Self::new();
        comment.from_json(json)?;

        Ok(comment)
    }
}
impl VSObjectType for Comment {
    fn to_vs(&self) -> String {
        format!(
            "{U_001A}{U_001A}Comment{U_001A}Position{U_001B}{}{U_001A}Size{U_001B}{}{U_001A}Color{U_001B}{}{U_001A}Name{U_001B}{}",
            self.visual_position.to_vs(),
            self.size.to_vs(),
            self.color.to_vs(),
            self.name.to_vs()
        )
    }

    fn from_vs<'a>(&mut self, vs: &'a str) -> Result<&'a str, &'static str> {
        todo!()
    }

    fn to_json(&self, visual_source: Option<&crate::VisualSource>) -> serde_json::Value {
        json!({
            "Position": self.visual_position.to_json(),
            "Color": self.color.to_json(),
            "Size": self.size.to_json(),
            "Name": self.name.to_json()
        })
    }

    fn from_json(&mut self, mut json: serde_json::Value) -> Result<(), &'static str> {
        let visual_position = VSVector2::from_json(json.get_mut("Position").ok_or("'Position' doesn't exist in Comment json")?.take())?;
        let color = VSColor3::from_json(json.get_mut("Color").ok_or("'Color' doens't exist in Comment json")?.take())?;
        let size = VSVector2::from_json(json.get_mut("Size").ok_or("'Size' doesn't exist in Comment json")?.take())?;
        let name = VSString::from_json(json.get_mut("Name").ok_or("'Name' doesn't exist in Comment json")?.take())?;

        self.visual_position = visual_position;
        self.color = color;
        self.size = size;
        self.name = name;

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Comment"
    }
}