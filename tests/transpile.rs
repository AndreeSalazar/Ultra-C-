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

#[test]
fn filecall_hola_upp_prints_hello() {
    let src = r#"
class Main:
  def run():
    hola.upp()
"#;
    let c = parser::parse(src);
    let s = codegen::source(&c);
    assert!(s.contains("std::cout << \"Hola mundo\" << std::endl;"));
}

#[test]
fn filecall_other_upp_prints_run_message() {
    let src = r#"
class Main:
  def run():
    util.upp()
"#;
    let c = parser::parse(src);
    let s = codegen::source(&c);
    assert!(s.contains("std::cout << \"Run util.upp\" << std::endl;"));
}
