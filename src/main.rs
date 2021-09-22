use html5ever::tokenizer::{
    BufferQueue,
    Tag,
    TagKind,
    TagToken,
    Token,
    TokenSink,
    TokenSinkResult,
    Tokenizer,
    TokenizerOpts,
}:
use std::borrow::Borrow;
use url::{
    ParseError,
    Url
};

#[derive(Default, Debug)]
struct LinkQueue {
    links: Vec<String>,
}

impl TokenSink for &mut LinkQueue {
    type Handle = ();

    fn process_token(&mut self, token: Token,line_number: u64) -> match Token {
        TagToken(ref tag @ Tag {
            king: TagKind::StartTag,
            ..
        },
    ) => {}
    _ => {}
    }
    TokenSinkResult::Continue
}

fn main() {

}
