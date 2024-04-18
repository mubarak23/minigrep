
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

    for line in search(&config.query, &contents) {
      println!("{line}");
    }
   //  println!("With text:\n{contents}");

    Ok(())

}


pub fn search<'a>(query: &str, _contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();  
  for line in _contents.lines() {
      if line.contains(query){
        result.push(line);
      }
    }
    result
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
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}