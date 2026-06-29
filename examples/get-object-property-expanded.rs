use visual_source_rs::{VisualSource, block::{Block, BlockInput, BlockInputVisibility, BlockOutput, BlockOutputValueType}, editor::Editor, vs_obj, vs_str};

fn main() -> Result<(), &'static str> {
    let visual_source = VisualSource {
        version: 4, // There isn't much utility to this, just as an identifier.
        root_objects: vec![
            Box::new(Editor {
                camera_position: (0.0, 0.0).into(),
                camera_zoom: 0.11.into()
            }),
            Box::new(Block {
                internal: "GetObjectProperty".into(),    // internal name (for example, GetObjectProperty, SolveEquation, KeyDown, etc)
                name: "Get Object Property1".into(),     // user-given name
                visual_position: (0.0, 0.0).into(),      // visual position
                child_blocks: vec!["Print1".into()],     // child block ids
                else_child_block: None,                  // else child block id (only Some<VSString> for If blocks)
                inputs: vec![
                    //                              The visibility of the value type
                    //        The input name      (uses variable, pre-determined or any)        The actual value
                    //                V                        V                                      V
                    BlockInput::new("Object", BlockInputVisibility::Implicit, vs_obj!("game.Workspace.RedButton")),
                    BlockInput::new("Property", BlockInputVisibility::Implicit, vs_str!("BrickColor"))
                ],
                outputs: vec![
                    //         The output name          The output type (list of variable names, aka tuples, or the variable name)
                    //                  V                                 V
                    BlockOutput::new("Value", BlockOutputValueType::String("brickcolor".into()))?
                ]
            })
        ]
    };

    println!("{:?}", visual_source);

    Ok(())
}