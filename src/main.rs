mod corpus;
mod parser;
mod paths;

#[cfg(test)]
mod tests {
    use crate::parser::{individual, project};

    // Tests that a project-metadata file can be successfully parsed.
    #[test]
    fn test_parse_project() {
        let result = project::parse("./metadata/projects/diffutils.json");
        assert!(
            result.is_ok(),
            "Failed to parse project metadata: {:?}",
            result.err()
        );
    }

    // Tests that an individual-metadata file can be successfully parsed.
    #[test]
    fn test_parse_individual() {
        let result = individual::parse("./metadata/individual/system-tools.json");
        assert!(
            result.is_ok(),
            "Failed to parse individual metadata: {:?}",
            result.err()
        );
    }
}

use parser::{individual, project};

fn main() {
    // Testing the clone functionality
    match individual::parse("./metadata/individual/system-tools.json") {
        Ok(metadata) => {
            match corpus::download_metadata(&metadata) {
                Ok(_) => println!("Successfully downloaded files!"),
                Err(error) => println!("Failed to copy files: {}", error),
            };
        }
        Err(error) => println!("Failed to parse: {error}"),
    };
    match project::parse("./metadata/project/diffutils.json") {
        Ok(metadata) => {
            match corpus::download_metadata(&metadata) {
                Ok(_) => println!("Successfully downloaded files!"),
                Err(error) => println!("Failed to copy files: {}", error),
            };
        }
        Err(error) => println!("Failed to parse: {error}"),
    };
}
