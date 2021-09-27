
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MustacheToken {
    Text(String),
    Lookup(Vec<String>),
}

pub type Tokens = Vec<MustacheToken>;