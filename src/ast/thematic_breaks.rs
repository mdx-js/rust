use nom::{branch::alt, character::complete::space0, multi::fold_many_m_n, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag};
use std::fmt;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ThematicBreak {
    pub char_count: u8,
    pub break_char: char,
}
impl fmt::Display for ThematicBreak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(
            f,
            "{}",
            self.break_char.to_string().repeat(self.char_count.into())
        )
    }
}

fn optionally_surrounded_by_spaces<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    nom::sequence::delimited(space0, inner, space0)
}

pub fn thematic_break(input: &str) -> IResult<&str, ThematicBreak, ErrorTree<&str>> {
    // basically anything can start with 0-3 spaces. We don't really care.
    let (input, _) = fold_many_m_n(0, 3, tag(" "), 0, |acc: u8, _| acc + 1)(input)?;
    // any more spaces and this would be a code block
    // we need a character to match on because the rest of the thematic
    // break characters need to be the same char.
    let (input, c) = alt((
        nom::character::complete::char('-'),
        nom::character::complete::char('*'),
        nom::character::complete::char('_'),
    ))(input)?;
    // try to parse at least two more chars of the same type.
    // there can be any number of whitespace surrounding them, including on the end
    // up to the line break
    let (input, num_break_chars) = fold_many_m_n(
        2,
        1000,
        optionally_surrounded_by_spaces(nom::character::complete::char(c)),
        0,
        |acc: u8, _| acc + 1,
    )(input)?;

    Ok((
        input,
        ThematicBreak {
            char_count: num_break_chars + 1,
            break_char: c,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn parse_thematic_break_dash() {
        assert_eq!(
            thematic_break("---").unwrap(),
            (
                "",
                ThematicBreak {
                    char_count: 3,
                    break_char: '-'
                }
            )
        );
    }

    #[test]
    fn parse_thematic_break_star() {
        assert_eq!(
            thematic_break("***").unwrap(),
            (
                "",
                ThematicBreak {
                    char_count: 3,
                    break_char: '*'
                }
            )
        );
    }

    #[test]
    fn parse_thematic_break_underscore() {
        assert_eq!(
            thematic_break("___").unwrap(),
            (
                "",
                ThematicBreak {
                    char_count: 3,
                    break_char: '_'
                }
            )
        );
    }
    #[test]
    fn parse_thematic_break_spaces() {
        assert_eq!(
            thematic_break("  -   -    -  ").unwrap(),
            (
                "",
                ThematicBreak {
                    char_count: 3,
                    break_char: '-'
                }
            )
        );
    }
    // #[test]
    // fn parse_thematic_break_fail_extra_chars() {
    //     assert_eq!(
    //         thematic_break("--s-"),

    //             Err(Error(nom::error::Error { input: "-s-", code: ErrorKind::ManyMN }))

    //     );
    // }
    // #[test]
    // // #hashtags are not valid headings, and
    // // instad parse as paragraphs.
    // fn parse_fail_other() {
    //     assert_eq!(
    //         thematic_break("####### boop"),

    //         Err(Error(nom::error::Error { input: "####### boop", code: ErrorKind::Char }))
    //     );
    // }
}
