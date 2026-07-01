use core::num;
use std::{collections::HashMap, fmt::Display, vec};

use serde_json::json;

use crate::{U_001A, U_001B, VSObjectType, VisualSource, field_types::{VSFieldType, new_field_from_vs_type, number::VSNumber, string::VSString, vector2::VSVector2}};

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
    pub fn into_vs(&self) -> String {
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
}
impl BlockInput
{
    pub fn new<T>(name: impl Into<VSString>, visibility: BlockInputVisibility, value: T) -> Self
    where
        T: VSFieldType + 'static,
    {
        Self {
            name: name.into(),
            visibility,
            value: Box::new(value),
        }
    }

    pub fn into_vs(&self) -> String {
        format!(
            "{}{U_001B}{}{U_001B}{}{}{}",
            self.name.into_vs(),
            self.visibility.into_vs(),
            self.value.into_vs(),
            if let BlockInputVisibility::Explicit = self.visibility { U_001B } else { "" },
            if let BlockInputVisibility::Explicit = self.visibility { self.value.get_type() } else { "" },
        )
    }

    pub fn into_json(&self) -> serde_json::Value {
        let variable = match &self.visibility {
            BlockInputVisibility::Variable => self.value.into_vs(),
            _ => String::new()
        };
        let use_variable = self.visibility.use_variable();
        let value = self.value.into_json();
        let value_type = self.value.get_type();

        json!({
            "Variable": variable,
            "UseVariable": use_variable,
            "Value": value,
            "ValueType": value_type,
        })
    }

    pub fn from_json(&mut self, block_id: String, input_name: String, json: serde_json::Value) -> Result<(), &'static str> {
        let variable = json.get("Variable").ok_or("Could not get Variable in block input json")?;
        let value = json.get("Value").ok_or("Could not get Value in block input json")?;
        let use_variable = json.get("UseVariable").ok_or("Could not get UseVariable in block input json")?;

        let vs_value: Box<dyn VSFieldType> = match use_variable {
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
            },
            _ => return Err("UseVariable in block input is not a bool")
        };

        self.value = vs_value;

        Ok(())
    }
}
impl<I, T> From<(I, BlockInputVisibility, T)> for BlockInput
where
    I: Into<VSString>,
    T: VSFieldType + 'static
{
    fn from(value: (I, BlockInputVisibility, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}
impl Display for BlockInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_vs())
    }
}


#[derive(Clone, Debug)]
pub enum BlockOutputValueType {
    Tuple(VSNumber),
    String(VSString)
}
impl BlockOutputValueType {
    fn into_vs(&self) -> String {
        match self {
            Self::Tuple(number) => number.into_vs(),
            Self::String(string) => string.into_vs()
        }
    }

    fn is_tuple(&self) -> bool {
        match self {
            Self::Tuple(_) => true,
            Self::String(_) => false
        }
    }

    fn is_tuple_from_name(name: &str) -> bool {
        name.contains("TUPLE_")
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

        match value {
            BlockOutputValueType::String(_) => {},
            BlockOutputValueType::Tuple(_) if name.contains("TUPLE_") => {},
            _ => return Err("The value doesn't match the output type, given by the name (tuple or not)")
        };

        Ok(Self {
            name: name.into(),
            value,
        })
    }

    pub fn is_tuple(value: serde_json::Value) -> bool {
        value.is_array()
    }

    /// Returns the tuple parameters' name. However, if the block output
    pub fn get_tuple_param_names(&self) -> Option<Vec<String>> {
        let raw_name = self.name.to_string();
        let (_, root_name) = raw_name.split_once("TUPLE_").unwrap_or(("", raw_name.as_str()));

        match self.value {
            BlockOutputValueType::Tuple(num_params) => {
                let num_params: isize = num_params.into();
                
                let mut param_names = vec![];
                for i in 1..=num_params {
                    param_names.push(format!("TUPLEPARAM_{root_name}_{i}"))
                }

                Some(param_names)
            },
            BlockOutputValueType::String(_) => return None
        }
    }

    pub fn into_json(&self) -> serde_json::Value {
        match &self.value {
            BlockOutputValueType::String(string) => string.into_json(),
            BlockOutputValueType::Tuple(number) => json!(number.0.0)
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
        
        self.value = output_type.ok_or("")?;

        Ok(())
    }
}
impl Display for BlockOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{U_001B}{}", self.name.into_vs(), self.value.into_vs())
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
    pub outputs: Vec<BlockOutput>
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
            outputs: vec![]
        }
    }

    pub fn get_parent_blocks(&self, visual_source: &VisualSource) -> Vec<serde_json::Value> {
        let mut parent_blocks = vec![];
        for (_, block) in &visual_source.blocks {
            if block.child_blocks.contains(&self.name) {
                parent_blocks.push(block.name.into_json())
            }
        }
        parent_blocks
    }

    pub fn from_json(json: serde_json::Value) -> Result<Self, &'static str> {
        let mut block = Self::new();
        block.from_json(json)?;

        Ok(block)
    }
}
impl VSObjectType for Block {
    fn into_vs(&self) -> String {
        format!(
            "{U_001A}{U_001A}Block{U_001A}Type{U_001B}{}{U_001A}Name{U_001B}{}{U_001A}VisualPosition{U_001B}{}{U_001A}ChildBlocks{}{}{U_001A}ElseChildBlock{U_001B}{}{U_001A}Inputs{}{}{U_001A}Outputs{}{}",
            self.internal.into_vs(), self.name.into_vs(), // internal_type and name
            self.visual_position.into_vs(), // visual position
            if self.child_blocks.len() <= 0 {""} else {U_001B}, // if there are no child blocks, there should be no u+001B characters
            self.child_blocks.iter().map(VSFieldType::into_vs).collect::<Vec<String>>().join(U_001B),
            self.else_child_block.clone().unwrap_or("nil".into()), // else child blocks
            if self.inputs.len() <= 0 {""} else {U_001B}, // if there are no child blocks, there should be no u+001B characters
            self.inputs.iter().map(BlockInput::into_vs).collect::<Vec<String>>().join(U_001B), // inputs
            if self.outputs.len() <= 0 {""} else {U_001B},
            self.outputs.iter().map(BlockOutput::to_string).collect::<Vec<String>>().join(U_001B), // outputs
        )
    }

    fn from_vs<'a>(&mut self, vs: &'a str) -> Result<&'a str, &'static str> {
        //{1a}{1a}Block{1a}Type{1b}SetObjectProperty{1a}Name{1b}Set1{1a}VisualPosition{1b}0,0{1b}ChildBlocks{1a}ElseChildBlock{1b}nil{1a}Inputs{1a}Outputs
        let mut vs_end = 0;
        
        let mut is_block = false;
        let mut u1a_count = 0;

        let mut set_property_name = false;
        let mut property_name = String::new();
        let mut set_property_value = false;
        let mut property_values: Vec<String> = vec![];

        for (i, c) in vs.chars().enumerate() {
            vs_end = i;

            if u1a_count >= 2 {
                if is_block == true {
                    break; // entering another Object
                }

                is_block = true;
                u1a_count = 0;
            }

            match c.to_string().as_str() {
                U_001A => {
                    u1a_count += 1;
                },
                _ => {}
            }

            // good stuff (or bad, if you lose sanity while coding, which I do and did while writting this code, unfortunately)
            if c.to_string() == U_001A {
                set_property_name = true;
                set_property_value = false;

                match property_name.as_str() {
                    "Type" => {
                        let result = self.internal.from_vs(&property_values[0][..]);
                        if result.is_err() {
                            return Err("Error when defining property valued for 'Type'.");
                        }
                    },
                    "Name" => {
                        let result = self.name.from_vs(&property_values[0][..]);
                        if result.is_err() {
                            return Err("Error when defining property valued for 'Name'.");
                        }
                    },
                    "VisualPosition" => {
                        let result = self.visual_position.from_vs(&property_values[0][..]);
                        if result.is_err() {
                            return Err("Error when defining property valued for 'VisualPosition'.");
                        }
                    },
                    "ChildBlocks" => {
                        self.child_blocks.clear();
                        for pv in &property_values {
                            let mut temp: VSString = "".into();
                            let result = temp.from_vs(&pv[..]);
                            if result.is_err() {
                                return Err("Error when defining property valued for 'Type'.");
                            }

                            self.child_blocks.push(temp);
                        }
                    },
                    "ElseChildBlock" => {
                        let mut c: VSString = "hi".into();
                        
                        let result = c.from_vs(&property_values[0][..]);
                        if result.is_err() {
                            self.else_child_block = None;
                            return Err("Error when defining property valued for 'Type'.");
                        }

                        self.else_child_block = Some(c);
                    },
                    "Inputs" => {
                        self.inputs.clear();

                        use std::mem;

                        let mut n_input = 0;
                        let mut vs_name = None;
                        let mut vs_visibility = None;

                        for i in 0..property_values.len() {
                            if vs_name.is_none() {
                                let in_name = {
                                    let in_name = &mut property_values[i];
                                    mem::take(in_name)
                                };

                                let mut temp_vs_name = VSString::new();
                                temp_vs_name.from_vs(in_name.as_str()).or(Err("Unable to identify Input name"))?;

                                vs_name = Some(temp_vs_name);
                            } else if vs_visibility.is_none() {
                                let in_visibility = {
                                    let in_visibility = &mut property_values[i];
                                    mem::take(in_visibility)
                                };
                                
                                let temp_vs_visibility = BlockInputVisibility::from_vs(&in_visibility)
                                    .or(Err("Unable to identify Input Value visibility (Uses Variable)"))?;
                            
                                vs_visibility = Some(temp_vs_visibility)
                            } else {
                                let in_value = {
                                    let in_value = &mut property_values[i];
                                    mem::take(in_value)
                                };

                                let visibility = vs_visibility.as_ref().unwrap();
                                let temp_vs_value = match visibility {
                                    BlockInputVisibility::Explicit => {
                                        let in_value_type = &mut property_values[i + 1];
                                        let mut vs_value = new_field_from_vs_type(in_value_type.as_str())
                                            .ok_or("Input value does not match with explicit type")?;

                                        vs_value.from_vs(&in_value).or(Err("Error in translating VisualSource into block input value.
                                        Input type or visibility could be invalid or incorrect"))?;

                                        vs_value
                                    },
                                    BlockInputVisibility::Implicit => {
                                        // TODO:
                                        /*
                                         * Get Block Input Types in /vs_blocks.json
                                         */

                                        //todo!("must get Block Input Types in /vs_blocks.json");

                                        let json_data = include_bytes!("vs_blocks.json");
                                        let parsed_json = serde_json::from_slice::<'_, serde_json::Value>(json_data).or(Err("Unable to parse vs_blocks.json"))?;
                                        if let Some(block) = parsed_json.get(self.internal.into_vs()) {
                                            // block
                                            if let Some(entry) = block["Inputs"].get(n_input) {
                                                // input
                                                let mut value_type = entry.get("value_type")
                                                                        .ok_or("Unable to find input type")?.as_str()
                                                                        .ok_or("Unable to parse input type into string")?;
                                                
                                                match value_type {
                                                    "EventConnection" | "Table" | "CFrame" | "Function" => {
                                                        vs_visibility = Some(BlockInputVisibility::Variable);
                                                        value_type = "String"
                                                    },
                                                    _ => {}
                                                }

                                                new_field_from_vs_type(value_type).ok_or("Unable to create a new value for input of input type")?
                                            } else {
                                                return Err("Input out of bounds. Make sure evert VisualSource block has the correct number of inputs");
                                            }
                                        } else {
                                            return Err(Box::leak(format!("Blueprint of {} hasn't been found in vs_blocks.json", self.internal).into_boxed_str()));
                                        }
                                    },
                                    BlockInputVisibility::Variable => {
                                        let mut vs_value = VSString::new();
                                        vs_value.from_vs(&in_value).or(Err("Unable to find variable name in input"))?;

                                        Box::new(vs_value)
                                    }
                                };

                                self.inputs.push(BlockInput {
                                    name: vs_name.take().unwrap(),
                                    visibility: vs_visibility.take().unwrap(),
                                    value: temp_vs_value
                                })
                            }

                            n_input += 1;
                        }
                    },
                    "Outputs" => {
                        use std::mem;
                        
                        for i in 0..property_values.len()/2 {
                            let in_name = {
                                let in_name = &mut property_values[i * 3 + 0];
                                mem::take(in_name)
                            };

                            let mut vs_name = VSString::new();
                            vs_name.from_vs(in_name.as_str())?;

                            let blocks: serde_json::Value = serde_json::from_slice(include_bytes!("vs_blocks.json")).or(Err(""))?;
                            let block_blueprint = blocks.get(self.internal.into_vs()).ok_or("Block Internal Name does not exist in src/vs_blocks.json")?;
                            let output = block_blueprint.get("Outputs").ok_or("Block in src/vs_blocks.json does not have Outputs key")?
                                                    .get(in_name).ok_or("Block does not have output in src/vs_blocks.json")?;
                            let is_output_tuple = output.is_array(); // the outputs that have their name inside an array are tuples

                            let in_value = {
                                let in_value = &mut property_values[i * 3 + 1];
                                mem::take(in_value)
                            };

                            let vs_value = match is_output_tuple {
                                true => {
                                    let mut vs_value = VSNumber::new();
                                    vs_value.from_vs(in_value.as_str())?;

                                    BlockOutputValueType::Tuple(vs_value)
                                }
                                false => {
                                    let mut vs_value = VSString::new();
                                    vs_value.from_vs(in_value.as_str())?;

                                    BlockOutputValueType::String(vs_value)
                                }
                            };

                            self.outputs.push(BlockOutput {
                                name: vs_name,
                                value: vs_value
                            });
                        }
                    },
                    _ => {}
                }
                
                property_name.clear();
                property_values.clear();
            } else if c.to_string() == U_001B {
                set_property_name = false;
                set_property_value = true;

                property_values.push(String::new());
            } else {
                // normal characters
                match (set_property_name, set_property_value) {
                    (true, false) => {
                        // set property name
                        property_name.push(c);
                    },
                    (false, true) => {
                        // set property value
                        let last_pv = property_values.last_mut().unwrap();
                        last_pv.push(c)
                    },
                    _ => {
                        continue;
                    }
                };
            }
        }

        Ok(&vs[vs_end..])
    }

    /// Converts self into a json. If `visual_source` parameter is None, there will be no `parent_blocks` in the returning json
    fn into_json(&self, visual_source: Option<&VisualSource>) -> serde_json::Value {
        let internal = &self.internal.0;
        let visual_position = self.visual_position.into_json();

        let child_blocks = self.child_blocks.iter().map(VSFieldType::into_json).collect::<Vec<_>>();
        let else_child_block = match &self.else_child_block {
            Some(else_child_block) => else_child_block.into_json(),
            None => serde_json::Value::Null
        };

        let mut tuple_relation = HashMap::new();
        let mut inputs = HashMap::new();
        for input in &self.inputs {
            if input.value.get_type() == "Tuple" {
                for tuple in input.value.into_vs().split(",") {
                    tuple_relation.insert(input.name.into_vs(), tuple.to_string());
                }
            }

            inputs.insert(input.name.into_vs(), input.into_json());
        }
        for (input_name, input) in &mut inputs {
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
        let outputs = self.outputs.iter().map(|output| output.into_json()).collect::<Vec<_>>();

        let parent_blocks = match visual_source {
            Some(visual_source) => json!(self.get_parent_blocks(visual_source)),
            None => serde_json::Value::Null
        };

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
            let mut input = BlockInput::new("", BlockInputVisibility::Explicit, VSString::new());
            input.from_json(internal.into_vs(), input_name.to_string(), value.take()).expect("Failed to create an input from json");

            input
        }).collect::<Vec<BlockInput>>();
        let outputs = json.get_mut("Outputs").ok_or("Error getting Outputs in block json")?.as_object_mut().ok_or("Outputs in block json is not an object")?.iter_mut().map(|(output_name, value)| {
            let mut output = BlockOutput::new("", BlockOutputValueType::String(VSString::new())).expect("Failed to create BlockOutput");
            output.from_json(internal.into_vs(), output_name.to_string(), value.take()).expect("Failed to convert json into BlockOutput");

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