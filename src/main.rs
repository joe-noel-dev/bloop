mod generators;
mod model;

fn main() {
    let project = generators::projects::generate_project(4, 3, 4);
    dbg!(project);
}
