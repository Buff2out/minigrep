use std::{env, error::Error};
use minigrep::Config;


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

    config.run()
    
    
    // --snip--
}

