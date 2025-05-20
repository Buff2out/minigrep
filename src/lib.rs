use std::fs;
use std::error::Error;
use std::cmp::Ordering::{Less, Equal, Greater};


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub contents: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len().cmp(&3) {
            Equal | Greater => {
                let query = args[1].clone();
                let file_path = args[2].clone();
                let contents = "".to_string();
                Ok(Config { query, file_path, contents })
            },
            Less => Err("not enough args"),
        }
    }
    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.contents = fs::read_to_string(self.file_path)?;
        let res = search(&self.query, &self.contents);
        println!("{:?}", res);
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