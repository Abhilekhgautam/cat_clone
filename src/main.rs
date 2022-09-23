use std::env;
use std::process;
use cat_clone::Config;

fn main() {
    
    let args : Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{

       println!("Problem Parsing arguments: {err}");
       process::exit(1);

    }); // collect fn returns a collection

    if let Err(e) = cat_clone::run(config) {
 
      println!("Application Error {e}");

      process::exit(1);

    }

}
