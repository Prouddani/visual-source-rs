use visual_source_rs::{U_001A, U_001B, VSObjectType, VisualSource, block::{Block, BlockInput, BlockInputVisibility, BlockOutput}, editor::Editor, field_types::{VSFieldType, bool::VSBool, brickcolor::VSBrickColor, number::VSNumber, object::VSObject, string::VSString, tuple::{self, VSTuple}, vector2::VSVector2, vector3::VSVector3}, hex::{self, Hex}, vs_bool, vs_num, vs_obj, vs_str, vs_tuple, vs_udim2, vs_vec2, vs_vec3};

fn main() {
    let visual_source = VisualSource {
        root_objects: vec![
            Box::new(Editor {
                camera_position: (0.0, 0.0).into(),
                camera_zoom: Hex(0.11).into()
            }),
            Box::new(Block {
                internal: "ExecuteFunction".into(),
                name: "Execute Function1".into(),
                visual_position: (0.0, 0.0).into(),
                child_blocks: vec![],
                else_child_block: None,
                inputs: vec![
                    BlockInput::new("Function", BlockInputVisibility::Implicit, vs_str!("h")),
                    BlockInput::new("Parameters", BlockInputVisibility::Implicit, vs_tuple!["hi"])
                ],
                outputs: vec![]
            }),
        ]
    };

    println!("{}", visual_source.to_string());

    println!(
        "{} {} {} {} {} {} {} {}",
        vs_str!("hi"),
        vs_num!(3),
        vs_obj!("game.Workspace.RedButton"),
        vs_bool!(true),
        vs_vec2!(2, 3),
        vs_vec3!(2, 3, 4),
        vs_tuple!["TUPLEPARAM1", "TUPLEPARAM2"],
        vs_udim2!(1, 2, 3, 4)
    );

    // let tuple_vs_properties = "TUPLEPARAM_Parameters_315NumberTUPLEPARAM_Parameters_11hello world!StringTUPLEPARAM_Parameters_2118BrickColor";
    // let mut tuple = VSTuple::new();
    // tuple.from_vs(tuple_vs_properties).unwrap()


    // ADD TUPLE. TO CHECK FOR ERRORS, USE FIRE-REMOTE-EVENT BLOCK
    // ADD CFRAME. TO CHECK FOR ERRORS, USE ADDITION BLOCK (contains a input that could be cframe)
    // ADD NUMBER RANGE. TO CHECK FOR ERRORS, USE SPLIT-NUMBER-RANGE BLOCK (contains an input that is a number range)
    // ADD UDIM2. TO CHECK FOR ERRORS, USE SPLIT-UDIM2 (contains an input that is a UDim2)
    // ADD COLOR3. TO CHECK FOR ERRORS, USE SPLIT-COLOR3 (contains an input that is UDim2)
    // ADD FUNCTION. TO CHECK FOR ERRORS, USE EXECUTE-FUNCTION (contains an input that is Function)
    // ADD TABLE. TO CHECK FOR ERRORS, USE GET-TABLE-VALUE BLOCK (contains an input that is Table)
}