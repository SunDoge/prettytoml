use std::cmp::Ordering;

const TYPE_BOOLEAN: TokenType = TokenType {
    priority: 0,
    name: "boolean",
    is_metadata: false,
};

pub struct TokenType {
    priority: u8,
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
        unimplemented!()
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        self.priority == other.priority
    }
}
