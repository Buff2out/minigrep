use std::fs;
use std::error::Error;
// use std::cmp::Ordering::{Less, Equal, Greater};


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub contents: String,
}

impl Config {
    pub fn new(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
            args.next();
            let query = match args.next() {
                Some(val) => val,
                None => return Err("Didn't get a query str"),
            };
            let file_path = match args.next() {
                Some(fpath) => fpath,
                None => return Err("Uncorrect filepath"),
            };
            let contents = "".to_string();
            Ok(Config { query, file_path, contents })
    }
    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        self.contents = fs::read_to_string(self.file_path)?;
        let res = search(&self.query, &self.contents);
        println!("{:?}", res);
        Ok(())
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query) ).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // лишняя операция, 
    // подумать как можно оптимизировать

    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
        match line.to_lowercase().contains(&query) {
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
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}