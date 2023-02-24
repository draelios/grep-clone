use std::process;
use std::env;
use grep_clone::Search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search = Search::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });
    
    if let Err(e) = grep_clone::run(search) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


