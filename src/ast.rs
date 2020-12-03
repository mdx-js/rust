use nom::IResult;

pub mod headings;

pub use headings::{atx_heading, ATXHeading};

// wrap everything in whitespace
// TODO: do we need to handle "one newline" vs "two newlines" worth
// of whitespace for paragraphs
fn wrapped<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    nom::sequence::delimited(
        nom::character::complete::multispace0,
        inner,
        nom::character::complete::multispace0,
    )
}

// TODO: maybe get rid of Copy/Clone here?
// it's required by fold_many0
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MdxAst<'a> {
    ATXHeading(ATXHeading<'a>),
}

pub fn mdx_elements(input: &str) -> IResult<&str, Vec<MdxAst>> {
    nom::multi::fold_many0(mdx_ast, Vec::new(), |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })(input)
}

fn mdx_ast(input: &str) -> IResult<&str, MdxAst> {
    let (input, ast) = wrapped(nom::branch::alt((ast_atx_heading, ast_atx_heading)))(input)?;
    Ok((input, ast))
}

/// We have to wrap the structs to fit in the MdxAst
fn ast_atx_heading(input: &str) -> IResult<&str, MdxAst> {
    let (input, atx) = atx_heading(input)?;
    Ok((input, MdxAst::ATXHeading(atx)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_heading() {
        assert_eq!(
            mdx_ast("# boop"),
            Ok((
                "",
                MdxAst::ATXHeading(ATXHeading {
                    level: 1,
                    value: "boop"
                }),
            ))
        );
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;

    #[test]
    // this input string is sloppy for a reason.
    // that reason is to ensure the parser handles sloppy
    // input when it comes to whitespace
    fn parse_headings() {
        assert_eq!(
            mdx_elements(
                "
# boop


## boop

"
            ),
            Ok((
                "",
                vec![
                    MdxAst::ATXHeading(ATXHeading {
                        level: 1,
                        value: "boop"
                    }),
                    MdxAst::ATXHeading(ATXHeading {
                        level: 2,
                        value: "boop"
                    }),
                ]
            ))
        );
    }
}
