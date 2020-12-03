use nom::IResult;

// mod headings;
// mod mdx_ast;
// mod mdx_error;
pub mod ast;
pub use ast::{mdx_elements, MdxAst};

#[derive(Debug, PartialEq, Eq)]
pub struct Mdx<'a> {
    pub ast: Vec<MdxAst<'a>>,
}

fn mdx(input: &str) -> IResult<&str, Mdx> {
    let (input, ast) = mdx_elements(input)?;
    Ok((input, Mdx { ast }))
}

// TODO: This function should not panic
pub fn parse<'a>(input: &'a str) -> Result<Mdx<'a>, std::io::Error> {
    let result = mdx(input);
    match result {
        Ok(("", m @ Mdx { .. })) => Ok(m),
        Ok((i, _m)) => panic!("leftover input {}", i),
        _ => panic!("something went wrong. TODO error message"),
    }
}

// TODO: there's probably a trait we can do for this?
// maybe Display somehow?
pub fn stringify(m: Mdx) -> String {
    m.ast
        .iter()
        .map(|ast| format!("{}", ast))
        .collect::<String>()
}
