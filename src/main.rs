mod utils;
mod types;
mod term;

fn main() {
    println!("{:?}", utils::path::load_dir("./sample"));
}
