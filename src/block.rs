use std::{collections::HashMap, fmt::Display, vec};

use serde_json::json;

use crate::{U_001A, U_001B, VSObjectType, VisualSource, field_types::{VSFieldType, new_field_from_vs_type, number::VSNumber, string::VSString, tuple::VSTuple, vector2::VSVector2}};

#[derive(Clone, Copy, Debug)]
/// Depicts how an input value type is stored in the RetroStudio block database. In this case, whether an input of a block already has its type determined
/// (implicit, because doesn't show up in Visual Source), or the user must determine its type (any), or if its a variable (string)
pub enum BlockInputVisibility {
    /// The value type of an input is already pre-determined inside the RetroStudio block database
    Implicit,
    
    /// The value type must be explicitly shown in Visual Source (any), since the Retro Studio block database doesn't cover it up
    Explicit,

    // The value is referenced through a variable name. In this case, the value type WILL be a string
    Variable
}
impl BlockInputVisibility {
    pub fn to_vs(&self) -> String {
        match self {
            Self::Implicit => "0",
            Self::Explicit => "1",
            Self::Variable => "2",
        }.to_string()
    }

    pub fn from_vs(vs: &str) -> Result<Self, &'static str> {
        match vs {
            "0" => Ok(Self::Implicit),
            "1" => Ok(Self::Explicit),
            "2" => Ok(Self::Variable),
            _ => Err("Invalid BlockInputVisibility")
        }
    }

    pub fn use_variable(&self) -> bool {
        match self {
            Self::Variable => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct BlockInput {
    pub name: VSString,
    pub visibility: BlockInputVisibility,
    pub value: Box<dyn VSFieldType>,
    tuple_of: Option<String>
}
impl BlockInput
{
    pub fn new<T>(name: impl Into<VSString>, uses_variable: bool, value: T) -> Self
    where
        T: VSFieldType + 'static,
    {
        let visibility = match uses_variable {
            true => BlockInputVisibility::Variable,
            false => BlockInputVisibility::Explicit
        };

        Self {
            name: name.into(),
            visibility,
            value: Box::new(value),
            tuple_of: None
        }
    }

    /// Mixes `BlockInput::new` and `BlockInput::of_tuple` together into 1 method.
    pub fn as_tuple_param<T>(tuple: impl Into<String>, i: u32, uses_variable: bool, value: T) -> Self
    where
        T: VSFieldType + 'static,
    {
        let tuple = tuple.into();

        let name = format!("TUPLEPARAM_{}_{}", tuple, i).into();
        let visibility = match uses_variable {
            true => BlockInputVisibility::Variable,
            false => BlockInputVisibility::Explicit
        };
        let value = Box::new(value);
        let tuple_of = Some(tuple);

        Self {
            name,
            visibility,
            value,
            tuple_of
        }
    }

    /// Sets an input as a tuple parameter.
    /// 
    /// Changes its visual source name to TUPLEPARAM_X_Y, where X and Y are the tuple parameter info, and makes it aware of its tuple parent
    pub fn of_tuple(mut self, tuple: impl Into<String>, i: u32) -> Self {
        let tuple = tuple.into();

        self.name = format!("TUPLEPARAM_{}_{}", tuple, i).into();
        self.tuple_of = Some(tuple);

        self
    }

    /// Method of BlockInput that, by giving the name of the block containing the input, returns itself,
    /// but now with a visibility field (whether the input is explicit, implicit or variable in Visual Source text)
    /// 
    /// If you are using variable, this method will return itself immediately.
    /// 
    /// Do not call this method if the input is a tuple parameter.
    pub fn of(mut self, owner: impl Into<String>) -> Result<Self, &'static str> {
        if let BlockInputVisibility::Variable = self.visibility {
            return Ok(self);
        }

        let vs_blocks: serde_json::Value = serde_json::from_slice(include_bytes!("vs_blocks.json")).or(Err("Error getting vs_blocks.json"))?;
        let block = vs_blocks.get(owner.into()).ok_or("Owner block not found in vs_blocks.json")?;
        let inputs = block.get("Inputs").ok_or("Inputs is not a member of a block blueprint in vs_blocks.json")?.as_array().ok_or("Inputs in vs_blocks.json is not an array")?;
        
        let mut this = None;
        for input in inputs {
            if input.get("name").ok_or("Name does not exist in Inputs from vs_blocks.json")?.as_str().ok_or("Name is not a string in vs_blocks.json")? == self.name.0 {
                this = Some(input);
                break;
            }
        }
        if this.is_none() { return Err("Unable to find input of owner in vs_blocks.json"); }

        let value_type = this.unwrap().get("value_type").ok_or("'value_type' does not exist in Input from vs_blocks.json")?.as_str().ok_or("'value_type' from input in vs_blocks.json is not a string")?;
        let input_visibility = match value_type {
            "Any" => BlockInputVisibility::Explicit,
            "Function" | "Table" | "CFrame" | "TextChannel" | "TextSource" | "TextChatMessage" | "Channel" => BlockInputVisibility::Variable,
            _ => BlockInputVisibility::Implicit
        };

        self.visibility = input_visibility;

        Ok(self)
    }

    pub fn to_vs(&self) -> String {
        format!(
            "{}{U_001B}{}{U_001B}{}{}{}",
            self.name.to_vs(),
            self.visibility.to_vs(),
            self.value.to_vs(),
            if let BlockInputVisibility::Explicit = self.visibility { U_001B } else { "" },
            if let BlockInputVisibility::Explicit = self.visibility { self.value.get_type() } else { "" },
        )
    }

    pub fn to_json(&self, inputs: &Vec<BlockInput>) -> serde_json::Value {
        let variable = match &self.visibility {
            BlockInputVisibility::Variable => self.value.to_vs(),
            _ => String::new()
        };
        let use_variable = self.visibility.use_variable();
        let mut value = self.value.to_json();
        let value_type = self.value.get_type();

        if value_type == "Tuple" {
            let tuple_parameters = VSTuple::get_from_input_vec(self, inputs);
            value = json!(tuple_parameters.iter().map(|param| param.name.to_string()).collect::<Vec<String>>());
        }

        json!({
            "Variable": variable,
            "UseVariable": use_variable,
            "Value": value,
            "ValueType": value_type,
        })
    }

    pub fn from_json(&mut self, block_id: String, input_name: String, mut json: serde_json::Value) -> Result<(), &'static str> {
        let variable = json.get_mut("Variable").ok_or("Could not get Variable in block input json")?.take();
        let value = json.get_mut("Value").ok_or("Could not get Value in block input json")?.take();
        let use_variable = json.get_mut("UseVariable").ok_or("Could not get UseVariable in block input json")?.take();

        let mut vs_value: Box<dyn VSFieldType> = match use_variable {
            serde_json::Value::Bool(true) => {
                // uses variable
                self.visibility = BlockInputVisibility::Variable;

                let mut vs_value = VSString::new();
                vs_value.from_vs(variable.as_str().ok_or("Variable in block input json is not a string")?)?;

                Box::new(vs_value)
            },
            serde_json::Value::Bool(false) => {
                // uses value
                let mut blocks_database: serde_json::Value = serde_json::from_slice(include_bytes!("vs_blocks.json")).or(Err("Could not conver vs_blocks.json into a serde_json::Value object"))?;
                let block_blueprint = blocks_database.get_mut(block_id).ok_or("Unable to get a block from type in vs_blocks.json")?;
                let inputs = block_blueprint.get_mut("Inputs").ok_or("Error getting inputs of block blueprint, from vs_blocks.json")?;

                let mut input = None;
                for i in inputs.as_array_mut().ok_or("Inputs in block blueprint must always be an array. However, one that isn't was found")? {
                    let name = i.get("name").ok_or("Input does not contain 'name' key")?;
                    if input_name == name.as_str().ok_or("Input name is not a string")? {
                        input = Some(i);
                    }
                }
                if input.is_none() { return Err("No input matching given input_name (function argument) from block in vs_blocks.json"); }

                let value_type = input.unwrap().get("value_type").ok_or("Input does not contain 'value_type' key")?.as_str().ok_or("Input value type is not a string, from vs_blocks.json")?;
                match value_type {
                    "Any" => {
                        self.visibility = BlockInputVisibility::Explicit;
                    }
                    _ => {
                        self.visibility = BlockInputVisibility::Implicit;
                    }
                }

                new_field_from_vs_type(value_type).ok_or("Invalid value_type in vs_blocks.json")?
            }
            _ => return Err("UseVariable in block input is not a bool")
        };
        vs_value.from_json(value)?;

        self.value = vs_value;

        Ok(())
    }

    pub(crate) fn is_of_tuple(&self, tuple: impl Into<String>) -> bool {
        self.tuple_of.is_some() && self.tuple_of.as_ref().unwrap() == &tuple.into()
    }
}
impl<I, T> From<(I, BlockInputVisibility, T)> for BlockInput
where
    I: Into<VSString>,
    T: VSFieldType + 'static
{
    fn from(value: (I, BlockInputVisibility, T)) -> Self {
        Self {
            name: value.0.into(),
            visibility: value.1,
            value: Box::new(value.2),
            tuple_of: None
        }
    }
}
impl Display for BlockInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_vs())
    }
}


#[derive(Clone, Debug)]
pub enum BlockOutputValueType {
    Tuple(VSNumber),
    String(VSString)
}
impl BlockOutputValueType {
    pub fn to_vs(&self) -> String {
        match self {
            Self::Tuple(number) => number.to_vs(),
            Self::String(string) => string.to_vs()
        }
    }

    pub fn is_tuple(&self) -> bool {
        match self {
            Self::Tuple(_) => true,
            Self::String(_) => false
        }
    }
}

#[derive(Debug)]
pub struct BlockOutput {
    pub name: VSString,
    pub value: BlockOutputValueType,
}
impl BlockOutput {
    pub fn new(
        name: impl Into<VSString>,
        value: BlockOutputValueType, // we use Vec<T>, because of tuple outputs
    ) -> Result<Self, &'static str> {
        let name = name.into();

        Ok(Self {
            name: name.into(),
            value,
        })
    }

    pub fn of_tuple(parent_tuple: impl Into<VSString>, i: u32, variable_name: impl Into<VSString>) -> Self {
        let name = format!("TUPLEPARAM_{}_{}", parent_tuple.into().to_string(), i);
        let value = BlockOutputValueType::String(variable_name.into());

        Self {
            name: name.into(),
            value
        }
    }

    pub fn is_tuple(value: serde_json::Value) -> bool {
        value.is_array()
    }

    /// Returns the tuple parameters' name. However, if the block output
    pub fn get_tuple_param_names(&self) -> Option<Vec<String>> {
        let raw_name = self.name.to_string();

        match self.value {
            BlockOutputValueType::Tuple(num_params) => {
                let num_params: isize = num_params.into();
                
                let mut param_names = vec![];
                for i in 1..=num_params {
                    param_names.push(format!("TUPLEPARAM_{raw_name}_{i}"))
                }

                Some(param_names)
            },
            BlockOutputValueType::String(_) => return None
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match &self.value {
            BlockOutputValueType::String(string) => string.to_json(),
            BlockOutputValueType::Tuple(number) => json!(number.0.0.to_string())
        }
    }

    pub fn from_json(&mut self, block_id: String, output_name: String, json: serde_json::Value) -> Result<(), &'static str> {
        let variable_name = json.as_str().ok_or("Output variable name is not a string")?;
        
        let block_blueprints: serde_json::Value = serde_json::from_slice(include_bytes!("vs_blocks.json")).or(Err("Unexpected error convering .json file in serde_json::Value"))?;
        let block_blueprint = block_blueprints.get(block_id).ok_or("BlockId is not an existant object of vs_blocks.json")?;
        let outputs = block_blueprint.get("Outputs").ok_or("Outputs doesn't exist in a block blueprint in vs_blocks.json")?.as_array().ok_or("Outputs in vs_blocks.json is not an array")?;
        let mut output_type = None;
        for output in outputs {
            if output.is_array() {
                // tuple
                if let serde_json::Value::String(string) = output.get(0).ok_or("Tuple output in vs_blocks.json is empty (no items in array)")? {
                    if string == &output_name {
                        output_type = Some(BlockOutputValueType::Tuple(VSNumber::from(variable_name.parse::<u32>().or(Err("Variable name in tuple output is not a number. It must represent the number of tuple parameter it has"))?)));
                        break;
                    }
                } else {
                    return Err("Output is not a string in vs_blocks.json. Note that, in this case, the output is a tuple, the error is happening inside of the array");
                }
            } else {
                // variable name
                if let serde_json::Value::String(string) = output {
                    if string == &output_name {
                        output_type = Some(BlockOutputValueType::String(variable_name.into()))
                    }
                } else {
                    return Err("Output is not a string in vs_blocks.json.");
                }
            }
        }
        self.value = output_type.ok_or("Could not find type of output (from_json)")?;

        Ok(())
    }
}
impl Display for BlockOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{U_001B}{}", self.name.to_vs(), self.value.to_vs())
    }
}

#[derive(Debug)]
pub struct Block {
    pub internal: VSString,
    pub name: VSString,
    pub visual_position: VSVector2,
    pub child_blocks: Vec<VSString>,
    pub else_child_block: Option<VSString>,
    pub inputs: Vec<BlockInput>,
    pub outputs: Vec<BlockOutput>,
    pub parent_blocks: Vec<String>
}
impl Block {
    /// Creates a new Block instance
    pub fn new() -> Self {
        Self {
            internal: "".into(),
            name: "".into(),
            visual_position: (0, 0).into(),
            child_blocks: vec![],
            else_child_block: None,
            inputs: vec![],
            outputs: vec![],
            parent_blocks: vec![]
        }
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut block = Self::new();
        block.from_json(json)?;

        Ok(block)
    }
}
impl VSObjectType for Block {
    /// Converts itself into a json. If `visual_source` parameter is None, there will be no `parent_blocks` in the returning json,
    /// since there is no way to check it with it
    fn to_json(&self) -> serde_json::Value {
        let internal = &self.internal.0;
        let visual_position = self.visual_position.to_json();

        let child_blocks = self.child_blocks.iter().map(VSFieldType::to_json).collect::<Vec<_>>();
        let else_child_block = match &self.else_child_block {
            Some(else_child_block) => else_child_block.to_json(),
            None => serde_json::Value::Null
        };

        let mut tuple_relation = HashMap::new();
        let mut inputs = HashMap::new();
        for input in &self.inputs {
            if input.value.get_type() == "Tuple" {
                for tuple in input.value.to_vs().split(",") {
                    tuple_relation.insert(input.name.to_vs(), tuple.to_string());
                }
            }

            inputs.insert(input.name.to_vs(), input.to_json(&self.inputs));
        }
        for (_, input) in &mut inputs {
            if let serde_json::Value::String(name) = &input["Name"] {
                let relationship = tuple_relation.iter_mut().find(|(k, _)| {
                    *k == name
                });

                if relationship.is_some() {
                    let (child, parent) = relationship.unwrap();
                    input.as_object_mut().unwrap().insert("TupleOf".to_string(), serde_json::Value::String(std::mem::take(parent)));
                }
            }
        }
        let outputs = self.outputs.iter().map(|output| {
            let is_tuple = output.value.is_tuple();
            let out_name = if is_tuple {
                format!("TUPLE_{}", output.name.to_string())
            } else {
                output.name.to_string()
            };

            (json!(out_name), output.to_json())
        }).collect::<HashMap<_, _>>();

        let parent_blocks = &self.parent_blocks;

        json!({
            "Type": internal,
            "VisualPosition": visual_position,
            "ChildBlocks": child_blocks,
            "ElseChildBlock": else_child_block,
            "Inputs": inputs,
            "Outputs": outputs,

            "ParentBlocks": parent_blocks
        })
    }
    
    fn from_json(&mut self, mut json: serde_json::Value) -> Result<(), &'static str> {
        let internal: VSString = json.get("Type").ok_or("Could not get 'Type' key of block json")?.as_str().ok_or("Type of block json is not a string")?.into();
        let visual_position = VSVector2::from_json(json.get_mut("VisualPosition").ok_or("Could not get 'VisualPosition' key of block json")?.take())?;
        let child_blocks = json.get("ChildBlocks").ok_or("Could not get 'ChildBlocks' key of block json")?.as_array().ok_or("Error converting ChildBlocks into array.")?.iter().map(|value| VSString::from(value.as_str().expect("Child blocks property of Block json must be string"))).collect::<Vec<_>>();
        let else_child_block = match json.get_mut("ElseChildBlock").ok_or("Error getting ElseChildBlock property for block json")? {
            serde_json::Value::String(string) => Some(std::mem::take(string).into()),
            serde_json::Value::Null => None,
            _ => return Err("ElseChild block")
        };
        let inputs = json.get_mut("Inputs").ok_or("Error getting Inputs in block json")?.as_object_mut().ok_or("Inputs in block json isn't an object")?.iter_mut().map(|(input_name, value)| {
            let uses_variable = value.get("UseVariable").expect("UseVariable is not a valid member if input of block json").as_bool().expect("UseVariable is not a bool, in block input json");
            
            let mut input = BlockInput::new("", uses_variable, VSString::new()).of(internal.to_vs()).expect("Error setting the owner of string");
            input.from_json(internal.to_vs(), input_name.to_string(), value.take()).expect("Failed to create an input from json");

            input
        }).collect::<Vec<BlockInput>>();
        let outputs = json.get_mut("Outputs").ok_or("Error getting Outputs in block json")?.as_object_mut().ok_or("Outputs in block json is not an object")?.iter_mut().map(|(output_name, value)| {
            let mut output = BlockOutput::new("", BlockOutputValueType::String(VSString::new())).expect("Failed to create BlockOutput");
            output.from_json(internal.to_vs(), output_name.to_string(), value.take()).expect("Failed to convert json into BlockOutput");

            output
        }).collect::<Vec<BlockOutput>>();

        self.internal = internal;
        self.visual_position = visual_position;
        self.child_blocks = child_blocks;
        self.else_child_block = else_child_block;
        self.inputs = inputs;
        self.outputs = outputs;

        Ok(())
    }

    fn get_type(&self) -> &'static str {
        "Block"
    }
}