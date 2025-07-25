use super::parser::schema::ProgramPair;
use super::paths::PROGRAMS_DIRECTORY;
use git2::Repository;
use std::{error::Error, fs, path::Path};
use tempfile::tempdir;

// TODO: Refactor out redundancy.
// Clone repos in a program pair into a temporary directory.
pub fn download_program_pair(pair: &ProgramPair) -> Result<(), Box<dyn Error>> {
    // Clone repository into temporary directory.
    let program_name = &pair.program_name;
    let c_repository_url = &pair.c_program.repository_url;
    let rust_repository_url = &pair.rust_program.repository_url;
    let c_temporary_directory = tempdir()?;
    let rust_temporary_directory = tempdir()?;
    let c_repository = Repository::clone(
        &c_repository_url,
        c_temporary_directory.path().join(&program_name),
    )?;
    let rust_repository = Repository::clone(
        &rust_repository_url,
        rust_temporary_directory.path().join(&program_name),
    )?;

    // Create directory for program pair.
    let program_directory = format!("{PROGRAMS_DIRECTORY}/{program_name}");
    let rust_program_directory = format!("{program_directory}/rust_program");
    let c_program_directory = format!("{program_directory}/c_program");
    fs::create_dir_all(&program_directory)?;
    fs::create_dir_all(&rust_program_directory)?;
    fs::create_dir_all(&c_program_directory)?;

    // Copy paths from repository into program directory.
    for file_path in pair.c_program.source_paths.iter() {
        let file_name = Path::new(file_path).file_name().unwrap();
        let source = c_repository.workdir().unwrap().join(&file_path);
        let destination = Path::new(&c_program_directory).join(file_name);
        if !destination.exists() {
            fs::copy(source, destination)?;
        }
    }

    for file_path in pair.rust_program.source_paths.iter() {
        let file_name = Path::new(file_path).file_name().unwrap();
        let source = rust_repository.workdir().unwrap().join(&file_path);
        let destination = Path::new(&rust_program_directory).join(file_name);
        if !destination.exists() {
            fs::copy(source, destination)?;
        }
    }

    Ok(())
}
