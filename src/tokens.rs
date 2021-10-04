#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MustacheToken {
    Text(String),
    Lookup(Vec<String>),
}

pub fn new_lookup(identifiers: &[&str]) -> MustacheToken {
    MustacheToken::Lookup(identifiers.iter().map(|s| String::from(*s)).collect())
}

pub fn new_text(text: &str) -> MustacheToken {
    MustacheToken::Text(String::from(text))
}
pub type Tokens = Vec<MustacheToken>;
