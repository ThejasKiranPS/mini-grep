use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    println!("Searching for `{0}` in {1}", config.query, config.file_path);

    if let Err(e) = run(config) {
        print!("Application Error: {e}");
        process::exit(1);
    }
}
