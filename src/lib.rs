use std::fs;
use std::error::Error;
use std::io;

pub struct Config<'a>(Vec<&'a String>);

impl <'a>Config<'a>{

   pub fn new(args: &[String]) -> Result<Config, &'static str>{

       if args.len() < 2 {


         loop {

            let mut my_str = String::new();
            io::stdin()
                .read_line(&mut my_str)
                .expect("Should have read the input");

             print!("{my_str}");

         }

       }

      let mut arg_vec = Vec::new();

      for arg in args{

          arg_vec.push(arg);
          
      }

    
      Ok(Config(arg_vec))

   }
}

pub fn run(config: Config) -> Result <(), Box<dyn Error>>{

    let len = config.0.len();
    
    let mut contents : Vec<String> = Vec::new();

    for i in 2..=len {

      contents.push(fs::read_to_string(config.0[i - 1])?);

    }

    for content in contents{

       println!("{content}");

    }

    Ok(())
}

