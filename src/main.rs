use std::{env, fs};

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments, they should follow the format [search_text] [file_path]");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query,
            file_path
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let contents = fs::read_to_string(&config.file_path)
        .expect("Should have been able to read the file");

    println!("Searching for: {}", config.query);
    println!("In file: {}\n", config.file_path);
    println!("Contents: {}", contents);
}
