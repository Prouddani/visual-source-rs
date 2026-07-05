use std::collections::HashMap;

use visual_source_rs::{VisualSource, block::{Block, BlockInput, BlockInputVisibility, BlockOutput, BlockOutputValueType}, editor::Editor, vs_brickcolor, vs_obj, vs_str, vs_tuple};

fn main() -> Result<(), &'static str> {
    let visual_source = VisualSource {
        version: 4,
        editor: Editor {
            camera_position: (0.0, 0.0).into(),
            camera_zoom: 0.45.into(),
        },
        blocks: HashMap::from([
            ("If".to_string(), Block {
                internal: "If".into(),
                name: "If is tikki phonk1".into(),
                visual_position: (0.0, 0.0).into(),
                child_blocks: vec![],
                else_child_block: None,
                inputs: vec![
                    BlockInput::new("Value 1", false, vs_str!("amongus")).of("If")?,
                    BlockInput::new("ComparisonType", false, vs_str!("newbite more like oldbite")).of("If")?,
                    BlockInput::new("Value 2", false, vs_str!("sussy")).of("If")?
                ],
                outputs: vec![]
            })
        ]),
        comments: vec![]
    };

    println!("{}", visual_source.to_json());

    Ok(())
}