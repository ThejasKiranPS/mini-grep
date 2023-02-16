use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Searching for `{0}` in {1}", config.query, config.file_path);

    let file_contents = fs::read_to_string(config.file_path).expect("Cannot read file.");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Self { query, file_path }
    }
}
