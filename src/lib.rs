use std::fs;
use std::error::Error;

pub struct Search<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Search<'a> {
    fn new(query: &'a str, file_path: &'a str) -> Self{
        Self { query, file_path }
    }

    pub fn build(args: &'a [String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!")
        }

        Ok(Search::new(&args[1], &args[2]))
    }
}

pub fn run (search: Search) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(search.file_path)?;
    for line in search_words(&content, search.query) {
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
        let query = "dUCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search_words(contents, query)
        );
    }
}