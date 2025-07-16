use super::schema::Metadata;
use jsonschema::validator_for;
use serde_json::Value;
use std::{error::Error, fs};

// Validates a JSON object with a schema specified by its path.
pub fn validate(metadata: &Metadata, schema_path: &str) -> Result<(), Box<dyn Error>> {
    let schema_str = fs::read_to_string(schema_path)?;
    let schema: Value = serde_json::from_str(&schema_str)?;
    let validator = validator_for(&schema)?;
    let data = serde_json::to_value(metadata)?;

    validator
        .validate(&data)
        .map_err(|e| format!("Schema validation failed: {e}").into())
}
