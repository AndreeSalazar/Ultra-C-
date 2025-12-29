use ultracpp::{codegen, parser};

#[test]
fn inheritance_colon_header() {
    let c = parser::parse(
        r#"
class Hijo : Base:
  def get(self) -> Int:
    return 1
"#,
    );
    let h = codegen::header(&c);
    assert!(h.contains("class Hijo : public Base") || h.contains("class UCPP_API Hijo : public Base"));
}
