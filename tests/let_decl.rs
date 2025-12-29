use ultracpp::{codegen, parser};

#[test]
fn let_and_colon_decl() {
    let c = parser::parse(
        r#"
class Decl:
  def make(self) -> Int:
    let a: Int = 1
    let b: Float
    c: String
    return a
"#,
    );
    let s = codegen::source(&c);
    assert!(s.contains("int a = 1;"));
    assert!(s.contains("float b;"));
    assert!(s.contains("std::string c;"));
}
