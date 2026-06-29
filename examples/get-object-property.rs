use visual_source_rs::{VisualSource, block::{Block, BlockInput, BlockInputVisibility, BlockOutput, BlockOutputValueType}, editor::Editor, vs_obj, vs_str};

fn main() -> Result<(), &'static str> {
    let visual_source = VisualSource {
        version: 4, // There isn't much utility to this, just as an identifier.
        root_objects: vec![
            Editor::new(
                (0.0, 0.0), // Visual Position (where the camera is located)
                0.11        // Camera Zoom (how zoomed the camera is)
            ),
            Block::new(
                "GetObjectProperty",    // internal name (for example, GetObjectProperty, SolveEquation, KeyDown, etc)
                "Get Object Property1", // user-given name
                (0.0, 0.0),             // visual position
                vec!["Print1".into()],  // child block ids
                None,                   // else child block id (only Some<VSString> for If blocks)
                vec![
                    BlockInput::new("Object", BlockInputVisibility::Implicit, vs_obj!("game.Workspace.RedButton")),
                    BlockInput::new("Property", BlockInputVisibility::Implicit, vs_str!("BrickColor"))
                ],
                vec![
                    //         The output name          The output type (list of variable names, aka tuples, or the variable name)
                    //                  V                                 V
                    BlockOutput::new("Value", BlockOutputValueType::String("brickcolor".into()))?
                ]
            )
        ]
    };

    println!("{:?}", visual_source);

    Ok(())
}