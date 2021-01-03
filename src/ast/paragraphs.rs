use nom::{
    branch::alt,
    bytes::complete::*,
    character::complete::{anychar, space0},
    combinator::value,
    multi::fold_many_m_n,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, parser_ext::ParserExt, tag::complete::tag};
use std::fmt;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Paragraph<'a> {
    pub words: &'a str,
}
impl<'a> fmt::Display for Paragraph<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.words.lines() {
            write!(f, "{}", line)?
        }
        Ok(())
    }
}

/// Paragraphs can end at \n\n or eof
///
/// TODO: they can also be interrupted by lists without
/// a second newline
///
/// ```md
/// things
/// - list item
/// ```
pub fn paragraph(input: &str) -> IResult<&str, Paragraph, ErrorTree<&str>> {
    let result: IResult<&str, &str, ErrorTree<&str>> = take_until("\n\n")(input);
    match result {
        Ok((input, para)) => Ok((input, Paragraph { words: para })),
        Err(e) => {
            if input == "" {
                Err(e)
            } else {
                Ok(("", Paragraph { words: input }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn parse_paragraph_dash() {
        assert_eq!(
            paragraph("---\n\n").unwrap(),
            ("\n\n", Paragraph { words: "---" })
        );
    }

    #[test]
    fn parse_paragraph_multiline() {
        assert_eq!(
            paragraph("a line\nanotherline\nyetanotherline\n\n").unwrap(),
            (
                "\n\n",
                Paragraph {
                    words: "a line\nanotherline\nyetanotherline"
                }
            )
        );
    }
}
