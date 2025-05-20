use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let config = match config {
        Ok(val) => val,
        Err(msg) => {
            println!("{}", msg);
            return Err(msg.into());
        },
    };

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config)
    
    
    // --snip--
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("{:?}", contents);
    Ok(())
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