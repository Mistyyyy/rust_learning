use std::env;
use std::process;

mod lib;

fn main() {
    let args_vec: Vec<String> = env::args().collect();

    let config  = lib::Config::new(&args_vec).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(&config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

