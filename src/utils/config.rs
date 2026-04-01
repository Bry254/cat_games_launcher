use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub global: Vec<RunnerOption>,
    pub runners: HashMap<String, Runner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Runner {
    #[serde(default)]
    pub command: String,
    pub options: Vec<RunnerOption>,
    #[serde(default)]
    pub variables: Vec<RunnerVarOption>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct RunnerOption {
    pub name: String,
    pub tipo: String,
    pub cmd: String,
    pub mode: String,
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunnerVarOption {
    #[serde(default)]
    pub name: String,
    pub cmd: String,
    pub mode: String,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub values: Vec<String>,
    pub var: String,
}
