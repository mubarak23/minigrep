
use std::fs;
use std::error::Error;

pub struct Config {
  query: String,
  file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
     // reading from a file 
    let contents = fs::read_to_string(config.file_path)
     .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    Ok(())

}