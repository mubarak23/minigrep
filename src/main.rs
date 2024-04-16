
use std::env;
use std::fs;


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // we use collect to turn the iterator into a vector containing all the values produced by the iterator
    dbg!(&args);

    let config = Config::new(&args);
    // parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // reading from a file 
    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
  
}



// fn parse_config(args: &[String]) -> Config {
//      let query = args[1].clone(); // reference the first argument pass via the CL
//     let file_path = args[2].clone();
//     Config {query, file_path}
// }