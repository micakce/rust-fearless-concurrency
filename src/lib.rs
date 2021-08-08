use std::{env, fs::{self, ReadDir}, process, sync::{Arc, Mutex}, thread};


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub dir: ReadDir,
    pub concurrently: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

        args.next();

        let query  = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query argument")
        };

        let dir  = match args.next() {
            Some(arg) => fs::read_dir(arg).unwrap_or_else(|err| {
                eprint!("Problems parsing aguments: {}", err);
                process::exit(1);
            }),
            None => return Err("Didn't get a folder argument")
        };

        // let concurrently = env::var("CONCURRENTLY").is_err();
        let concurrently = match env::var("CONCURRENTLY") {
            Ok(val) => {
                if val == "1" || val == "true" {
                    println!("Running program multi threaded");
                    true
                } else {
                    println!("Running program single threaded, must pass \"1\" or \"true\"" );
                    false
                }
            },
            Err(_e) => {
                // println!("couldn't interpret {}: {}", "CONCURRENTLY", e);
                println!("Running program single threaded");
                false
            }
        };

        Ok(Config { query, dir, concurrently})
    }
}

pub fn search_and_count_concurrently(query: &str, contents: &str) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // divide the file content in 4 chunks, one for each thread
    let chunk_size = contents.len()/4;
    let content_chunks = (0..4).map(|seg| {
        String::from(&contents[chunk_size*seg..chunk_size*(seg+1)])
    });

    for chunks in content_chunks {
        let query = query.to_lowercase();
        let counter = Arc::clone(&counter);
        let  handle = thread::spawn(move || {
            for line in chunks.lines() {
                for word in line.to_lowercase().split(" ") {
                    if word == query {
                        let mut num = counter.lock().unwrap();
                        *num+=1;
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let x = *counter.lock().unwrap(); x
}


pub fn search_and_count<'a>(query: &str, contents: &'a str) -> i32 {
    let counter = Mutex::new(0);
    let query = query.to_lowercase();

    for line in contents.lines() {
        for word in line.to_lowercase().split(" ") {
            if word == query {
                let mut num = counter.lock().unwrap();
                *num+=1;
            }
        }
    }
    let x = *counter.lock().unwrap(); x
}
