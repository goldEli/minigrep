use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parse arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if let Err(e) = run(&config) {
    //     println!("Application error: {}", e);
    //     process::exit(1);
    // };
    minigrep::run(&config).unwrap_or_else(|err| {
        println!("Application error: {}", err);
        process::exit(1);
    })
}
