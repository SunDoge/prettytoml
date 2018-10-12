use super::super::tokens::Token;

/// An immutable subset of a token sequence
pub struct TokenStream<'a> {
    head_index: usize,
    tokens: &'a Vec<Token>,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &Vec<Token>) -> TokenStream {
        TokenStream {
            head_index: 0,
            tokens,
        }
    }

    pub fn head(&self) -> Option<&Token> {
        self.tokens.get(self.head_index)
    }

    pub fn tail(&self) -> TokenStream {
        TokenStream {
            tokens: self.tokens,
            head_index: self.head_index + 1,
        }
    }

    pub fn offset(&self) -> usize {
        self.head_index
    }

    pub fn at_end(&self) -> bool {
        self.offset() >= self.tokens.len()
    }

    pub fn len(&self) -> usize {
        self.tokens.len() - self.offset()
    }
}
