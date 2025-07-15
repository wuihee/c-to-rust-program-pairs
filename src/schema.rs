use serde::{Deserialize, Serialize};

// Our final, metadata data structure that we extract from individual /
// project metadata files.
#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub pairs: Vec<ProgramPair>,
}

// Contains information about each C-to-Rust program pair.
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

// Contains information about each C or Rust program.
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

// Specifies the method of translating from C to Rust.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Translation {
    Manual,
    Automatic,
}

// Specifies the features of the Rust project in relation to its C counterpart.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Features {
    Subset,
    Equivalent,
    Superset,
}

// Specifies the language used for the project.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    C,
    Rust,
}
