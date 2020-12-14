use itertools::Itertools;

// mod headings;
// mod mdx_ast;
// mod mdx_error;
pub mod ast;
pub use ast::{mdx_elements, MdxAst};

#[derive(Debug, PartialEq, Eq)]
pub struct Mdx<'a> {
    pub ast: Vec<MdxAst<'a>>,
}

pub fn parse(
    input: &str,
) -> Result<Mdx, nom_supreme::error::ErrorTree<nom_supreme::final_parser::Location>> {
    mdx_elements(input).map(|ast| Mdx { ast })
}

// TODO: there's probably a trait we can do for this?
// maybe Display somehow?
pub fn stringify(m: Mdx) -> String {
    m.ast
        .iter()
        .map(|ast| format!("{}", ast))
        .intersperse("\n\n".to_string())
        .collect::<String>()
}
