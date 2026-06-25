use visual_source_rs::{U_001A, U_001B, VSObjectType, VisualSource, block::{Block, BlockInput, BlockInputType}, editor::Editor, field_types::{VSFieldType, brickcolor::VSBrickColor, number::VSNumber, object::VSObject, string::VSString, vector2::VSVector2}, hex::{self, Hex}};

fn main() {
    let visual_source = VisualSource {
        root_objects: vec![
            Box::new(Editor {
                camera_position: (0.0, 0.0).into(),
                camera_zoom: Hex(0.11).into()
            }),
            Box::new(Block {
                internal: "SetObjectProperty".into(),
                name: "Set1".into(),
                visual_position: (0.0, 0.0).into(),
                child_blocks: vec![],
                else_child_block: None,
                inputs: vec![
                    BlockInput {
                        name: "Object".into(),
                        visibility: BlockInputType::Implicit,
                        value: Box::<VSObject>::new("workspace.RedButton".into())
                    },
                    BlockInput {
                        name: "Property".into(),
                        visibility: BlockInputType::Implicit,
                        value: Box::new(VSString::from("BrickColor"))
                    },
                    BlockInput {
                        name: "Value".into(),
                        visibility: BlockInputType::Explicit,
                        value: Box::new(VSBrickColor::from(3))
                    }
                ],
                outputs: vec![]
            }),
        ]
    };

    println!("{}", visual_source.to_string().escape_debug())

    // TEST FIND-PART-ON-RAY BLOCK (it contains a input of type Table, in other words, only variables in it.)
}