#[test]
fn test_parse() {
    assert_eq!(
        mdx::parse("# boop").unwrap(),
        mdx::Mdx {
            ast: vec![mdx::ast::MdxAst::ATXHeading(mdx::ast::ATXHeading {
                level: 1,
                value: "boop"
            })]
        }
    );
}

#[test]
#[should_panic]
// should panic beause we haven't implemented everything yet
fn test_parse_panic() {
    assert_eq!(
        mdx::parse(
            "# boop

something else

```js
const some = {}
```"
        )
        .unwrap(),
        mdx::Mdx {
            ast: vec![mdx::ast::MdxAst::ATXHeading(mdx::ast::ATXHeading {
                level: 1,
                value: "boop"
            })]
        }
    );
}
