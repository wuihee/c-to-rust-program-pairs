use std::{error::Error, fs};

use super::schema::{Features, Language, Metadata, Program, ProgramPair, Translation};
use serde::{Deserialize, Serialize};

// Schema for project metadata files.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectMetadata {
    project: ProjectGlobal,
    pairs: Vec<ProjectProgramPair>,
}

// Global information about the project that apply to every program pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectGlobal {
    name: String,
    description_url: String,
    translation: Translation,
    features: Features,
    c_program: ProjectGlobalProgram,
    rust_program: ProjectGlobalProgram,
}

// Global information that applies to each specific C / Rust program.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectGlobalProgram {
    url: String,
    build: Vec<String>,
    test: Vec<String>,
    dependencies: Vec<String>,
}

// Specific information for each individual program pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectProgramPair {
    name: String,
    description: String,
    c_program: ProjectProgram,
    rust_program: ProjectProgram,
}

// Specific information for each individual C / Rust program in a pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectProgram {
    source: Vec<String>,
    executable: String,
}

// Parses a project metadata file into a schema::Metadata object.
pub fn parse(path: &str) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let project_metadata: ProjectMetadata = serde_json::from_str(&raw_metadata)?;
    let global_metadata = project_metadata.project;

    let pairs: Vec<ProgramPair> = project_metadata
        .pairs
        .into_iter()
        .map(|pair| ProgramPair {
            name: pair.name,
            description: pair.description,
            description_url: global_metadata.description_url.clone(),
            translation: global_metadata.translation.clone(),
            features: global_metadata.features.clone(),
            c_program: Program {
                language: Language::C,
                url: global_metadata.c_program.url.clone(),
                build: global_metadata.c_program.build.clone(),
                test: global_metadata.c_program.test.clone(),
                dependencies: global_metadata.c_program.dependencies.clone(),
                source: pair.c_program.source,
                executable: pair.c_program.executable,
            },
            rust_program: Program {
                language: Language::Rust,
                url: global_metadata.rust_program.url.clone(),
                build: global_metadata.rust_program.build.clone(),
                test: global_metadata.rust_program.test.clone(),
                dependencies: global_metadata.rust_program.dependencies.clone(),
                source: pair.rust_program.source,
                executable: pair.rust_program.executable,
            },
        })
        .collect();

    Ok(Metadata { pairs })
}
