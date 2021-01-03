use mdx::{ast::MdxAst, *};
use nom_supreme::error::{BaseErrorKind, ErrorTree};

#[test]
fn test_parse() {
    assert_eq!(
        parse(
            "# boop

---

## boop 2"
        )
        .unwrap(),
        Mdx {
            ast: vec![
                ast::MdxAst::ATXHeading(ast::ATXHeading {
                    level: 1,
                    value: "boop"
                }),
                ast::MdxAst::ThematicBreak(ast::ThematicBreak {
                    char_count: 3,
                    break_char: '-'
                }),
                ast::MdxAst::ATXHeading(ast::ATXHeading {
                    level: 2,
                    value: "boop 2"
                }),
            ]
        }
    );
}

#[test]
fn test_parse_fail_thematic_break_into_paragraph() {
    assert_eq!(
        parse("# boop\n\n-d--\n\n## boop 2\n\n",).unwrap(),
        Mdx {
            ast: vec![
                MdxAst::ATXHeading(ast::ATXHeading {
                    level: 1,
                    value: "boop"
                }),
                MdxAst::Paragraph(ast::Paragraph { words: "-d--" }),
                MdxAst::ATXHeading(ast::ATXHeading {
                    level: 2,
                    value: "boop 2"
                })
            ]
        }
    );
}

#[test]
#[should_panic]
// should panic beause we haven't implemented everything yet
fn test_parse_panic() {
    assert_eq!(
        parse(
            "# boop

something else

```js
const some = {}
```"
        )
        .unwrap(),
        Mdx {
            ast: vec![ast::MdxAst::ATXHeading(ast::ATXHeading {
                level: 1,
                value: "boop"
            })]
        }
    );
}

#[test]
#[test]
fn round_trip() {
    assert_eq!(
        parse(
            "
            
# boop        
"
        )
        .map(|ast| stringify(ast))
        .unwrap(),
        "# boop"
    );
}
