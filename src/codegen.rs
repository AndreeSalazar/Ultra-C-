use crate::{Class, Expr, Method, Visibility};

fn cpp_type(t: &str) -> String {
    match t.trim() {
        "String" => "std::string".to_string(),
        "Void" => "void".to_string(),
        "Int" => "int".to_string(),
        "Float" => "float".to_string(),
        "Bool" => "bool".to_string(),
        _ => t.to_string(),
    }
}

fn cpp_default_init(t: &str) -> String {
    match t {
        "String" => "std::string()".to_string(),
        "Int" | "int" => "0".to_string(),
        "Bool" | "bool" => "false".to_string(),
        "Float" | "float" => "0.0f".to_string(),
        "Double" | "double" => "0.0".to_string(),
        _ => "{}".to_string(),
    }
}

pub fn header(c: &Class) -> String {
    let mut h = String::new();
    h.push_str("#pragma once\n");
    h.push_str("#include <string>\n");
    
    // Auto-includes for fields
    let mut seen_includes: Vec<String> = Vec::new();
    for f in &c.fields {
        let t = f.ty.trim();
        if !["Int", "Float", "Bool", "String", "Void"].contains(&t) {
             let inc = format!("{}.hpp", t.to_lowercase());
             if !seen_includes.contains(&inc) && inc != format!("{}.hpp", c.name.to_lowercase()) {
                  h.push_str(&format!("#include \"{}\"\n", inc));
                  seen_includes.push(inc);
             }
        }
    }

    // Auto-includes for methods (params)
    for m in &c.methods {
        for p in &m.params {
            let t = p.ty.trim();
            if !["Int", "Float", "Bool", "String", "Void"].contains(&t) {
                 let inc = format!("{}.hpp", t.to_lowercase());
                 if !seen_includes.contains(&inc) && inc != format!("{}.hpp", c.name.to_lowercase()) {
                      h.push_str(&format!("#include \"{}\"\n", inc));
                      seen_includes.push(inc);
                 }
            }
        }
    }

    if !c.extra_includes.is_empty() {
        for inc in &c.extra_includes {
            if !seen_includes.contains(inc) {
                h.push_str(&format!("#include <{}>\n", inc));
                seen_includes.push(inc.clone());
            }
        }
    }
    if let Some(b) = &c.base {
        h.push_str(&format!("#include \"{}.hpp\"\n", b.to_lowercase()));
    }
    if let Some(b) = &c.base {
        h.push_str(&format!("class {} : public {} {{\n", c.name, b));
    } else {
        h.push_str(&format!("class {} {{\n", c.name));
    }
    h.push_str("public:\n");
    for f in c.fields.iter().filter(|f| f.vis == Visibility::Public) {
        h.push_str(&format!("  {} {};\n", cpp_type(&f.ty), f.name));
    }
    let ctor_needed = c.ctor_params.is_some() || !c.fields.is_empty();
    if ctor_needed {
        let has_custom_default = if let Some(params) = &c.ctor_params {
            params.is_empty()
        } else {
            false
        };

        if !has_custom_default {
            h.push_str(&format!("  {}();\n", c.name));
        }
        if let Some(params) = &c.ctor_params {
            let mut ps: Vec<String> = Vec::new();
            for p in params {
                ps.push(format!("{} {}", cpp_type(&p.ty), p.name));
            }
            h.push_str(&format!("  {}(", c.name));
            h.push_str(&ps.join(", "));
            h.push_str(");\n");
        } else {
            let mut params: Vec<String> = Vec::new();
            for f in &c.fields {
                params.push(format!("{} {}", cpp_type(&f.ty), f.name));
            }
            h.push_str(&format!("  {}(", c.name));
            h.push_str(&params.join(", "));
            h.push_str(");\n");
        }
    }
    for m in c.methods.iter().filter(|m| m.vis == Visibility::Public) {
        if m.is_static {
            h.push_str(&format!("  static {} {}(", cpp_type(&m.return_type), m.name));
        } else {
            h.push_str(&format!("  {} {}(", cpp_type(&m.return_type), m.name));
        }
        let mut params: Vec<String> = Vec::new();
        for p in &m.params {
            params.push(format!("{} {}", cpp_type(&p.ty), p.name));
        }
        h.push_str(&params.join(", "));
        h.push_str(");\n");
    }
    let priv_fields: Vec<_> = c.fields.iter().filter(|f| f.vis == Visibility::Private).collect();
    let priv_methods: Vec<_> = c.methods.iter().filter(|m| m.vis == Visibility::Private).collect();
    if !priv_fields.is_empty() || !priv_methods.is_empty() {
        h.push_str("private:\n");
        for f in priv_fields {
            h.push_str(&format!("  {} {};\n", cpp_type(&f.ty), f.name));
        }
        for m in priv_methods {
            if m.is_static {
                h.push_str(&format!("  static {} {}(", cpp_type(&m.return_type), m.name));
            } else {
                h.push_str(&format!("  {} {}(", cpp_type(&m.return_type), m.name));
            }
            let mut params: Vec<String> = Vec::new();
            for p in &m.params {
                params.push(format!("{} {}", cpp_type(&p.ty), p.name));
            }
            h.push_str(&params.join(", "));
            h.push_str(");\n");
        }
    }
    h.push_str("};\n");
    h
}

fn expr_cpp(e: &Expr, c: &Class) -> String {
    match e {
        Expr::LiteralString(s) => format!("std::string(\"{}\")", s.replace('"', "\\\"")),
        Expr::LiteralInt(n) => format!("{}", n),
        Expr::LiteralBool(b) => {
            if *b { "true".to_string() } else { "false".to_string() }
        }
        Expr::LiteralFloat(f) => format!("{}", f),
        Expr::SelfField(n) => format!("this->{}", n),
        Expr::SelfCall { name, args } => {
            let mut a: Vec<String> = Vec::new();
            for x in args {
                a.push(expr_cpp(x, c));
            }
            format!("this->{}({})", name, a.join(", "))
        }
        Expr::SuperCall { name, args } => {
            let mut a: Vec<String> = Vec::new();
            for x in args {
                a.push(expr_cpp(x, c));
            }
            if let Some(b) = &c.base {
                format!("{}::{}({})", b, name, a.join(", "))
            } else {
                format!("{}({})", name, a.join(", "))
            }
        }
        Expr::Concat(a, b) => format!("{} + {}", expr_cpp(a, c), expr_cpp(b, c)),
        Expr::Native(s) => s.replace("\\\"", "\""),
    }
}

pub fn source(c: &Class) -> String {
    let mut s = String::new();
    s.push_str(&format!("#include \"{}.hpp\"\n", c.name.to_lowercase()));
    s.push_str("#include <string>\n");
    
    let ctor_needed = c.ctor_params.is_some() || !c.fields.is_empty();
    if ctor_needed {
        let has_custom_default = if let Some(params) = &c.ctor_params {
            params.is_empty()
        } else {
            false
        };

        if !has_custom_default {
            s.push_str(&format!("{}::{}() : ", c.name, c.name));
            let mut inits: Vec<String> = Vec::new();
            for f in &c.fields {
                inits.push(format!("{}({})", f.name, cpp_default_init(&f.ty)));
            }
            s.push_str(&inits.join(", "));
            s.push_str(" {}\n");
        }

        s.push_str(&format!("{}::{}(", c.name, c.name));
        let mut params: Vec<String> = Vec::new();
        let mut param_names: Vec<String> = Vec::new();
        if let Some(ps) = &c.ctor_params {
            for p in ps {
                params.push(format!("{} {}", cpp_type(&p.ty), p.name));
                param_names.push(p.name.clone());
            }
        } else {
            for f in &c.fields {
                params.push(format!("{} {}", cpp_type(&f.ty), f.name));
                param_names.push(f.name.clone());
            }
        }
        s.push_str(&params.join(", "));
        s.push_str(") : ");
        let mut inits: Vec<String> = Vec::new();
        for f in &c.fields {
             if param_names.contains(&f.name) {
                 inits.push(format!("{}({})", f.name, f.name));
             } else {
                 inits.push(format!("{}({})", f.name, cpp_default_init(&f.ty)));
             }
        }
        s.push_str(&inits.join(", "));
        s.push_str(" {\n");
        if let Some(body) = &c.ctor_body {
             if let Expr::Native(code) = body {
                 s.push_str(&format!("  {}\n", code.replace("\\\"", "\"")));
             } else {
                 s.push_str(&format!("  {};\n", expr_cpp(body, c)));
             }
        }
        s.push_str("}\n");
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
    if let Expr::Native(code) = &m.body {
        out.push_str(&format!("  {}\n", code.replace("\\\"", "\"")));
    } else if m.return_type != "Void" {
        out.push_str(&format!("  return {};\n", expr_cpp(&m.body, c)));
    }
    out.push_str("}\n");
    out
}
