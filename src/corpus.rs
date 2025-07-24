use super::parser::schema::ProgramPair;
use crate::paths::TEMP_PROGRAMS_DIR;
use git2::Repository;

// Clone repos in a program pair into a temporary directory.
pub fn clone(pair: &ProgramPair) {
    let program_name = &pair.program_name;
    let rust_repository_url = &pair.rust_program.repository_url;
    match Repository::clone(
        &rust_repository_url,
        format!("{TEMP_PROGRAMS_DIR}/{program_name}"),
    ) {
        Ok(_) => println!("Successfully cloned!"),
        Err(error) => println!("Failed to clone: {}", error.message()),
    };
}
