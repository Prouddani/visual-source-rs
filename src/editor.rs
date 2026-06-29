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
}
impl VSObjectType for Editor {
    fn into_vs(&self) -> String {
        format!(
            "{U_001A}{U_001A}Editor{U_001A}CameraPosition{U_001B}{}{U_001A}CameraZoom{U_001B}{}",
            self.camera_position.into_vs(),
            self.camera_zoom.into_vs()
        )
    }

    fn from_vs<'a>(&mut self, vs: &'a str) -> Result<&'a str, &'static str> {
        // {U_001A}CameraPosition{U_001B}0,0{U_001A}CameraZoom{U_001B}0.1C2
        let mut setting_property_name = false;
        let mut property_name = String::new();
        let mut setting_property_value = false;
        let mut property_value = String::new();

        let mut double_1a_encounters = 0;
        let mut vs_end = 0;

        for (i, b) in vs.chars().enumerate() {
            vs_end = i;
            
            if b.to_string() == U_001A {
                if setting_property_name { // in case there are 2 u\001A, which is used for objects
                    setting_property_name = false;
                    double_1a_encounters += 1;

                    if double_1a_encounters >= 2 {
                        break;
                    }

                    continue;
                }

                // property name
                setting_property_name = true;
                setting_property_value = false;

                match property_name.as_str() {
                    "CameraPosition" => {
                        let _ = self.camera_position.from_vs(&property_value);
                    },
                    "CameraZoom" => {
                        let _ = self.camera_zoom.from_vs(&property_value);
                    }
                    _ => {}
                }

                property_value.clear();
                property_name.clear();
            } else if b.to_string() == U_001B {
                // property value
                setting_property_name = false;
                setting_property_value = true;
            } else {
                // normal character
                if setting_property_name {
                    property_name.push(b);
                } else if setting_property_value {
                    property_value.push(b);
                }
            }
        }

        // it could have ended without detecting any special unicode charters
        println!("{}", property_name);
        match property_name.as_str() {
            "CameraPosition" => {
                let _ = self.camera_position.from_vs(&property_value);
            },
            "CameraZoom" => {
                let _ = self.camera_zoom.from_vs(&property_value);
            }
            _ => {}
        }

        Ok(&vs[vs_end..])
    }
}