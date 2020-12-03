use nom::{bytes::complete::tag, multi::fold_many_m_n, IResult};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ATXHeading<'a> {
    pub level: u8,
    pub value: &'a str,
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
pub fn atx_heading(input: &str) -> IResult<&str, ATXHeading> {
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
            atx_heading("# boop"),
            Ok((
                "",
                ATXHeading {
                    level: 1,
                    value: "boop"
                }
            ))
        );
    }
    #[test]
    fn parse_atx_heading_level_2() {
        assert_eq!(
            atx_heading("## boop"),
            Ok((
                "",
                ATXHeading {
                    level: 2,
                    value: "boop"
                }
            ))
        );
    }
    #[test]
    fn parse_atx_heading_level_3() {
        assert_eq!(
            atx_heading("### boop"),
            Ok((
                "",
                ATXHeading {
                    level: 3,
                    value: "boop"
                }
            ))
        );
    }
    #[test]
    fn parse_atx_heading_level_4() {
        assert_eq!(
            atx_heading("#### boop"),
            Ok((
                "",
                ATXHeading {
                    level: 4,
                    value: "boop"
                }
            ))
        );
    }
    #[test]
    fn parse_atx_heading_level_5() {
        assert_eq!(
            atx_heading("##### boop"),
            Ok((
                "",
                ATXHeading {
                    level: 5,
                    value: "boop"
                }
            ))
        );
    }
    #[test]
    fn parse_atx_heading_level_6() {
        assert_eq!(
            atx_heading("###### boop"),
            Ok((
                "",
                ATXHeading {
                    level: 6,
                    value: "boop"
                }
            ))
        );
    }
    #[test]
    // headings can be empty
    fn parse_atx_heading_empty() {
        assert_eq!(
            atx_heading("######   "),
            Ok((
                "",
                ATXHeading {
                    level: 6,
                    value: ""
                }
            ))
        );
    }

    #[test]
    // a heading may be preceeded or followed by
    // any number of spaces
    fn parse_atx_heading_spaces() {
        assert_eq!(
            atx_heading("#     boop    "),
            Ok((
                "",
                ATXHeading {
                    level: 1,
                    value: "boop"
                }
            ))
        );
    }

    #[test]
    // a heading may be preceeded or followed by
    // any number of spaces
    fn parse_atx_heading_symbols() {
        assert_eq!(
            atx_heading("# a bunch-of valid (symbols), like:+  "),
            Ok((
                "",
                ATXHeading {
                    level: 1,
                    value: "a bunch-of valid (symbols), like:+"
                }
            ))
        );
    }
    #[test]
    // #hashtags are not valid headings, and
    // instad parse as paragraphs.
    fn parse_fail_hashtags() {
        assert_eq!(
            atx_heading("#boop"),
            Err(Error(nom::error::Error { input: "boop", code: ErrorKind::MultiSpace }))
        );
    }
    #[test]
    // #hashtags are not valid headings, and
    // instad parse as paragraphs.
    fn parse_fail_7_hashes() {
        assert_eq!(
            atx_heading("####### boop"),
            Err(Error(nom::error::Error { input: "# boop", code: ErrorKind::MultiSpace }))

        );
    }
}
