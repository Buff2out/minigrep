use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let config = match config {
        Ok(val) => val,
        Err(msg) => {
            println!("{}", msg);
            return;
        },
    };

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    
    println!("{:?}", contents);
    // --snip--
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough args");
        }
        let query = args[0].clone();
        let file_path = args[1].clone();
        
        Ok(Config { query, file_path })
    }
}