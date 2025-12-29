use ultracpp::{codegen, parser};

#[test]
fn inheritance_header() {
    let src = r#"
class Hijo(Base):
    otro: Int
    def get(self) -> Int:
        return self.otro
"#;
    let c = parser::parse(src);
    let h = codegen::header(&c);
    assert!(h.contains("class Hijo : public Base") || h.contains("class UCPP_API Hijo : public Base"));
}

#[test]
fn super_call_in_source() {
    let src = r#"
class Hijo(Base):
    def saludo(self) -> String:
        return super().saludo()
"#;
    let c = parser::parse(src);
    let s = codegen::source(&c);
    assert!(s.contains("return Base::saludo();"));
}
