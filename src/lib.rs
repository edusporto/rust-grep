use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::env;

extern crate termcolor;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let s: Vec<&str> = search(query, contents)
            .iter()
            .map(|a| a.line)
            .collect();

        assert_eq!(vec!["safe, fast, productive."], s);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let s: Vec<&str> = search_case_insensitive(query, contents)
            .iter()
            .map(|a| a.line)
            .collect();
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            s
        );
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 1) Read content to be searched
    let content = read_content(&config)?;

    // 2) Search the query in the specified content
    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    // 3) Print the lines from the content containing the query
    print_results(&results, &config)?;

    Ok(())
}

fn read_content(config: &Config) -> Result<String, io::Error> {
    if config.filename.is_empty() {
        // if no file is specified, the program will read
        // from stdin until EOF
        let mut buffer = String::new();
        while io::stdin().read_to_string(&mut buffer)? != 0 {}

        Ok(buffer)
    } else {
        fs::read_to_string(&config.filename)
    }
}

fn print_results(results: &Vec<Match>, config: &Config) -> Result<(), Box<dyn Error>> {
    let query_len = config.query.chars().count();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let standard_color = ColorSpec::new();
    stdout.set_color(&standard_color)?;

    for m in results {
        write!(&mut stdout, "{}", &m.line[0 .. m.index])?;
        // writes everything up to the matched query

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        // sets the color to blue

        write!(&mut stdout, "{}", &m.line[m.index .. m.index+query_len])?;
        // writes the matched query

        stdout.set_color(&standard_color)?;
        // sets the color back to default

        writeln!(&mut stdout, "{}",
                 &m.line[m.index+config.query.chars().count() ..])?;
        // writes everything past the matched query
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<Match<'a>> {
    let mut results = Vec::new();

    for line in content.lines() {
        if let Some(index) = line.find(query) {
            results.push(Match { line, index });
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<Match<'a>> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in content.lines() {
        if let Some(index) = line.to_lowercase().find(&query) {
            results.push(Match { line, index });
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<I>(args: &mut I) -> Result<Config, &'static str>
    where
        I: Iterator<Item = String>,
    {
        args.next();

        let query = args.next().ok_or("Missing query argument")?;
        let filename = args.next().unwrap_or(String::new());
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub struct Match<'a> {
    pub line: &'a str,
    pub index: usize,
}
