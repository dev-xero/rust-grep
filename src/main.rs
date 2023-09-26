use std::{env, fs, process};

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments, expected format: [search_text] [file_path]");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {
            query,
            file_path
        })
    }
}

fn run(config: Config) {
    let contents = fs::read_to_string(&config.file_path)
        .expect("Should have been able to read the file");

    println!("Contents: {}", contents);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });
    
    println!("Searching for: {}", config.query);
    println!("In file: {}\n", config.file_path);

    run(config);
}
