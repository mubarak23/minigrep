
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // we use collect to turn the iterator into a vector containing all the values produced by the iterator
    dbg!(&args);

    let query = &args[1]; // reference the first argument pass via the CL
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", &file_path);

    // reading from a file 
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
  
}
