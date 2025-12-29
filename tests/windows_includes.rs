use ultracpp::{parser, codegen};

#[test]
fn windows_headers_present_when_needed() {
    let c = parser::parse(r#"
class WinUse:
  def tick(self):
    native """
    #ifdef _WIN32
    if (_kbhit()) { Sleep(1); }
    #endif
    """
"#);
    let s = codegen::source(&c);
    assert!(s.contains("#include <conio.h>"));
    assert!(s.contains("#include <windows.h>"));
}

#[test]
fn windows_headers_absent_when_not_needed() {
    let c = parser::parse(r#"
class NoWin:
  def say(self):
    native "std::cout << 123 << std::endl;"
"#);
    let s = codegen::source(&c);
    assert!(!s.contains("#include <windows.h>"));
}

