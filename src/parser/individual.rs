use super::schema::{Features, Language, Metadata, Program, ProgramPair, Translation};
use super::validator;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

// Schema for individual metadata files.
#[derive(Debug, Serialize, Deserialize)]
struct IndividualMetadata {
    pairs: Vec<IndividualProgramPair>,
}

// Contains information for each program pair.
#[derive(Debug, Serialize, Deserialize)]
struct IndividualProgramPair {
    program_name: String,
    program_description: String,
    translation_method: Translation,
    translation_tool: String,
    feature_relationship: Features,
    c_program: IndividualProgram,
    rust_program: IndividualProgram,
}

// Contains information for each C / Rust program in each pair.
#[derive(Debug, Serialize, Deserialize)]
struct IndividualProgram {
    documentation_url: String,
    repository_url: String,
    build_commands: Vec<String>,
    test_commands: Vec<String>,
    dependencies: Vec<String>,
    source_paths: Vec<String>,
    executable_paths: Vec<String>,
}

// Parses an individual metadata file into a schema::Metadata object.
pub fn parse(path: &str) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let individual_metadata: IndividualMetadata = serde_json::from_str(&raw_metadata)?;

    let pairs: Vec<ProgramPair> = individual_metadata
        .pairs
        .into_iter()
        .map(|pair| ProgramPair {
            program_name: pair.program_name,
            program_description: pair.program_description,
            translation_method: pair.translation_method,
            translation_tool: pair.translation_tool,
            feature_relationship: pair.feature_relationship,
            c_program: Program {
                language: Language::C,
                documentation_url: pair.c_program.documentation_url,
                repository_url: pair.c_program.repository_url,
                build_commands: pair.c_program.build_commands,
                test_commands: pair.c_program.test_commands,
                dependencies: pair.c_program.dependencies,
                source_paths: pair.c_program.source_paths,
                executable_paths: pair.c_program.executable_paths,
            },
            rust_program: Program {
                language: Language::Rust,
                documentation_url: pair.rust_program.documentation_url,
                repository_url: pair.rust_program.repository_url,
                build_commands: pair.rust_program.build_commands,
                test_commands: pair.rust_program.test_commands,
                dependencies: pair.rust_program.dependencies,
                source_paths: pair.rust_program.source_paths,
                executable_paths: pair.rust_program.executable_paths,
            },
        })
        .collect();

    let metadata = Metadata { pairs };
    let _ = validator::validate(&metadata, "./schema/main/individual-pairs.schema.json");

    Ok(metadata)
}
