use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::{char, space0},
    multi::fold_many_m_n,
    IResult,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};
use std::fmt;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct FencedCodeblock<'a> {
    pub language: &'a str,
    pub infostring: &'a str,
    pub code: &'a str,
}
impl fmt::Display for FencedCodeblock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "```
{}
```",
            self.code.to_string()
        )
    }
}

fn parse_infostring(
    input: &str,
) -> IResult<&str, &str, ErrorTree<&str>> {
    let (input, _) = space0(input)?;
    let (input, infostring) = take_until("\n")(input)?;
    let (input, _) =
        nom::character::complete::line_ending(input)?;
    Ok((input, infostring))
}

pub fn fenced_codeblock(
    input: &str,
) -> IResult<&str, FencedCodeblock, ErrorTree<&str>> {
    // basically anything can start with 0-3 spaces. We don't really care.
    let (input, _) =
        fold_many_m_n(0, 3, tag(" "), 0, |acc: u8, _| {
            acc + 1
        })(input)?;

    // codeblocks can be defined by ``` or ~~~, which
    // also have slightly different rules as to what's
    // allowable in the infostring
    let (input, c) = alt((char('`'), char('~')))(input)?;

    // try to parse at least two more chars of the same type.
    let (input, num_break_chars) = fold_many_m_n(
        2,
        1000,
        char(c),
        0,
        |acc: u8, _| acc + 1,
    )(input)?;
    let (input, infostring) = parse_infostring(input)?;
    // infostring parsing
    // end at newline
    //code vv
    // : IResult<&str, &str, ErrorTree<&str>>
    let (input, code) = take_until("```")(input)?;
    let (input, _) = tag("```")(input)?;
    Ok((
        input,
        FencedCodeblock {
            language: "",
            infostring,
            code: code,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err::Error};

    #[test]
    fn parse_fenced_codeblock() {
        assert_eq!(
            fenced_codeblock("```\n\n```").unwrap(),
            (
                "",
                FencedCodeblock {
                    language: "",
                    infostring: "",
                    code: "\n",
                }
            )
        );
    }
    #[test]
    fn parse_fenced_codeblock_infostring() {
        assert_eq!(
            fenced_codeblock(
                "```js title=something.txt
const t = {};
```"
            )
            .unwrap(),
            (
                "",
                FencedCodeblock {
                    language: "",
                    infostring: "js title=something.txt",
                    code: "const t = {};\n",
                }
            )
        );
    }

    #[test]
    fn parse_fenced_codeblock_infostring_with_spaces() {
        assert_eq!(
            fenced_codeblock(
                "```          js title=something.txt
const t = {};
```"
            )
            .unwrap(),
            (
                "",
                FencedCodeblock {
                    language: "",
                    infostring: "js title=something.txt",
                    code: "const t = {};\n",
                }
            )
        );
    }
}
