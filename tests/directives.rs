use ultracpp::{parser, resolve_includes};

#[test]
fn scan_std_profile_and_run() {
    let src = r#"
std
run Hola
Hola
  name String
  greet -> String
    "Hola " + name
"#;
    let d = parser::scan_directives(src);
    let incs = resolve_includes(&d);
    assert!(incs.contains(&"string".to_string()));
    assert!(incs.contains(&"memory".to_string()));
    assert!(incs.contains(&"iostream".to_string()));
}

#[test]
fn parse_all_alt_two_classes() {
    let src = r#"
std
Uno
  a Int
  get -> Int
    1
Dos
  b String
  get -> String
    "x"
"#;
    let classes = parser::parse_all(src);
    assert!(classes.len() >= 2);
    assert!(classes.iter().any(|c| c.name == "Uno"));
    assert!(classes.iter().any(|c| c.name == "Dos"));
}
