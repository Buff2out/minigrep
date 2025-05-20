use std::fs;
use std::error::Error;


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough args");
        }
        let query = args[0].clone();
        let file_path = args[1].clone();
        
        Ok(Config { query, file_path })
    }
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(self.file_path)?;
    
        println!("{:?}", contents);
        Ok(())
    }
}