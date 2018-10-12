mod parser;
mod tokenstream;
mod recdesc;

use self::tokenstream::TokenStream;
use super::tokens::Token;

pub fn parse_tokens(tokens: Vec<Token>) {
    parse_token_stream(TokenStream::new(&tokens));
}

fn parse_token_stream(token_stream: TokenStream) {}
