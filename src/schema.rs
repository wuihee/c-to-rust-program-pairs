use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub pairs: Vec<ProgramPair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramPair {
    pub name: String,
    pub description: String,
    pub description_url: String,
    pub translation: Translation,
    pub features: Features,
    pub c_program: Program,
    pub rust_program: Program,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub language: Language,
    pub url: String,
    pub build: Vec<String>,
    pub test: Vec<String>,
    pub dependencies: Vec<String>,
    pub source: Vec<String>,
    pub executable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Translation {
    Manual,
    Automatic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Features {
    Subset,
    Equivalent,
    Superset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    C,
    Rust,
}
