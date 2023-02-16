use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for `{0}` in {1}", query, file_path);
}
