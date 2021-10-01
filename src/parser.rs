use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, space0};
use nom::combinator::fail;
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, terminated};
use nom::IResult;

use crate::tokens::{MustacheToken, Tokens};

type Result<'a> = IResult<&'a str, MustacheToken>;
type TokenizeResult<'a> = IResult<&'a str, Tokens>;

fn mlookup(identifiers: &[&str]) -> MustacheToken {
    MustacheToken::Lookup(identifiers.iter().map(|s| String::from(*s)).collect())
}

fn lookup(input: &str) -> Result {
    let (rest, idents) = delimited(
        terminated(tag("{{"), space0),
        separated_list1(tag("."), alpha1),
        terminated(space0, tag("}}")),
    )(input)?;
    Ok((rest, mlookup(&idents)))
}

fn mtext(text: &str) -> MustacheToken {
    MustacheToken::Text(String::from(text))
}

fn until_lookup(input: &str) -> Result {
    match take_until("{{")(input) {
        Ok((rest, text)) => Ok((rest, mtext(text))),
        Err(e) => Err(e),
    }
}

fn rest_of_text(input: &str) -> Result {
    match input {
        "" => fail(input),
        _ => Ok(("", mtext(input))),
    }
}

fn single_token(input: &str) -> Result {
    Ok(alt((lookup, until_lookup, rest_of_text))(input)?)
}

pub fn tokenize(input: &str) -> TokenizeResult {
    let (rest, tokens) = many0(single_token)(input)?;
    Ok((rest, tokens))
}

mod tests {
    use super::*;

    #[test]
    fn lookup_test() {
        for (template, token) in vec![
            ("{{x}}", mlookup(&vec!["x"])),
            ("{{x.y}}", mlookup(&vec!["x", "y"])),
            ("{{ x}}", mlookup(&vec!["x"])),
            ("{{ x.y}}", mlookup(&vec!["x", "y"])),
            ("{{x }}", mlookup(&vec!["x"])),
            ("{{x.y }}", mlookup(&vec!["x", "y"])),
            ("{{ x }}", mlookup(&vec!["x"])),
            ("{{ x.y }}", mlookup(&vec!["x", "y"])),
        ] {
            assert_eq!(Ok(("", token)), lookup(template));
        }
    }

    #[test]
    fn single_token_test() {
        for (template, token, rest) in vec![
            ("abc", mtext("abc"), ""),
            ("abc{{x}}", mtext("abc"), "{{x}}"),
            ("{{x}}", mlookup(&vec!["x"]), ""),
        ] {
            assert_eq!(Ok((rest, token)), single_token(template));
        }
    }

    #[test]
    fn tokenize_test() {
        for (template, tokens) in vec![
            ("{{x}}", vec![mlookup(&vec!["x"])]),
            ("abc", vec![mtext("abc")]),
            (
                "abc {{ x.y.z }} def",
                vec![mtext("abc "), mlookup(&vec!["x", "y", "z"]), mtext(" def")],
            ),
        ] {
            assert_eq!(Ok(("", tokens)), tokenize(template));
        }
    }
}
