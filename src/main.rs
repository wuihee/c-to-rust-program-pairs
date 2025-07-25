mod corpus;
pub mod parser;
mod paths;

#[cfg(test)]
mod tests {
    use super::parser::{individual, project};

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

fn main() {
    // Testing the clone functionality
    match parser::project::parse("./metadata/projects/diffutils.json") {
        Ok(metadata) => {
            let pair = &metadata.pairs[0];
            match corpus::download_program_pair(&pair) {
                Ok(_) => println!("Successfully copied files!"),
                Err(error) => println!("Failed to copy files: {}", error),
            };
        }
        Err(_) => println!("Failed to parse!"),
    };
}
