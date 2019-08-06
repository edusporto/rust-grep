use std::env;

use termcolor::Color;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub color: Option<Color>,
}

impl Config {
    pub fn new<I>(args: &mut I) -> Result<Config, &'static str>
    where
        I: Iterator<Item = String>,
    {
        args.next();

        let query = args.next().ok_or("Missing query argument")?;
        let filename = args.next().unwrap_or_default();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let color = Some(Color::Yellow);
        // for now, the color isn't configurable

        Ok(Config { query, filename, case_sensitive, color })
    }
}