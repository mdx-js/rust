use nom::{multi::fold_many_m_n, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ATXHeading<'a> {
    pub level: u8,
    pub value: &'a str,
}
impl<'a> fmt::Display for ATXHeading<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{} {}", "#".repeat(self.level.into()), self.value)
    }
}

fn inner_heading<'a, F: 'a, O, E: nom::error::ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    nom::sequence::delimited(
        nom::character::complete::space0,
        inner,
        nom::character::complete::space0,
    )
}
pub fn atx_heading(input: &str) -> IResult<&str, ATXHeading, ErrorTree<&str>> {
    let (input, _) = fold_many_m_n(0, 3, tag(" "), 0, |acc: u8, _| acc + 1)(input)?;
    let (input, num_hashes) = nom::sequence::terminated(
        fold_many_m_n(0, 6, tag("#"), 0, |acc: u8, _| acc + 1),
        nom::character::complete::multispace1,
    )(input)?;

    // empty headings are a thing, so any parsing below this is optional
    let (input, val) = inner_heading(nom::character::complete::not_line_ending)(input)?;
    Ok((
        input,
        ATXHeading {
            level: num_hashes,
            value: val.trim(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn parse_atx_heading_level_1() {
        assert_eq!(
            atx_heading("# boop").unwrap(),
            (
                "",
                ATXHeading {
                    level: 1,
                    value: "boop"
                }
            )
        );
    }
    #[test]
    fn parse_atx_heading_level_2() {
        assert_eq!(
            atx_heading("## boop").unwrap(),
            (
                "",
                ATXHeading {
                    level: 2,
                    value: "boop"
                }
            )
        );
    }
    #[test]
    fn parse_atx_heading_level_3() {
        assert_eq!(
            atx_heading("### boop").unwrap(),
            (
                "",
                ATXHeading {
                    level: 3,
                    value: "boop"
                }
            )
        );
    }
    #[test]
    fn parse_atx_heading_level_4() {
        assert_eq!(
            atx_heading("#### boop").unwrap(),
            (
                "",
                ATXHeading {
                    level: 4,
                    value: "boop"
                }
            )
        );
    }
    #[test]
    fn parse_atx_heading_level_5() {
        assert_eq!(
            atx_heading("##### boop").unwrap(),
            (
                "",
                ATXHeading {
                    level: 5,
                    value: "boop"
                }
            )
        );
    }
    #[test]
    fn parse_atx_heading_level_6() {
        assert_eq!(
            atx_heading("###### boop").unwrap(),
            (
                "",
                ATXHeading {
                    level: 6,
                    value: "boop"
                }
            )
        );
    }
    #[test]
    // headings can be empty
    fn parse_atx_heading_empty() {
        assert_eq!(
            atx_heading("######   ").unwrap(),
            (
                "",
                ATXHeading {
                    level: 6,
                    value: ""
                }
            )
        );
    }

    #[test]
    // a heading may be preceeded or followed by
    // any number of spaces
    fn parse_atx_heading_spaces() {
        assert_eq!(
            atx_heading("#     boop    ").unwrap(),
            (
                "",
                ATXHeading {
                    level: 1,
                    value: "boop"
                }
            )
        );
    }

    #[test]
    // a heading may be preceeded or followed by
    // any number of spaces
    fn parse_atx_heading_symbols() {
        assert_eq!(
            atx_heading("# a bunch-of valid (symbols), like:+  ").unwrap(),
            (
                "",
                ATXHeading {
                    level: 1,
                    value: "a bunch-of valid (symbols), like:+"
                }
            )
        );
    }
    // #[test]
    // // #hashtags are not valid headings, and
    // // instad parse as paragraphs.
    // fn parse_fail_hashtags() {
    //     assert_eq!(
    //         atx_heading("#boop").unwrap_err(),
    //         ErrorTree(nom::error::Error {
    //             input: "boop",
    //             code: ErrorKind::MultiSpace
    //         })
    //     );
    // }
    // #[test]
    // // #hashtags are not valid headings, and
    // // instad parse as paragraphs.
    // fn parse_fail_7_hashes() {
    //     assert_eq!(
    //         atx_heading("####### boop").unwrap_err(),
    //         BaseErrorKind::Kind(ErrorKind::MultiSpace)
    //     );
    // }
}
