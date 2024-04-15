
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // we use collect to turn the iterator into a vector containing all the values produced by the iterator
    dbg!(args);
  
}
