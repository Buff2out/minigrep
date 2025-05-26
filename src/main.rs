use std::{env, error::Error, process};
use minigrep::Config;


fn main() -> Result<(), Box<dyn Error>> { 

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    config.run()
}

