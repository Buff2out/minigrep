use std::fs;
use std::error::Error;
use std::cmp::Ordering::{Less, Equal, Greater};


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len().cmp(&2) {
            Equal | Greater => {
                let query = args[0].clone();
                let file_path = args[1].clone();
                
                Ok(Config { query, file_path })
            },
            Less => Err("not enough args"),
        }
    }
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(self.file_path)?;
    
        println!("{:?}", contents);
        Ok(())
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
        match line.contains(query) {
            true => res.push(line),
            false => (),
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}