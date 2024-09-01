use std::fs;
use std::error::Error;


pub struct Arguments {
    search_query: String,
    file_path: String
}

impl Arguments {
    pub fn build(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments");
        }

//      TODO: Update for efficiency
        let search_query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Arguments { search_query, file_path })
    }
}

pub fn run(cli_args: Arguments) -> Result<(), Box<dyn Error>> { 
    let contents = fs::read_to_string(cli_args.file_path)?;
    println!("{contents}");

    Ok(())
}
