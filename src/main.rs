mod generators;
mod model;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

fn main() {
    let project = generators::projects::generate_project(4, 3, 4);
    let serialised = serde_json::to_string_pretty(&project).unwrap();
    println!("{}", &serialised);
}
