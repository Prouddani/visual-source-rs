use std::collections::HashMap;

use visual_source_rs::{VisualSource, block::{Block, BlockInput}, editor::Editor, vs_brickcolor, vs_str};

fn main() -> Result<(), &'static str> {
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
                    BlockInput::new("Object", true, vs_str!("red_button")).of("SetObjectProperty")?, // kinda useless to put 'true' in uses_variable, because object always allow variables
                    BlockInput::new("Property", false, vs_str!("BrickColor")).of("SetObjectProperty")?,
                    BlockInput::new("Value", false, vs_brickcolor!(40, 40, 40)).of("SetObjectProperty")?,
                ],
                outputs: vec![]
            })
        ]),
        comments: vec![]
    };

    println!("{}", visual_source.to_json());

    Ok(())
}