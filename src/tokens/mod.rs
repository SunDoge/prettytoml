use std::cmp::Ordering;

#[derive(Debug)]
pub enum TokenTypeName {
    Boolean,
    Integer,
    Comma,
    SquareLeftBracket,
    SquareRightBracket,
    CurlyLeftBracket,
    CurlyRightBracket,
    Assignment,
    DoubleSquareLeftBracket,
    DoubleSquareRightBracket,
    Float,
    Date,
    Dot,
    BareString,
    String,
    MultilineString,
    LiteralString,
    MultilineLiteralString,
    Newline,
    Whitespace,
    Comment,
}

pub const TYPE_BOOLEAN: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::Boolean,
    is_metadata: false,
};

pub const TYPE_INTEGER: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::Integer,
    is_metadata: false,
};

pub const TYPE_OP_COMMA: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::Comma,
    is_metadata: true,
};

pub const TYPE_OP_SQUARE_LEFT_BRACKET: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::SquareLeftBracket,
    is_metadata: true,
};

pub const TYPE_OP_SQUARE_RIGHT_BRACKET: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::SquareRightBracket,
    is_metadata: true,
};

pub const TYPE_OP_CURLY_LEFT_BRACKET: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::CurlyLeftBracket,
    is_metadata: true,
};

pub const TYPE_OP_CURLY_RIGHT_BRACKET: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::CurlyRightBracket,
    is_metadata: true,
};

pub const TYPE_OP_ASSIGNMENT: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::Assignment,
    is_metadata: true,
};

pub const TYPE_OP_DOUBLE_SQUARE_LEFT_BRACKET: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::DoubleSquareLeftBracket,
    is_metadata: true,
};

pub const TYPE_OP_DOUBLE_SQUARE_RIGHT_BRACKET: TokenType = TokenType {
    priority: 0,
    name: TokenTypeName::DoubleSquareRightBracket,
    is_metadata: true,
};

pub const TYPE_FLOAT: TokenType = TokenType {
    priority: 1,
    name: TokenTypeName::Float,
    is_metadata: false,
};

pub const TYPE_DATE: TokenType = TokenType {
    priority: 40,
    name: TokenTypeName::Date,
    is_metadata: true,
};

pub const TYPE_OP_DOT: TokenType = TokenType {
    priority: 40,
    name: TokenTypeName::Dot,
    is_metadata: true,
};

pub const TYPE_BARE_STRING: TokenType = TokenType {
    priority: 50,
    name: TokenTypeName::BareString,
    is_metadata: false,
};

pub const TYPE_STRING: TokenType = TokenType {
    priority: 90,
    name: TokenTypeName::String,
    is_metadata: false,
};

pub const TYPE_MULTILINE_STRING: TokenType = TokenType {
    priority: 90,
    name: TokenTypeName::MultilineString,
    is_metadata: false,
};

pub const TYPE_LITERAL_STRING: TokenType = TokenType {
    priority: 90,
    name: TokenTypeName::LiteralString,
    is_metadata: false,
};

pub const TYPE_MULTILINE_LITERAL_STRING: TokenType = TokenType {
    priority: 90,
    name: TokenTypeName::MultilineLiteralString,
    is_metadata: false,
};

pub const TYPE_NEWLINE: TokenType = TokenType {
    priority: 91,
    name: TokenTypeName::Newline,
    is_metadata: true,
};
pub const TYPE_WHITESPACE: TokenType = TokenType {
    priority: 93,
    name: TokenTypeName::Whitespace,
    is_metadata: true,
};
pub const TYPE_COMMENT: TokenType = TokenType {
    priority: 95,
    name: TokenTypeName::Comment,
    is_metadata: true,
};

#[derive(Debug)]
pub struct TokenType {
    pub priority: u8,
    name: TokenTypeName,
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
