mod util;
mod model;

mod ege;

fn main() {
    let file_name = std::env::var("FILE")
        .expect("No input file is provided");

    let content = util::read_input_from_file_into_string(file_name)
        .expect("Can not read file input");

    let (projects, contributors) = model::parse_input(content);
    let executor = ege::executor;

    dbg!(projects);
    dbg!(contributors);
}
