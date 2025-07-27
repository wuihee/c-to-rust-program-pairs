use super::{parser::schema::ProgramPair, paths::PROGRAMS_DIRECTORY};
use git2::{FetchOptions, Progress, RemoteCallbacks, build::RepoBuilder};
use indicatif::{ProgressBar, ProgressStyle};
use std::{error::Error, fs, path::Path};
use tempfile;

// Downloads a program-pair into the ./programs directory.
// The C source files will be downloaded to ./programs/<program_name>/c-program.
// The Rust source files will be downloaded to ./programs/<program_name>/rust-program.
pub fn download_program_pair(pair: &ProgramPair) -> Result<(), Box<dyn Error>> {
    let program_name = &pair.program_name;
    let base_program_path = Path::new(PROGRAMS_DIRECTORY).join(program_name);
    let c_program_path = base_program_path.join("c-program");
    let rust_program_path = base_program_path.join("rust-program");
    fs::create_dir_all(&c_program_path)?;
    fs::create_dir_all(&rust_program_path)?;

    download_files(
        program_name,
        &c_program_path,
        &pair.c_program.repository_url,
        &pair.c_program.source_paths,
    )?;
    download_files(
        program_name,
        &rust_program_path,
        &pair.rust_program.repository_url,
        &pair.rust_program.source_paths,
    )?;

    Ok(())
}

// Clones a repository copy the relevant files to the program directory.
// Also prints a progress bar to standard output to track progress.
fn download_files(
    program_name: &str,
    program_directory: &Path,
    repository_url: &str,
    files: &[String],
) -> Result<(), Box<dyn Error>> {
    // Create a progress bar.
    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.cyan} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
    );
    progress_bar.set_message(format!("Cloning {program_name}..."));

    // Set up remote callbacks for progress tracking.
    let mut remote_callbacks = RemoteCallbacks::new();

    // Clone progress callback.
    remote_callbacks.transfer_progress(|progress: Progress| {
        let received_objects = progress.received_objects();
        let received_bytes = progress.received_bytes();
        let total_objects = progress.total_objects();
        let indexed_objects = progress.indexed_objects();

        // Downloading objects.
        if received_objects < total_objects {
            progress_bar.set_length(total_objects as u64);
            progress_bar.set_position(received_objects as u64);
            progress_bar.set_message(format!(
                "Receiving objects: {received_objects}/{total_objects} ({received_bytes:.1} MB)"
            ));
        }
        // Processing downloaded objects.
        else if indexed_objects < total_objects {
            progress_bar.set_position(indexed_objects as u64);
            progress_bar.set_message(format!(
                "Indexing objects: {indexed_objects}/{total_objects}"
            ));
        }
        // Done with objects.
        else {
            progress_bar.set_message("Resolving deltas...");
        }

        true
    });

    // Setup up fetch options with our callbacks.
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    // Clone only the latest commit to save time / space.
    fetch_options.depth(1);

    // Clone the repository.
    let temporary_directory = tempfile::tempdir()?;
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let repository = builder.clone(
        &repository_url,
        &temporary_directory.path().join(program_name),
    )?;

    progress_bar.set_message("Copying files...");
    progress_bar.set_style(ProgressStyle::default_spinner());

    // Copy the given files from the repository to the given directory.
    for file_path in files.iter() {
        let file_name = Path::new(file_path)
            .file_name()
            .ok_or("Invalid file path.")?;
        let source = repository
            .workdir()
            .ok_or("Failed to find source file in repository.")?
            .join(&file_path);
        let destination = program_directory.join(file_name);
        if !destination.exists() {
            fs::copy(source, destination)?;
        }
    }

    progress_bar.finish_with_message("Download completed!");
    Ok(())
}
