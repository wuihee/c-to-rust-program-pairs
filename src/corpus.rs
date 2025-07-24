use super::parser::schema::ProgramPair;
use git2::Repository;

// Clone repos in a program pair into a temporary directory.
pub fn clone(pair: &ProgramPair) {
    let rust_repository_url = &pair.rust_program.repository_url;
    println!("Rust Repository URL: {rust_repository_url}");
    match Repository::clone(&rust_repository_url, "./diffutils") {
        Ok(_) => println!("Successfully cloned!"),
        Err(error) => println!("Failed to clone: {}", error.message()),
    };
}
