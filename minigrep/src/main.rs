use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;


fn main() {
    let args:Vec<String>= env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("A problem occur when parsing arguments: {} ",err);
        process::exit(1);
    });
    println!("Searching for {}",config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
