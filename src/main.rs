
use std::env;
use std::process;


use minigrep::Config;



fn main() {
    let args: Vec<String> = env::args().collect(); // we use collect to turn the iterator into a vector containing all the values produced by the iterator
    // dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // unwrap_or_else allows us to define some custom, non-panic! error handling.
    // parse_config(&args);


   if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
   }
  
}



// fn parse_config(args: &[String]) -> Config {
//      let query = args[1].clone(); // reference the first argument pass via the CL
//     let file_path = args[2].clone();
//     Config {query, file_path}
// }