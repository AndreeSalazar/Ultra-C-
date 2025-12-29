use ultracpp::{parser, codegen};

#[test]
fn native_triple_quotes_in_source() {
    let c = parser::parse(r#"
class Nativo:
  def print(self):
    native """
    std::cout << "A" << std::endl;
    std::cout << "B" << std::endl;
    """
"#);
    let s = codegen::source(&c);
    assert!(s.contains("std::cout << \"A\" << std::endl;"));
    assert!(s.contains("std::cout << \"B\" << std::endl;"));
}

