use std::error::Error;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct UsrInput {
    // Use struct instead of packing with tuple
    query: String,
    filename: String,
    pub case_sensitive: bool
}

impl UsrInput {
    pub fn new(args: &[String]) ->  Result<UsrInput, &str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
            let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
            return Ok(UsrInput{ query, filename, case_sensitive });
        }
    }
}

pub fn run(values: UsrInput) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(values.filename)?;
    //    .expect("file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //    .expect("Something went wrong");
    
    let results = if values.case_sensitive {
        search(&values.query, &contents)
    } else {
        search_case_insensitive(&values.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    return Ok(()); 
    // idiomatic way to indicate that we’re calling `run`
    // for its side effects only
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str)
    -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

     #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Someone say Duct tape?";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "Latex";
        let contents = "\
LaTeX:
verbose, turing complete, markup.
And latex costumes...";
        assert_eq!(
            vec!["LaTeX:", "And latex costumes..."],
            search_case_insensitive(query, contents)
        );
    }
}
