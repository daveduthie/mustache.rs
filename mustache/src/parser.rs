use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, space0};
use nom::combinator::fail;
use nom::IResult;
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, terminated};

use crate::tokens::{MustacheToken, new_lookup, new_text, Tokens};

type Result<'a> = IResult<&'a str, MustacheToken>;
type TokenizeResult<'a> = IResult<&'a str, Tokens>;

fn lookup(input: &str) -> Result {
    let (rest, idents) = delimited(
        terminated(tag("{{"), space0),
        separated_list1(tag("."), alpha1),
        terminated(space0, tag("}}")),
    )(input)?;
    Ok((rest, new_lookup(&idents)))
}

// TODO: this is quite a nasty solution, and I'll probably need to find a better one to add more of
// the mustache syntax
fn until_lookup(input: &str) -> Result {
    match take_until("{{")(input) {
        Ok((rest, text)) => Ok((rest, new_text(text))),
        Err(e) => Err(e),
    }
}

fn rest_of_text(input: &str) -> Result {
    match input {
        "" => fail(input),
        _ => Ok(("", new_text(input))),
    }
}

fn single_token(input: &str) -> Result {
    Ok(alt((lookup, until_lookup, rest_of_text))(input)?)
}

pub fn tokenize(input: &str) -> TokenizeResult {
    let (rest, tokens) = many0(single_token)(input)?;
    Ok((rest, tokens))
}

#[cfg(test)]
mod parser_tests {

    use crate::parser::{tokenize, single_token, lookup};
    use crate::tokens::{new_lookup, new_text};
    #[test]
    fn lookup_test() {
        for (template, token) in vec![
            ("{{x}}", new_lookup(&vec!["x"])),
            ("{{x.y}}", new_lookup(&vec!["x", "y"])),
            ("{{ x}}", new_lookup(&vec!["x"])),
            ("{{ x.y}}", new_lookup(&vec!["x", "y"])),
            ("{{x }}", new_lookup(&vec!["x"])),
            ("{{x.y }}", new_lookup(&vec!["x", "y"])),
            ("{{ x }}", new_lookup(&vec!["x"])),
            ("{{ x.y }}", new_lookup(&vec!["x", "y"])),
        ] {
            assert_eq!(Ok(("", token)), lookup(template));
        }
    }

    #[test]
    fn single_token_test() {
        for (template, token, rest) in vec![
            ("abc", new_text("abc"), ""),
            ("abc{{x}}", new_text("abc"), "{{x}}"),
            ("{{x}}", new_lookup(&vec!["x"]), ""),
        ] {
            assert_eq!(Ok((rest, token)), single_token(template));
        }
    }

    #[test]
    fn tokenize_test() {
        for (template, tokens) in vec![
            ("{{x}}", vec![new_lookup(&vec!["x"])]),
            ("abc", vec![new_text("abc")]),
            (
                "abc {{ x.y.z }} def",
                vec![
                    new_text("abc "),
                    new_lookup(&vec!["x", "y", "z"]),
                    new_text(" def"),
                ],
            ),
        ] {
            assert_eq!(Ok(("", tokens)), tokenize(template));
        }
    }
}
