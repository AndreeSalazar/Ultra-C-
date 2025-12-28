use ultracpp::{parser, codegen};

#[test]
fn transpile_basic_class() {
    let src = r#"
class Persona:
    nombre: String
    def saludo(self) -> String:
        return "Hola " + self.nombre
"#;
    let c = parser::parse(src);
    let h = codegen::header(&c);
    let s = codegen::source(&c);
    assert!(h.contains("class Persona"));
    assert!(h.contains("std::string nombre;"));
    assert!(h.contains("std::string saludo();"));
    assert!(s.contains("#include \"persona.hpp\""));
    assert!(s.contains("Persona::Persona("));
    assert!(s.contains("return std::string(\"Hola \") + this->nombre;"));
}
