use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space0};
use nom::multi::separated_list1;
use nom::sequence::{delimited, terminated};
use nom::IResult;

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

mod tests {
    use super::*;

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
