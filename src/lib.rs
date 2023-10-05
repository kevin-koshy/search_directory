use std::error::Error;
use std::fs;


pub struct Config{
    pub dir: String,
    pub file_name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let dir = args[1].clone();
        let file_name = args[2].clone();
        Ok(Config { dir, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    recursive_search(&config.file_name,&config.dir.as_str());
    Ok(())
     }

fn search<'a>(file_to_search:&str,  directory:&'a str) -> Vec< String> {
    let files = fs::read_dir(directory).unwrap().filter_map(|e|e.ok()).map(|e|e.path()).collect::<Vec<_>>();
    let mut results = Vec::new();
    println!("Searching for {} in {}", file_to_search, directory);
    for file in files {
        if file.ends_with(file_to_search) {
            results.push(file.into_os_string().into_string().unwrap())  ;
                }
    }
    if results.is_empty() {
    }
    results
}

fn recursive_search<'a>(file_to_search:&str,  directory:&'a str) -> Vec< String> {
    let files = fs::read_dir(directory).unwrap().filter_map(|e|e.ok()).map(|e|e.path()).collect::<Vec<_>>();
    let mut results:Vec<String> = Vec::new();
    println!("Recursively searching for {} in {}", file_to_search, directory);
    for path in files {
        if path.is_dir() {
            let mut rec_search_results = recursive_search(file_to_search, &path.display().to_string());
            if !results.is_empty() {
                results = rec_search_results;
                println!("Found {} in {}", file_to_search, directory);
                break;
            }
            // let search_results = search(file_to_search, &path.display().to_string());
            // if !search_results.is_empty() {
            //     results = search_results;
            //      println!("Found¸ {} in {}", file_to_search, directory);
            //     break;
            // }
        }
        else {
            if path.ends_with(file_to_search) {
                results.push(path.into_os_string().into_string().unwrap());
                println!("Found {} in {}", file_to_search, directory);
                break;
            }
        }
    }
    // println!("{:?}", results);
    results
}