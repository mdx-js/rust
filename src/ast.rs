use nom::{character::complete::*, multi::many1_count, IResult};
use nom_supreme::{
    error::ErrorTree,
    final_parser::{final_parser, Location},
};
use std::fmt;

pub mod headings;
pub mod thematic_breaks;

pub use headings::{atx_heading, ATXHeading};
pub use thematic_breaks::{thematic_break, ThematicBreak};

// TODO: maybe get rid of Copy/Clone here?
// it's required by fold_many0
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MdxAst<'a> {
    ATXHeading(ATXHeading<'a>),
    ThematicBreak(ThematicBreak),
}
impl<'a> fmt::Display for MdxAst<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MdxAst::ATXHeading(atx @ ATXHeading { .. }) => write!(f, "{}", atx),
            MdxAst::ThematicBreak(brk @ ThematicBreak { .. }) => write!(f, "{}", brk),
        }
        // Use `self.number` to refer to each positional data point.
    }
}

pub fn mdx_elements(input: &str) -> Result<Vec<MdxAst>, ErrorTree<Location>> {
    final_parser(mdx_elements_internal)(input)
}
fn mdx_elements_internal(input: &str) -> IResult<&str, Vec<MdxAst>, ErrorTree<&str>> {
    let (input, _) = multispace0(input)?;
    let (input, result) = nom::multi::separated_list1(many1_count(newline), mdx_ast)(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = nom::combinator::eof(input)?;
    Ok((input, result))
}

fn mdx_ast(input: &str) -> IResult<&str, MdxAst, ErrorTree<&str>> {
    nom::branch::alt((ast_atx_heading, ast_thematic_break))(input)
}

/// We have to wrap the structs to fit in the MdxAst
fn ast_atx_heading(input: &str) -> IResult<&str, MdxAst, ErrorTree<&str>> {
    let (input, atx) = atx_heading(input)?;
    Ok((input, MdxAst::ATXHeading(atx)))
}

fn ast_thematic_break(input: &str) -> IResult<&str, MdxAst, ErrorTree<&str>> {
    let (input, thematic_break) = thematic_break(input)?;
    Ok((input, MdxAst::ThematicBreak(thematic_break)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_heading() {
        assert_eq!(
            mdx_ast("# boop").unwrap(),
            (
                "",
                MdxAst::ATXHeading(ATXHeading {
                    level: 1,
                    value: "boop"
                }),
            )
        );
    }
    #[test]
    fn parse_thematic_break() {
        assert_eq!(
            mdx_ast("---").unwrap(),
            (
                "",
                MdxAst::ThematicBreak(ThematicBreak {
                    char_count: 3,
                    break_char: '-'
                }),
            )
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
            )
            .unwrap(),
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
        );
    }
}
