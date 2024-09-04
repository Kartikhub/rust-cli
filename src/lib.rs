use std::fs;
use std::error::Error;
use std::env;

pub struct Arguments {
    search_query: String,
    file_path: String,
    ignore_case: bool
}

impl Arguments {
    pub fn build(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 3 {
            return Err("Insufficient arguments");
        }

//      TODO: Update for efficiency
        let search_query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Arguments { search_query, file_path, ignore_case })
    }
}

pub fn run(cli_args: Arguments) -> Result<(), Box<dyn Error>> { 

    let contents = fs::read_to_string(cli_args.file_path)?;

    let results = if cli_args.ignore_case {
        search_case_insensitive(&cli_args.search_query, &contents)
    } else {
        search(&cli_args.search_query, &contents)
    };

    for (i, line) in results {
        println!("LINE: {} {}", i, line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize , &'a str)> {
    let mut result: Vec<(usize, &str)> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.contains(query) {
            result.push((i+1, line));
        }
    }
    result
}


fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize , &'a str)> {
    let mut result: Vec<(usize, &str)> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            result.push((i+1, line));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_test() {
        let query = "effi";
        let contents = "\
Rust:
reliable, efficient, fearless.
Choose all three.";

        assert_eq!(vec![(2, "reliable, efficient, fearless.")], search(query, contents));
    }

    #[test]
    fn test_search_case_insenitive() {
        let query = "FEar";
        let contents = "\
Rust:
reliable, efficient, fearless.
Choose all three.";

        assert_eq!(vec![(2, "reliable, efficient, fearless.")], search(query, contents));
    }
}
