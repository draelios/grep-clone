use std::{fs, env};
use std::error::Error;

pub struct Search {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Search {
    fn new(query: String, file_path: String, ignore_case: bool) -> Self{
        Self { query, file_path, ignore_case }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        
        args.next();  // we remove the prgram directory 
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next().as_deref() {
            Some("true") => true,
            None => env::var("IGNORE_CASE").is_ok(),
            _ => env::var("IGNORE_CASE").is_ok(),
        };
        
        Ok(Search::new(query, file_path, ignore_case))
    }
}

pub fn run (search: Search) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(search.file_path)?;

    let results = if search.ignore_case {
        search_words_case_insensitive(&content, &search.query)
    } else{
        search_words(&content, &search.query)
    };

    for line in results {
        println!("{line}")
    }
    Ok(())
}

fn search_words<'a>(content: &'a str, query: &str) -> Vec<&'a str> {  
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
} 


fn search_words_case_insensitive<'a>(content: &'a str, query: &str) -> Vec<&'a str> {
    
    content
        .lines()
        .filter(|line| {
            line.to_lowercase().contains(&query.to_lowercase())
        })
        .collect()
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