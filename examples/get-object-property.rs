use std::collections::HashMap;

use visual_source_rs::{VisualSource, block::{Block, BlockInput, BlockOutput, BlockOutputValueType}, editor::Editor, vs_obj, vs_str};

fn main() -> Result<(), &'static str> {
    let visual_source = VisualSource {
        version: 4, // There isn't much utility to this, just as an identifier.
        editor: Editor {
                camera_position: (0.0, 0.0).into(),
                camera_zoom: 0.11.into()
            },
        blocks: HashMap::from([
            ("Get Object Property1".to_string(), Block {
                internal: "GetObjectProperty".into(),    // internal name (for example, GetObjectProperty, SolveEquation, KeyDown, etc)
                name: "Get Object Property1".into(),     // user-given name
                visual_position: (0.0, 0.0).into(),      // visual position
                child_blocks: vec!["Print1".into()],     // child block ids
                else_child_block: None,                  // else child block id (only Some<VSString> for If blocks)
                inputs: vec![
                    //                                                                     method that makes the input aware
                    //        the input name   uses variable    the value of the input   of its owner block (GetObjectProperty)
                    //                 v          v                  v                                 v
                    BlockInput::new("Object",   false, vs_obj!("game.Workspace.RedButton")).of("GetObjectProperty")?,
                    BlockInput::new("Property", false, vs_str!("BrickColor"))              .of("GetObjectProperty")?
                ],
                outputs: vec![
                    //                                   The output type (list of variable names, aka tuples, or the variable name)
                    //           The output name             in this case, it is the variable name, because it is only a string
                    //                  V                                 V
                    BlockOutput::new("Value", BlockOutputValueType::String("brickcolor".into()))?
                ]
            })
        ]),
        comments: vec![]
    };

    println!("{:?}", visual_source);

    Ok(())
}