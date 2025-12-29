use ultracpp::{codegen, parser, Directives};

#[test]
fn test_filecall_expr_parsed() {
    let src = r#"
class Principal:
  def run():
    call hola.upp
"#;
    let classes = parser::parse_all(src);
    assert_eq!(classes.len(), 1);
    let c = &classes[0];
    let m = c.methods.iter().find(|m| m.name == "run").expect("run not found");
    match &m.body {
        ultracpp::Expr::Block(stmts) => {
            assert!(!stmts.is_empty(), "block empty");
            match &stmts[0] {
                ultracpp::Expr::FileCall(name) => {
                    assert_eq!(name, "hola_upp");
                }
                _ => panic!("expected FileCall, got {:?}", stmts[0]),
            }
        }
        _ => panic!("expected block body"),
    }
}

#[test]
fn test_auto_includes_var_decl() {
    let src = r#"
class Rect:
  width: Int
  height: Int

class Principal:
  def run():
    let r: Rect = Rect()
"#;
    let mut classes = parser::parse_all(src);
    // Attach directives to include profiles for standard libs if needed
    let d = Directives::default();
    for c in classes.iter_mut() {
        c.extra_includes = ultracpp::resolve_includes(&d);
    }
    let principal = classes.iter().find(|c| c.name == "Principal").expect("Principal not found");
    let src_cpp = codegen::source(principal);
    assert!(src_cpp.contains("#include \"rect.hpp\""), "rect.hpp not included:\n{}", src_cpp);
}

#[test]
fn test_dotted_static_auto_include() {
    let src = r#"
class Principal:
  def run():
    Utils.Version.current()
"#;
    let mut classes = parser::parse_all(src);
    let d = Directives::default();
    for c in classes.iter_mut() {
        c.extra_includes = ultracpp::resolve_includes(&d);
    }
    let principal = classes.iter().find(|c| c.name == "Principal").expect("Principal not found");
    let src_cpp = codegen::source(principal);
    assert!(src_cpp.contains("#include \"version.hpp\"") || src_cpp.contains("#include \"Utils.hpp\"") || src_cpp.contains("#include \"utils.hpp\""),
        "expected auto-include for dotted static, got:\n{}", src_cpp);
}
