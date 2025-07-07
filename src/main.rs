use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct MetadataFile {
    project: ProjectMetadata,
    pairs: Vec<ProgramPair>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectMetadata {
    name: String,
    description: String,
    description_url: String,
    relationship_type: String,
    c_project: Project,
    rust_project: Project,
}

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    project_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProgramPair {
    name: String,
    description: String,
    c_program: Program,
    rust_program: Program,
}

#[derive(Debug, Deserialize, Serialize)]
struct Program {
    path: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the YAML file.
    let yaml_content = fs::read_to_string("./metadata/projects/util-linux.yaml")?;

    // Parse the YAML content.
    let config: MetadataFile = serde_yaml::from_str(&yaml_content)?;

    // Count the number of items under `pairs`.
    let item_count = config.pairs.len();
    println!("Number of program pairs is: {item_count}");

    Ok(())
}
