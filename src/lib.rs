//! # Art
//!
//! A library for modeling artistic concepts.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


pub mod kinds {
	/// The primary colors according to the RYB color model.
	pub enum PrimaryColor {
			Red,
			Yellow,
			Blue,
	}

	/// The secondary colors according to the RYB color model.
	pub enum SecondaryColor {
			Orange,
			Green,
			Purple,
	}
}

pub mod utils {
	use crate::kinds::*;

	/// Combines two primary colors in equal amounts to create
	/// a secondary color.
	pub fn mix(c1: PrimaryColor, c2: PrimaryColor) {
			// --snip--
	}
}

use std::env;
use std::error::Error;
use std::fs;

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = minigrep::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
	x + 1
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename.clone())?;
	let lines = if config.case_sensitive {
		search(&config.query[..], &contents[..])
	} else {
		search_insensitive(&config.query[..], &contents[..])
	};
	println!("with text:\n{}", contents);
	for line in lines {
		println!("\nmatch: \n{}", line);
	}

	Ok(())
}

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		args.next();
		let query = match args.next() {
			Some(query) => query,
			None => return Err("Didn't get query string"),
		};
		let filename = match args.next() {
			Some(filename) => filename,
			None => return Err("Didn't get filename string"),
		};

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
		Ok(Config {
			query,
			filename,
			case_sensitive,
		})
	}
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	content.lines()
		.filter(|line| line.contains(query))
		.collect()
}

fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut ret = vec![];
	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
			ret.push(line)
		}
	}
	ret
}

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
			";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents))
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
			";

		assert_eq!(vec!["Rust:"], search_insensitive(query, contents))
	}
}
