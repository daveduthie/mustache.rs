use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, space0};
use nom::combinator::not;
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, terminated};

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum MustacheToken<'a> {
    Text(&'a str),
    Lookup(Vec<&'a str>),
}

type Result<'a> = IResult<&'a str, MustacheToken<'a>>;

fn lookup(input: &str) -> Result {
    let (rest, idents) = delimited(
        terminated(tag("{{"), space0),
        separated_list1(tag("."), alpha1),
        terminated(space0, tag("}}")),
    )(input)?;
    Ok((rest, MustacheToken::Lookup(idents)))
}

fn not_lookup(input: &str) -> Result {
    let (rest, text) = take_until("{{")(input)?;
    Ok((rest, MustacheToken::Text(text)))
}

fn mustache_token(input: &str) -> Result {
    let (rest, tok) = alt((lookup, not_lookup))(input)?;
    Ok((rest, tok))
}

mod tests {
    use super::*;

    #[test]
    fn not_lookup_test() {
        for (template, token) in vec![
            ("abc", MustacheToken::Text("abc")),
        ] {
            assert_eq!(Ok(("", token)), not_lookup(template));
        }
    }

    #[test]
    fn lookup_test() {
        for (template, token) in vec![
            ("{{x}}", MustacheToken::Lookup(vec!["x"])),
            ("{{x.y}}", MustacheToken::Lookup(vec!["x", "y"])),
            ("{{ x}}", MustacheToken::Lookup(vec!["x"])),
            ("{{ x.y}}", MustacheToken::Lookup(vec!["x", "y"])),
            ("{{x }}", MustacheToken::Lookup(vec!["x"])),
            ("{{x.y }}", MustacheToken::Lookup(vec!["x", "y"])),
            ("{{ x }}", MustacheToken::Lookup(vec!["x"])),
            ("{{ x.y }}", MustacheToken::Lookup(vec!["x", "y"])),
        ] {
            assert_eq!(Ok(("", token)), lookup(template));
        }
    }
}
