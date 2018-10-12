#[macro_use]
extern crate lazy_static;
extern crate regex;

mod elements;
mod lexer;
mod parser;
mod tokens;

use self::lexer::tokenize;
use self::parser::parse_tokens;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub fn prettify(toml_text: &str) -> String {
    let tokens = tokenize(toml_text, true).expect("Fail to tokenize.");

    println!("tokens: {:#?}", tokens);

    let _elements = parse_tokens(tokens);
    "".to_string()
}

pub fn prettify_from_file<P: AsRef<Path>>(file_path: P) -> Result<String, io::Error> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(prettify(&contents))
}
