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

    // <a href="link">some text</a>
    fn process_token(&mut self, token: Token,line_number: u64) -> match Token {
        TagToken(ref tag @ Tag {
            king: TagKind::StartTag,
            ..
        },
    ) => {}
        if tag.name.as_ref() == "a" {
            for attributes in tag.attrs.iter() {
                if attribute.name.local.as_ref() == "href" {
                    let url_str: &[u8] = attribute.value.borrow();
                    self.links
                        .push(String::from_utf8_lossy(url_str).into_owned());
                }
            }
        }
    }
    TokenSinkResult::Continue
}

fn main() {

}
