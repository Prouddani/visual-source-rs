use std::{fmt::Display, process::Output};

use serde_json::{Value, value};

use crate::{U_001A, U_001B, VSObjectType, field_types::{self, VSFieldType, bool::VSBool, new_field_from_vs_type, number::VSNumber, string::VSString, vector2::VSVector2}, hex::Hex};

#[derive(Clone, Copy, Debug)]
pub enum BlockInputVisibility {
    Implicit, // doesn't use variable, but type is already defined in the block's blueprint | Appears as 0 in Visual Source
    Explicit, // doesn't use variable, and we choose the type                               | Appears as 1 in Visual Source
    Variable  // uses variable                                                              | Appears as 2 in Visual Source
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
            value: Box::new(value)
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

    pub fn from_vs<'a>(src: &'a str) -> Result<(Self, &'a str), &'static str> {
        let vs_end = 0;

        todo!()
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

#[derive(Debug)]
pub struct BlockOutput {
    pub name: VSString,
    pub var_names: Vec<VSString>, // we use Vec<T>, because of tuple outputs
}
impl BlockOutput {
    pub fn new(
        name: impl Into<VSString>,
        var_names: Vec<VSString>
    ) -> Self {
        Self {
            name: name.into(),
            var_names,
        }
    }
}
impl Display for BlockOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{U_001B}{}", self.name.into_vs(), self.var_names.iter().map(VSFieldType::into_vs).collect::<Vec<String>>().join(U_001B))
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
    pub fn new(
        internal: impl Into<VSString>,
        name: impl Into<VSString>,
        visual_position: impl Into<VSVector2>,
        child_blocks: Vec<VSString>,
        else_child_block: Option<VSString>,
        inputs: Vec<BlockInput>,
        outputs: Vec<BlockOutput>
    ) -> Box<Self> {
        Box::new(Self {
            internal: internal.into(),
            name: name.into(),
            visual_position: visual_position.into(),
            child_blocks,
            else_child_block,
            inputs,
            outputs
        })
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
                                        let parsed_json = serde_json::from_slice::<'_, Value>(json_data).or(Err("Unable to parse vs_blocks.json"))?;
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
                            let in_value = {
                                let in_value = &mut property_values[i * 3 + 1];
                                mem::take(in_value)
                            };

                            let mut vs_name = VSString::new();
                            let mut vs_value = VSString::new();

                            let _ = vs_name.from_vs(in_name.as_str());
                            let _ = vs_value.from_vs(in_value.as_str());

                            self.outputs.push(BlockOutput {
                                name: vs_name,
                                var_names: vec![vs_value]
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
}