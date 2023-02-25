use std::{fs, env};
use std::error::Error;

pub struct Search<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool,
}

impl<'a> Search<'a> {
    fn new(query: &'a str, file_path: &'a str, ignore_case: bool) -> Self{
        Self { query, file_path, ignore_case }
    }

    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!")
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Search::new(&args[1], &args[2], ignore_case))
    }
}

pub fn run (search: Search) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(search.file_path)?;

    let results = if search.ignore_case {
        search_words_case_insensitive(&content, search.query)
    } else{
        search_words(&content, search.query)
    };

    for line in results {
        println!("{line}")
    }
    Ok(())
}

fn search_words<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    
    let mut finds: Vec<&str> = vec![];
    
    for line in content.lines() {
        if line.contains(query) {
            finds.push(line);
        }
    }

    finds
} 


fn search_words_case_insensitive<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    
    let mut finds: Vec<&str> = vec![];
    let query = query.to_lowercase();
    
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            finds.push(line);
        }
    }

    finds
} 





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_words(contents, query)
        );
    }

    #[test]
    fn case_insensitive_search() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_words_case_insensitive(contents, query)
        );
    }
}