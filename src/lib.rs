#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod lexer;
pub mod tokens;

use std::path::Path;

pub fn prettify(toml_text: &str) -> String {
    let tokens = lexer::tokenize(toml_text, true);
    println!("tokens: {:#?}", tokens.unwrap_or(vec![]));
    "".to_string()
}

pub fn prettify_from_file<P: AsRef<Path>>(file_path: P) -> String {
    "".to_string()
}
