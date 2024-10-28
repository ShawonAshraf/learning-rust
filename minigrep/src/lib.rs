use std::error::Error;
use std::fs;

// #[derive(Debug)]
pub struct Config {
    file_name: String,
    operation: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let operation = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { file_name, operation })
    }
}


// return a dynamic error type
// this will make the function def flexible
// since no concrete err implementation needs to be supplied immediately
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_name)?;

    println!("The contents of the file is:\n{}", contents);

    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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