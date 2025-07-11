use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramInfo {
    pub url: String,
    pub version: Option<String>,
    pub setup: Option<Vec<String>>,
    pub build: Vec<String>,
    pub test: Vec<String>,
    pub dependencies: Vec<String>,
    pub source: String,
    pub executable: String,
}

fn main() {}
