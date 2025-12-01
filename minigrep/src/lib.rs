use std::env;
use std::fs::File;
use std::io::Read;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> anyhow::Result<Config> {
        args.next();
        let query = args
            .next()
            .ok_or_else(|| anyhow::anyhow!("query is required"))?;
        let filename = args
            .next()
            .ok_or_else(|| anyhow::anyhow!("filename is required"))?;
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> anyhow::Result<()> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.";

    #[rstest]
    #[case::match_case("duct", vec!["safe, fast, productive."])]
    #[case::no_match("nothing", vec![])]
    #[case::empty_query("", vec!["Rust:", "safe, fast, productive."])]
    fn search_test(#[case] query: &str, #[case] expected: Vec<&str>) {
        assert_eq!(expected, search(query, CONTENTS));
    }
}
