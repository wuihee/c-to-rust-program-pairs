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
    name: String,
    description: String,
    description_url: String,
    translation: Translation,
    features: Features,
    c_program: IndividualProgram,
    rust_program: IndividualProgram,
}

// Contains information for each C / Rust program in each pair.
#[derive(Debug, Serialize, Deserialize)]
struct IndividualProgram {
    url: String,
    build: Vec<String>,
    test: Vec<String>,
    dependencies: Vec<String>,
    source: Vec<String>,
    executable: String,
}

// Parses an individual metadata file into a schema::Metadata object.
pub fn parse(path: &str) -> Result<Metadata, Box<dyn Error>> {
    let raw_metadata = fs::read_to_string(path)?;
    let individual_metadata: IndividualMetadata = serde_json::from_str(&raw_metadata)?;

    let pairs: Vec<ProgramPair> = individual_metadata
        .pairs
        .into_iter()
        .map(|pair| ProgramPair {
            name: pair.name,
            description: pair.description,
            description_url: pair.description_url,
            translation: pair.translation,
            features: pair.features,
            c_program: Program {
                language: Language::C,
                url: pair.c_program.url,
                build: pair.c_program.build,
                test: pair.c_program.test,
                dependencies: pair.c_program.dependencies,
                source: pair.c_program.source,
                executable: pair.c_program.executable,
            },
            rust_program: Program {
                language: Language::Rust,
                url: pair.rust_program.url,
                build: pair.rust_program.build,
                test: pair.rust_program.test,
                dependencies: pair.rust_program.dependencies,
                source: pair.rust_program.source,
                executable: pair.rust_program.executable,
            },
        })
        .collect();

    let metadata = Metadata { pairs };
    let _ = validator::validate(&metadata, "./metadata/individual.schema.json");

    Ok(metadata)
}
