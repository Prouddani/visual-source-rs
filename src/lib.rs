use std::{collections::HashMap, fmt::{Debug, Display}};

use serde_json::json;

use crate::{block::Block, comment::Comment, editor::Editor, field_types::VSFieldType};

pub mod field_types;
pub mod editor;
pub mod block;
pub mod comment;

mod hex;

pub mod macros;

const LATEST_VS_VERSION: u8 = 4;

const U_001A: &str = "\u{001A}";
const U_001B: &str = "\u{001B}";

trait VSObjectType {
    fn into_vs(&self) -> String;
    fn from_vs<'a>(&mut self, vs: &'a str) -> Result<&'a str, &'static str>;
    fn into_json(&self, visual_source: Option<&VisualSource>) -> serde_json::Value;
    fn from_json(&mut self, json: serde_json::Value) -> Result<(), &'static str>;
    fn get_type(&self) -> &'static str;
}

pub struct VisualSource {
    /// Depicts the version of the visual source
    pub version: u8,

    /// Editor instance. It represents the camera in the script editor
    pub editor: Editor,

    /// HashMap where the key is the block name, and the value is the actual block instance
    pub blocks: HashMap<String, Block>,

    /// Vector of comments (not the Comment block)
    pub comments: Vec<Comment>
}
impl VisualSource {
    pub fn into_json(&self) -> serde_json::Value {
        let editor = self.editor.into_json(Some(self));
        let blocks = self.blocks.iter().map(|(block_name, block)| (block_name, block.into_json(Some(self)))).collect::<HashMap<_, _>>();
        let comments = self.comments.iter().map(|comment| comment.into_json(Some(self))).collect::<Vec<serde_json::Value>>();

        json!({
            "Editor": editor,
            "Blocks": blocks,
            "Comments": comments
        })
    }

    pub fn from_json(mut json: serde_json::Value) -> Result<Self, &'static str> {
        let editor = Editor::from_json(json.get_mut("Editor").ok_or("'Editor' does not exist in Visual Source json")?.take())?;
        let blocks = json.get_mut("Blocks").ok_or("'Blocks' does not exist in Visual Source json")?.take()
                                                .as_array_mut().ok_or("'Blocks' in Visual Source json is not an array")?
                                                .iter_mut().map(|block| {
                                                    let b = Block::from_json(block.take()).unwrap();
                                                    (b.name.to_string(), b)
                                                })
                                                .collect::<HashMap<_, _>>();
        let comments = json.get_mut("Comments").ok_or("'Comments' does not exist in Visual Source json")?.take()
                                        .as_array_mut().ok_or("'Comments' in Visual Source json is not an array")?
                                        .iter_mut().map(|comment| Comment::from_json(comment.take()).unwrap())
                                        .collect::<Vec<_>>();

        Ok(Self {
            version: LATEST_VS_VERSION,
            editor,
            blocks,
            comments
        })
    }
}
impl Display for VisualSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vs_blocks = self.blocks.iter().map(|(_, block)| block.into_vs()).collect::<Vec<String>>();
        
        write!(f, "{}{}", self.editor.into_vs(), vs_blocks.join(""))
    }
}
impl Debug for VisualSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().escape_debug())
    }
}


impl Display for Box<dyn VSFieldType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs().escape_debug())
    }
}
impl Debug for Box<dyn VSFieldType> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs().escape_debug())
    }
}