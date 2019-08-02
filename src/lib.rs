use std::error::Error;
use std::fs;
use std::io::{self, Read};

extern crate termcolor;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        let s: Vec<&str> = search(query, contents)
            .iter()
            .map(|a| a.line)
            .collect();

        assert_eq!(vec!["safe, fast, productive."], s);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = if config.filename.is_empty() {
        // if no file is specified, the program will read
        // from stdin until EOF
        let mut buffer = String::new();
        while io::stdin().read_to_string(&mut buffer)? != 0 {}

        buffer
    } else {
        fs::read_to_string(&config.filename)?
    };

    let results = search(&config.query, &content);

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let standard_color = ColorSpec::new();
    stdout.set_color(&standard_color)?;

    for m in results {
        write!(&mut stdout, "{}", &m.line()[0..m.index()])?;
        // writes everything up to the matched query

        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        // sets the color to blue

        write!(&mut stdout, "{}", &config.query)?;
        // writes the matched query

        stdout.set_color(&standard_color)?;
        // sets the color back to default

        writeln!(&mut stdout, "{}",
                 &m.line()[m.index()+config.query.chars().count()..])?;
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

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new<I>(args: &mut I) -> Result<Config, &'static str>
    where
        I: Iterator<Item = String>,
    {
        args.next();

        let query = args.next().ok_or("Missing query argument")?;
        let filename = args.next().unwrap_or(String::new());

        Ok(Config { query, filename })
    }
}

pub struct Match<'a> {
    line: &'a str,
    index: usize,
}

impl<'a> Match<'a> {
    pub fn new(line: &'a str, index: usize) -> Match<'a> {
        Match {line, index}
    }

    pub fn line(&self) -> &'a str {
        self.line
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
