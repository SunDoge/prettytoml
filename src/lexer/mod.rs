use super::tokens::*;
use regex::{Regex, RegexBuilder};
use std::error::Error;
use std::fmt;

lazy_static! {
    static ref LEXICAL_SPECS: Vec<(TokenType, Regex)> = vec![
        (TYPE_COMMENT, Regex::new(r"^(#.*)\n").unwrap()),
        (TYPE_STRING, Regex::new(r#"^("(([^"]|\\")+?[^\\]|([^"]|\\"))")"#).unwrap()), // Single line only, no |
        (TYPE_MULTILINE_STRING, RegexBuilder::new(r#"^(""".*?""")"#).dot_matches_new_line(true).build().unwrap()),
        (TYPE_LITERAL_STRING, Regex::new(r"^('.*?')").unwrap()),
        (TYPE_MULTILINE_LITERAL_STRING, RegexBuilder::new(r"^('''.*?''')").dot_matches_new_line(true).build().unwrap()),
        (TYPE_BARE_STRING, Regex::new(r"^([A-Za-z0-9_-]+)").unwrap()),
        (TYPE_DATE, Regex::new(r"^([0-9]{4}-[0-9]{2}-[0-9]{2}(T[0-9]{2}:[0-9]{2}:[0-9]{2}(\.[0-9]*)?)?(([zZ])|((\+|-)[0-9]{2}:[0-9]{2}))?)").unwrap()),
        (TYPE_WHITESPACE, RegexBuilder::new(r"^( |\t)").dot_matches_new_line(true).build().unwrap()),
        (TYPE_INTEGER, Regex::new(r"^(((\+|-)[0-9_]+)|([0-9][0-9_]*))").unwrap()),
        (TYPE_FLOAT, Regex::new(r"^((((\+|-)[0-9_]+)|([1-9][0-9_]*))(\.[0-9_]+)?([eE](\+|-)?[0-9_]+)?)").unwrap()),
        (TYPE_BOOLEAN, Regex::new(r"^(true|false)").unwrap()),

        (TYPE_OP_SQUARE_LEFT_BRACKET, Regex::new(r"^(\[)").unwrap()),
        (TYPE_OP_SQUARE_RIGHT_BRACKET, Regex::new(r"^(\])").unwrap()),

        (TYPE_OP_CURLY_LEFT_BRACKET, Regex::new(r"^(\{)").unwrap()),
        (TYPE_OP_CURLY_RIGHT_BRACKET, Regex::new(r"^(\})").unwrap()),

        (TYPE_OP_ASSIGNMENT, Regex::new(r"^(=)").unwrap()),
        (TYPE_OP_COMMA, Regex::new(r"^(,)").unwrap()),

        (TYPE_OP_DOUBLE_SQUARE_LEFT_BRACKET, Regex::new(r"^(\[\[)").unwrap()),
        (TYPE_OP_DOUBLE_SQUARE_RIGHT_BRACKET, Regex::new(r"^(\]\])").unwrap()),

        (TYPE_OP_DOT, Regex::new(r"^(\.)").unwrap()),
        (TYPE_NEWLINE, Regex::new(r"^(\n|\r\n)").unwrap()),
    ];
}

#[derive(Debug)]
pub struct LexerError(String);

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LexerError: {}", self.0.to_string())
    }
}

impl Error for LexerError {
    fn description(&self) -> &str {
        &self.0
    }
}

fn munch_a_token(source: &str) -> Option<Token> {
    let mut token = Token::new(&TYPE_BOOLEAN, "", 0, 0);
    let mut max_score = 0;
    for token_spec in &*LEXICAL_SPECS {
        let caps = token_spec.1.captures(source);
        if let Some(groups) = caps {
            let group1 = groups.get(1).unwrap().as_str();
            // Find the token with highest priority and longest source substring.
            if group1.len() + token_spec.0.priority as usize > max_score {
                token.source_substring = group1.to_string();
                token.token_type = &token_spec.0;
                max_score = group1.len() + token_spec.0.priority as usize;
            }
        }
    }
    if max_score == 0 {
        None
    } else {
        Some(token)
    }
}

pub fn tokenize(source: &str, is_top_level: bool) -> Result<Vec<Token>, LexerError> {
    let mut source = source.replace("\r\n", "\n");

    if is_top_level && !source.ends_with("\n") {
        source.push('\n');
    }

    let mut next_row = 1;
    let mut next_col = 1;
    let mut next_index = 0;

    let mut tokens: Vec<Token> = Vec::new();

    while next_index < source.len() {
        let new_token = munch_a_token(&source[next_index..]);
        if let Some(token) = new_token {
            next_index += token.source_substring.len();

            for c in token.source_substring.bytes() {
                if c == b'\n' {
                    next_row += 1;
                    next_col = 1;
                } else {
                    next_col += 1;
                }
            }
            tokens.push(token);
        } else {
            return Err(LexerError(format!(
                "failed to read the next token at ({}, {}): {}",
                next_row,
                next_col,
                &source[next_index..]
            )));
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {

    #[test]
    fn valid_tokenizing() {}
}
