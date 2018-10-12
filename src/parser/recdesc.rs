use super::tokenstream::TokenStream;

pub struct Capture<'a> {
    token_stream: TokenStream<'a>,
}

impl<'a> Capture<'a> {
    pub fn new(token_stream: TokenStream) -> Capture {
        Capture { token_stream }
    }

    pub fn pending_tokens(&self) -> &TokenStream {
        &self.token_stream
    }
}
