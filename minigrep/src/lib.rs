use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let results = match config.case_sensitive {
        true => search(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents),
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const CONTENTS: &str = "\
        Rust:
safe, fast, productive.
Pick three.
Lol jokes.
Go is much better dude!";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        assert_eq!(vec!["safe, fast, productive."], search(query, CONTENTS));
    }

    #[test]
    fn case_insensitive() {
        let query = "DuCt";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_insensitive(query, CONTENTS)
        );
    }

    #[test]
    fn multiple_result() {
        let query = "u";
        assert_eq!(
            vec![
                "Rust:",
                "safe, fast, productive.",
                "Go is much better dude!",
            ],
            search(query, CONTENTS)
        );
    }

    #[test]
    fn no_result() {
        let query = "safe, slow, hard.";
        assert_eq!(Vec::<String>::new(), search(query, CONTENTS));
    }
}
