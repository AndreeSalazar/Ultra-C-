use ultracpp::{parser, codegen};

#[test]
fn static_method_and_bool_int_literals() {
    let src = r#"
class Version:
    def version() -> String:
        return "1.0.0"
class Pruebas:
    activo: Bool
    def estado(self) -> Bool:
        return true
    def numero(self) -> Int:
        return 42
"#;
    let c1 = parser::parse(src);
    let h1 = codegen::header(&c1);
    assert!(h1.contains("static std::string version();"));
    let c2 = parser::parse(r#"
class Nums:
    def f(self) -> Int:
        return 1 + 2
"#);
    let s2 = codegen::source(&c2);
    assert!(s2.contains("return 1 + 2;"));
}

#[test]
fn self_method_call() {
    let c = parser::parse(r#"
class Persona:
    def saludo(self) -> String:
        return "Hola"
    def saludo2(self) -> String:
        return self.saludo()
"#);
    let s = codegen::source(&c);
    assert!(s.contains("return this->saludo();"));
}
