use crate::{Class, Expr, Method};

fn cpp_type(t: &str) -> String {
    match t {
        "String" => "std::string".to_string(),
        "Void" => "void".to_string(),
        _ => t.to_string(),
    }
}

pub fn header(c: &Class) -> String {
    let mut h = String::new();
    h.push_str("#pragma once\n");
    h.push_str("#include <string>\n");
    h.push_str(&format!("class {} {{\npublic:\n", c.name));
    for f in &c.fields {
        h.push_str(&format!("  {} {};\n", cpp_type(&f.ty), f.name));
    }
    if !c.fields.is_empty() {
        let mut params: Vec<String> = Vec::new();
        for f in &c.fields {
            params.push(format!("{} {}", cpp_type(&f.ty), f.name));
        }
        h.push_str(&format!("  {}(", c.name));
        h.push_str(&params.join(", "));
        h.push_str(");\n");
    }
    for m in &c.methods {
        h.push_str(&format!("  {} {}(", cpp_type(&m.return_type), m.name));
        let mut params: Vec<String> = Vec::new();
        for p in &m.params {
            params.push(format!("{} {}", cpp_type(&p.ty), p.name));
        }
        h.push_str(&params.join(", "));
        h.push_str(");\n");
    }
    h.push_str("};\n");
    h
}

fn expr_cpp(e: &Expr) -> String {
    match e {
        Expr::LiteralString(s) => format!("std::string(\"{}\")", s.replace('"', "\\\"")),
        Expr::SelfField(n) => format!("this->{}", n),
        Expr::Concat(a, b) => format!("{} + {}", expr_cpp(a), expr_cpp(b)),
    }
}

pub fn source(c: &Class) -> String {
    let mut s = String::new();
    s.push_str(&format!("#include \"{}.hpp\"\n", c.name.to_lowercase()));
    s.push_str("#include <string>\n");
    if !c.fields.is_empty() {
        s.push_str(&format!("{}::{}(", c.name, c.name));
        let mut params: Vec<String> = Vec::new();
        for f in &c.fields {
            params.push(format!("{} {}", cpp_type(&f.ty), f.name));
        }
        s.push_str(&params.join(", "));
        s.push_str(") : ");
        let mut inits: Vec<String> = Vec::new();
        for f in &c.fields {
            inits.push(format!("{}({})", f.name, f.name));
        }
        s.push_str(&inits.join(", "));
        s.push_str(" {}\n");
    }
    for m in &c.methods {
        s.push_str(&method_impl(c, m));
    }
    s
}

fn method_impl(c: &Class, m: &Method) -> String {
    let mut out = String::new();
    out.push_str(&format!("{} {}::{}(", cpp_type(&m.return_type), c.name, m.name));
    let mut params: Vec<String> = Vec::new();
    for p in &m.params {
        params.push(format!("{} {}", cpp_type(&p.ty), p.name));
    }
    out.push_str(&params.join(", "));
    out.push_str(") {\n");
    if m.return_type != "Void" {
        out.push_str(&format!("  return {};\n", expr_cpp(&m.body)));
    }
    out.push_str("}\n");
    out
}

