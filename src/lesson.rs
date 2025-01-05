use std::collections::BTreeSet;

use serde::Deserialize;

use crate::tools::{byte_count, tokenize};

#[derive(Deserialize, Debug)]
pub enum Test {
    #[serde(untagged)]
    CEmu {
        input: Vec<Variable>,
        output: Vec<Variable>,
    },
    #[serde(untagged)]
    FulltextMatch { regex: String },
    #[serde(untagged)]
    Group(Vec<Test>),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum VariableData {
    String(String),
    RealList(Vec<f64>),
    RealNumber(f64),
}

impl VariableData {
    pub fn file_extension(&self) -> &str {
        match self {
            VariableData::String(_) => "8xs",
            VariableData::RealList(_) => "8xl",
            VariableData::RealNumber(_) => "8xn",
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: VariableData,
}

#[derive(Deserialize)]
pub struct Lesson {
    pub id: u16,
    pub name: String,
    pub requirements: BTreeSet<u16>,
    pub starting_program: String,
    pub required_savings: usize,
    pub brief_description: Option<String>,
    pub tests: Vec<Test>,
}

impl Lesson {
    pub fn byte_threshold(&self) -> usize {
        byte_count(&tokenize(&self.starting_program).collect::<Vec<_>>()) - self.required_savings
    }
}
