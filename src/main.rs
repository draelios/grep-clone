use std::process;
use std::env;
use grep_clone::Search;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search = Search::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });
    
    if let Err(e) = grep_clone::run(search) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


