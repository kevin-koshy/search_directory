use std::error::Error;
use std::fs;

#[derive(Debug, Clone)]
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
    search(&config.file_name,&config.dir.as_str());

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
    println!("{:?}", results);
    results
}