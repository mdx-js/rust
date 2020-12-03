use mdx::*;

#[test]
fn test_parse() {
    assert_eq!(
        parse("# boop").unwrap(),
        Mdx {
            ast: vec![ast::MdxAst::ATXHeading(ast::ATXHeading {
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
