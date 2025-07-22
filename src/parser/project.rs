use super::schema::{Features, Language, Metadata, Program, ProgramPair, Translation};
use super::validator;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, fs};

// Schema for project metadata files.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectMetadata {
    project_information: ProjectInformation,
    pairs: Vec<ProjectProgramPair>,
}

// Global information about the project that apply to every program pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectInformation {
    program_name: String,
    translation_method: Translation,
    translation_tool: String,
    feature_relationship: Features,
    c_program: ProjectGlobalProgram,
    rust_program: ProjectGlobalProgram,
}

// Global information that applies to each specific C / Rust program.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectGlobalProgram {
    documentation_url: String,
    repository_url: String,
    build_commands: Vec<String>,
    test_commands: Vec<String>,
    dependencies: HashMap<String, Vec<String>>,
}

// Specific information for each individual program pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectProgramPair {
    program_name: String,
    program_description: String,
    c_program: ProjectProgram,
    rust_program: ProjectProgram,
}

// Specific information for each individual C / Rust program in a pair.
#[derive(Debug, Serialize, Deserialize)]
struct ProjectProgram {
    source_paths: Vec<String>,
    executable_paths: Vec<String>,
}

// Parses a project metadata file into a schema::Metadata object.
pub fn parse(path: &str) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let project_metadata: ProjectMetadata = serde_json::from_str(&raw_metadata)?;
    let global_metadata = project_metadata.project_information;

    let pairs: Vec<ProgramPair> = project_metadata
        .pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name,
            program_description: pair.program_description,
            translation_method: global_metadata.translation_method.clone(),
            translation_tool: global_metadata.translation_tool.clone(),
            feature_relationship: global_metadata.feature_relationship.clone(),
            c_program: Program {
                language: Language::C,
                documentation_url: global_metadata.c_program.documentation_url.clone(),
                repository_url: global_metadata.c_program.repository_url.clone(),
                build_commands: global_metadata.c_program.build_commands.clone(),
                test_commands: global_metadata.c_program.test_commands.clone(),
                dependencies: global_metadata.c_program.dependencies.clone(),
                source_paths: pair.c_program.source_paths,
                executable_paths: pair.c_program.executable_paths,
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: global_metadata.rust_program.documentation_url.clone(),
                repository_url: global_metadata.rust_program.repository_url.clone(),
                build_commands: global_metadata.rust_program.build_commands.clone(),
                test_commands: global_metadata.rust_program.test_commands.clone(),
                dependencies: global_metadata.rust_program.dependencies.clone(),
                source_paths: pair.rust_program.source_paths,
                executable_paths: pair.rust_program.executable_paths,
            },
        })
        .collect();

    let metadata = Metadata { pairs };
    let _ = validator::validate(&metadata)?;

    Ok(metadata)
}
