#[macro_use]
extern crate lazy_static;
extern crate regex;

mod elements;
mod lexer;
mod parser;
mod tokens;

use self::lexer::tokenize;
use self::parser::parse_tokens;
use std::path::Path;

pub fn prettify(toml_text: &str) -> String {
    let tokens = tokenize(toml_text, true).expect("Fail to tokenize.");

    println!("tokens: {:#?}", tokens);

    let _elements = parse_tokens(tokens);
    "".to_string()
}

pub fn prettify_from_file<P: AsRef<Path>>(file_path: P) -> String {
    "".to_string()
}
