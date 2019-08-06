use std::env;
use std::process;

use rust_grep::config::Config;

fn main() {
    let mut args = env::args();
    
    let config = Config::new(&mut args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
