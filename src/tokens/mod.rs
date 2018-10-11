use std::cmp::Ordering;

pub const TYPE_BOOLEAN: TokenType = TokenType {
    priority: 0,
    name: "boolean",
    is_metadata: false,
};

#[derive(Debug)]
pub struct TokenType {
    pub priority: u8,
    name: &'static str,
    is_metadata: bool,
}

// impl TokenType {
//     pub fn new(name: &'static str, priority: u8, is_metadata: bool) -> TokenType {
//         TokenType {
//             name,
//             priority,
//             is_metadata,
//         }
//     }
// }

impl PartialOrd for TokenType {
    fn partial_cmp(&self, other: &TokenType) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        self.priority == other.priority
    }
}

#[derive(Debug)]
pub struct Token {
    pub source_substring: String,
    pub token_type: &'static TokenType,
    pub col: usize,
    pub row: usize,
}

impl Token {
    pub fn new(
        token_type: &'static TokenType,
        source_substring: &str,
        col: usize,
        row: usize,
    ) -> Token {
        Token {
            token_type,
            source_substring: source_substring.to_string(),
            col,
            row,
        }
    }
}
