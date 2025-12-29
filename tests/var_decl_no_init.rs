use ultracpp::{parser, codegen};

#[test]
fn var_decl_no_init_in_method() {
    let c = parser::parse(r#"
class Vars:
  def mk(self):
    x: Int
    y: Float
"#);
    let s = codegen::source(&c);
    assert!(s.contains("int x;"));
    assert!(s.contains("float y;"));
}

