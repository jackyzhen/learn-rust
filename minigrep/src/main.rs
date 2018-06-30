extern crate minigrep;

use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
