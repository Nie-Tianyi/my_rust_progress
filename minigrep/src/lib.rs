use std::error::Error;
use std::fs;
use std::env;

#[derive(Debug)]
pub struct Config{
    pub query:String,
    pub file_path:String,
    pub ignore_case:bool,
}

impl Config {
    pub fn build(args:&[String]) -> Result<Config,&str>{
        if args.len() < 3 {
            return Err("Not Enough Argument, use: cargo run -query -file_path");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query,file_path,ignore_case})
    }
}

pub fn run(cfg:Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(cfg.file_path)?;

    let results = if cfg.ignore_case {
        case_insensitivity_search(&cfg.query,&contents)
    }else {
        search(&cfg.query,&contents)
    };

    for line in results{
        println!("{}",line);
    }

    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    //vec![]
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(&query){
            results.push(line);
        }
    }

    results
}

pub fn case_insensitivity_search<'a>(query:&str, contents:&'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/*******************Test Driven Development***********************/

/*
1. write a wrong test
2. implement function
3. write a correct test
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_one_words(){
        let query = "duct";
        let content = "\
Rust:
safe,fast,productive.
Pick three.";

        assert_eq!(vec!["safe,fast,productive."],search(&query,&content));

    }

    #[test]
    fn ignore_case(){
        let query = "rUsT";
        let content = "\
Rust:
safe,fast,productive.
Pick three.";
        assert_eq!(vec!["Rust:"], case_insensitivity_search(&query, &content));
    }
}