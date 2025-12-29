use ultracpp::{codegen, parser};

#[test]
fn dotted_static_call_maps_to_double_colon() {
    let c = parser::parse(
        r#"
class Caller:
  def call(self) -> String:
    return Version.version()
"#,
    );
    let s = codegen::source(&c);
    assert!(s.contains("return Version::version();"));
}
