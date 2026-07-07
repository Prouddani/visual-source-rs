use serde_json::json;

use crate::{U_001A, U_001B, VSObjectType, field_types::{VSFieldType, number::VSNumber, vector2::VSVector2}};

#[derive(Clone, Copy, Debug)]
pub struct Editor {
    pub camera_position: VSVector2,
    pub camera_zoom: VSNumber,
}
impl Editor {
    /// Creates a new Editor instance
    pub fn new(
        camera_position: impl Into<VSVector2>,
        camera_zoom: impl Into<VSNumber>
    ) -> Box<Self> {
        Box::new(Self {
            camera_position: camera_position.into(),
            camera_zoom: camera_zoom.into()
        })
    }

    pub fn from_json(mut json: serde_json::Value) -> Result<Self, &'static str> {
        let camera_position = VSVector2::from_json(json.get_mut("CameraPosition").ok_or("'CameraPosition' does not exist in Editor json")?.take())?;
        let camera_zoom = VSNumber::from_json(json.get_mut("CameraZoom").ok_or("'CameraZoom' does not exist in Editor json")?.take())?;

        Ok(Self {
            camera_position,
            camera_zoom
        })
    }
}
impl VSObjectType for Editor {
    fn to_json(&self) -> serde_json::Value {
        let camera_position = self.camera_position.to_json();
        let camera_zoom = self.camera_zoom.to_json();

        json!({
            "CameraPosition": camera_position,
            "CameraZoom": camera_zoom
        })
    }

    fn from_json(&mut self, mut json: serde_json::Value) -> Result<(), &'static str> {
        self.camera_position = VSVector2::from_json(json.get_mut("CameraPosition").ok_or("Error getting CameraPosition of Editor, from json")?.take())?;
        self.camera_zoom = VSNumber::from_json(json.get_mut("CameraZoom").ok_or("Error getting CameraZoom of Editor, from json")?.take())?;

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Editor"
    }
}