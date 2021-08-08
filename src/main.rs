use std::{env, process};
use rust_fearless_concurrency::{Config, search_and_count, search_and_count_concurrently};

fn main() {

    let args: env::Args = env::args();

    let config = Config::new(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let dir : Vec<String> = config.dir.map(|path| {
        path.unwrap().path().display().to_string()
    }).collect();
    for filename in dir {
        let contents : String = std::fs::read_to_string(&filename).unwrap();
        // println!("Value of CONCURRENTLY is {}", config.concurrently);
        if config.concurrently {
            let count = search_and_count_concurrently("how", &contents);
            println!("The word \"{}\" appears {} time in file \"{}\"","how", count, filename);
        } else {
            let count = search_and_count("how", &contents);
            println!("The word \"{}\" appears {} time in file \"{}\"","how", count, filename);
        }
    }

}
