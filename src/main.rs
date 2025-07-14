mod individual;
mod project;
mod schema;

fn main() {
    let metadata = project::parse("./metadata/projects/diffutils.json").expect("Failed");
    let pairs = metadata.pairs;
    let item = &pairs[0];
    println!("{}", item.name);
}
