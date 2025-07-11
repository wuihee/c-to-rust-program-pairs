use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IndividualMetadata {
    pub pairs: Vec<ProgramPair>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgramPair {
    name: String,
    description: String,
    description_url: String,
    translation: String, // TODO: Make this enum
    features: String,    // TODO: Make this enum
    c_program: Program,
    rust_program: Program,
}

#[derive(Debug, Deserialize, Serialize)]
struct Program {
    url: String,
    version: String,
    setup: Vec<String>,
    build: Vec<String>,
    test: Vec<String>,
    dependencies: Vec<String>,
    source: String,
    executable: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMetadata {
    pub project_information: ProjectInformation,
    pub pairs: Vec<ProjectProgramPairs>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectInformation {
    pub name: String,
    description_url: String,
    translation: String,
    features: String,
    c_program: ProjectProgramInformation,
    rust_program: ProjectProgramInformation,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectProgramInformation {
    url: String,
    version: String,
    setup: Vec<String>,
    build: Vec<String>,
    test: Vec<String>,
    dependencies: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectProgramPairs {
    name: String,
    description: String,
    c_program: ProjectProgram,
    rust_program: ProjectProgram,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectProgram {
    source: String,
    executable: String,
}
