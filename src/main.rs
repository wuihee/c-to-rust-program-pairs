mod parser;

use parser::{individual, project};

fn main() {
    let project_metadata = project::parse("./metadata/projects/diffutils.json").expect("Failed");
    println!("{:?}", project_metadata);

    let individual_metadata =
        individual::parse("./metadata/individual/system-tools.json").expect("Failed");
    println!("{:?}", individual_metadata);
}
