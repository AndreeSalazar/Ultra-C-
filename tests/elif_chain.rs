use ultracpp::{codegen, parser};

#[test]
fn elif_chain_emits_nested_if_else() {
    let c = parser::parse(
        r#"
class Elifs:
  def f(self) -> String:
    if  x == 1:
      return "A"
    elif x == 2:
      return "B"
    else:
      return "C"
"#,
    );
    let s = codegen::source(&c);
    assert!(s.contains("if ("));
    assert!(s.contains("else {"));
}
