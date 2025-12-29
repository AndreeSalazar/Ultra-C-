use ultracpp::{codegen, parser};

#[test]
fn ops_keywords_and_or_not() {
    let c = parser::parse(
        r#"
class Ops:
  def eval(self) -> Bool:
    return not (true and false) or (1 < 2)
"#,
    );
    let s = codegen::source(&c);
    assert!(s.contains("return (!(true && false) || (1 < 2));"));
}
