use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct MetadataFile {
    pairs: Vec<ProgramPair>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProgramPair {
    id: String,
    name: String,
    description: String,
    c_program: Program,
    rust_program: Program,
    relationship: Relationship,
}

#[derive(Debug, Deserialize, Serialize)]
struct Program {
    url: String,
    path: String,
    commit: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Relationship {
    #[serde(rename = "type")]
    relationship_type: String,
    description_url: String,
    test_compatibility: bool,
    shared_test_suite: String,
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
