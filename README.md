# C-Rust Program Pairs

This repository contains pairs of programs.
Each pair consists of a C program and a Rust program.

Each program is provided in its full source code tree.  For example, this is equivalent to a git clone, without the `.git/` subdirectory.

There is a script (name TBD) that re-creates or updates the contents of this repository, by downloading from the Internet and optionally doing some postprocessing.

## Metadata

There is a metadata file (name and organization TBD) that includes:

- a URL for the C program
- a URL for the Rust program
- a URL that describes the relationship between the programs.  This might be the README for the Rust program, or a blog post, etc.
- whether the Rust program is a direct translation, a rewrite, or a program inspired by the C program that might have different features.  If a direct translation or a rewrite, then the tests of each version ought to pass on the other implementation -- if they do not, that is a bug rather than a design choice.

### Terminology

- **Fields**: Singular properties, attributes or fields that give us information.
- **Project**: The entire repository or project that has been translated from C to Rust.
- **Program**: An individual program that can be executed; each project can contain a single or multiple programs.
- **Pair**: A pair of C and Rust programs, where the Rust program was translated or inspired by the C program.
- **Metadata**: A single metadata file containing information about a project and it's programs.

### Schema Files

## CLI Specification

1. Create a folder with the name of the program-pair.
2. Clone the C repository into a temporary directory, but check if this has already been done.
3. Copy the C program from the cloned repository into our folder.
4. Clone the Rust repository into a temporary directory, but check if this has already been done.
5. Copy the Rust program from the cloned repository into our folder.

## Resources

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust): Contains a list of awesome Rust projects - many of which might be translated from C projects.
