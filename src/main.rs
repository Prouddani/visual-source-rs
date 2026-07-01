use std::collections::HashMap;

use visual_source_rs::{VisualSource, block::{Block, BlockInput, BlockInputVisibility}, editor::Editor, field_types::{object::VSObject, string::VSString}, vs_brickcolor};

fn main() {
    let visual_source = VisualSource {
        version: 4,
        editor: Editor {
            camera_position: (0.0, 0.0).into(),
            camera_zoom: 0.45.into(),
        },
        blocks: HashMap::from([
            ("Set Object Property1".to_string(), Block {
                internal: "SetObjectProperty".into(),
                name: "Set Object Property1".into(),
                visual_position: (0.0, 0.0).into(),
                child_blocks: vec![],
                else_child_block: None,
                inputs: vec![
                    BlockInput::new("Object", BlockInputVisibility::Implicit, VSObject::from_path("workspace.RedButton")),
                    BlockInput::new("Property", BlockInputVisibility::Implicit, VSString::from("BrickColor")),
                    BlockInput::new("Value", BlockInputVisibility::Explicit, vs_brickcolor!(40, 40, 40)),
                ],
                outputs: vec![]
            })
        ]),
        comments: vec![]
    };

    println!("{}", visual_source.into_json());
}