use crate::schema::{IndividualMetadata, ProgramPair, ProjectMetadata};
use std::fs;

pub fn parse_individual(file: &str) -> Result<Vec<ProgramPair>, Box<dyn std::error::Error>> {
    let raw_data = fs::read_to_string(file)?;
    let metadata: IndividualMetadata = serde_yaml::from_str(&raw_data)?;
    Ok(metadata.pairs)
}

pub fn parse_project(file: &str) -> Result<Vec<ProgramPair>, Box<dyn std::error::Error>> {
    let raw_data = fs::read_to_string(file)?;
    let metadata: ProjectMetadata = serde_yaml::from_str(&raw_data)?;
    let pairs: Vec<ProgramPair> = Vec::new();

    for project_pair in metadata.pairs.iter() {
        let pair = ProgramPair {
            name: metadata.project_information.name,
        }
    }

    Ok()
}
