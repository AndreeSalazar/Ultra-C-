use ultracpp::{codegen, parser};

#[test]
fn init_constructor_params() {
    let src = r#"
class Persona:
    nombre: String
    def __init__(self, nombre: String):
        return ""
    def saludo(self) -> String:
        return "Hola " + self.nombre
"#;
    let c = parser::parse(src);
    let h = codegen::header(&c);
    let s = codegen::source(&c);
    assert!(h.contains("Persona(std::string nombre);"));
    assert!(s.contains("Persona::Persona(std::string nombre)"));
    assert!(s.contains("nombre(nombre)"));
}
