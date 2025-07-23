mod parser;

use parser::{individual, project};

fn main() {
    // Tests to show that we can parse and extract consistent data structures from individual and project metadata files.
    let project_metadata = project::parse("./metadata/projects/diffutils.json").expect("Failed");
    println!("{:?}", project_metadata);

    let individual_metadata =
        individual::parse("./metadata/individual/system-tools.json").expect("Failed");
    println!("{:?}", individual_metadata);
}
