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
    h.push_str("#include <vector>\n");
    h.push_str("#include <iostream>\n");
    h.push_str("#include <memory>\n");
    
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
    if let Some(ns) = &c.namespace {
        h.push_str(&format!("namespace {} {{\n", ns));
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
    if c.namespace.is_some() {
        h.push_str("}\n");
    }
    h
}

fn gen_expr(e: &Expr, c: &Class) -> String {
    match e {
        Expr::LiteralString(s) => format!("std::string(\"{}\")", s.replace('"', "\\\"")),
        Expr::LiteralInt(n) => format!("{}", n),
        Expr::LiteralFloat(f) => format!("{}", f),
        Expr::LiteralBool(b) => if *b { "true".to_string() } else { "false".to_string() },
        Expr::UnaryOp(op, x) => {
            let cpp = match op.as_str() {
                "not" => "!",
                _ => op.as_str(),
            };
            format!("({}{})", cpp, gen_expr(x, c))
        }
        Expr::Variable(n) => n.clone(),
        Expr::SelfField(n) => format!("this->{}", n),
        Expr::SelfCall { name, args } => {
            let a: Vec<String> = args.iter().map(|x| gen_expr(x, c)).collect();
            // Basic static check: if method is in class and static, use ClassName::
            // We need to find if 'name' is a static method in 'c'.
            let is_static = c.methods.iter().any(|m| m.name == *name && m.is_static);
            if is_static {
                format!("{}::{}({})", c.name, name, a.join(", "))
            } else {
                format!("this->{}({})", name, a.join(", "))
            }
        }
        Expr::SuperCall { name, args } => {
             let a: Vec<String> = args.iter().map(|x| gen_expr(x, c)).collect();
             if let Some(b) = &c.base {
                 format!("{}::{}({})", b, name, a.join(", "))
             } else {
                 format!("{}({})", name, a.join(", "))
             }
        }
        Expr::FunctionCall { name, args } => {
            let a: Vec<String> = args.iter().map(|x| gen_expr(x, c)).collect();
            // Replace dot with double colon for likely static calls if it looks like Class.Method
            let cpp_name = if name.contains('.') && name.chars().next().unwrap().is_uppercase() {
                name.replace('.', "::")
            } else {
                name.clone()
            };
            format!("{}({})", cpp_name, a.join(", "))
        }
        Expr::BinaryOp(l, op, r) => {
            let cpp_op = match op.as_str() {
                "and" => "&&",
                "or" => "||",
                _ => op.as_str(),
            };
            format!("{} {} {}", gen_expr(l, c), cpp_op, gen_expr(r, c))
        }
        Expr::Concat(l, r) => format!("({} + {})", gen_expr(l, c), gen_expr(r, c)),
        Expr::Native(s) => s.replace("\\\"", "\""),
        _ => "".to_string(), 
    }
}

fn gen_stmt(e: &Expr, c: &Class, indent: usize) -> String {
    let prefix = "  ".repeat(indent);
    match e {
        Expr::Block(stmts) => {
            let mut out = String::new();
            for s in stmts {
                out.push_str(&gen_stmt(s, c, indent));
            }
            out
        }
        Expr::FileCall(name) => {
            let mut out = String::new();
            if name.to_lowercase() == "hola" {
                out.push_str(&format!("{}std::cout << \"Hola mundo\" << std::endl;\n", prefix));
            } else {
                out.push_str(&format!("{}std::cout << \"Run {}.upp\" << std::endl;\n", prefix, name));
            }
            out
        }
        Expr::If { cond, then_body, else_body } => {
            let mut out = format!("{}if ({}) {{\n", prefix, gen_expr(cond, c));
            out.push_str(&gen_stmt(then_body, c, indent + 1));
            out.push_str(&format!("{}}}", prefix));
            if let Some(else_b) = else_body {
                 out.push_str(" else {\n");
                 out.push_str(&gen_stmt(else_b, c, indent + 1));
                 out.push_str(&format!("{}}}", prefix));
            }
            out.push('\n');
            out
        }
        Expr::While { cond, body } => {
            let mut out = format!("{}while ({}) {{\n", prefix, gen_expr(cond, c));
            out.push_str(&gen_stmt(body, c, indent + 1));
            out.push_str(&format!("{}}}\n", prefix));
            out
        }
        Expr::VarDecl { name, ty, value } => {
            if let Some(v) = value {
                format!("{}{} {} = {};\n", prefix, cpp_type(ty), name, gen_expr(v, c))
            } else {
                format!("{}{} {};\n", prefix, cpp_type(ty), name)
            }
        }
        Expr::Return(val) => {
            if let Some(v) = val {
                format!("{}return {};\n", prefix, gen_expr(v, c))
            } else {
                format!("{}return;\n", prefix)
            }
        }
        Expr::Native(s) => {
            let lines: Vec<&str> = s.lines().collect();
            let mut out = String::new();
            for l in lines {
                out.push_str(&format!("{}{}\n", prefix, l.replace("\\\"", "\"")));
            }
            out
        }
        _ => {
            format!("{}{};\n", prefix, gen_expr(e, c))
        }
    }
}

pub fn source(c: &Class) -> String {
    let mut s = String::new();
    s.push_str(&format!("#include \"{}.hpp\"\n", c.name.to_lowercase()));
    // Auto-includes for local VarDecl types used inside methods
    let mut local_includes: Vec<String> = Vec::new();
    let mut need_win = false;
    fn is_builtin(t: &str) -> bool {
        matches!(t.trim(), "Int" | "Float" | "Bool" | "String" | "Void" | "int" | "float" | "bool" | "double" | "Double")
    }
    fn collect_types(e: &Expr, acc: &mut Vec<String>) {
        match e {
            Expr::VarDecl { ty, .. } => {
                let t = ty.trim().to_string();
                if !is_builtin(&t) && !acc.iter().any(|x| x == &t) {
                    acc.push(t);
                }
            }
            Expr::Block(stmts) => {
                for s in stmts {
                    collect_types(s, acc);
                }
            }
            Expr::If { then_body, else_body, .. } => {
                collect_types(then_body, acc);
                if let Some(e) = else_body {
                    collect_types(e, acc);
                }
            }
            Expr::While { body, .. } => collect_types(body, acc),
            Expr::Return(Some(v)) => collect_types(v, acc),
            _ => {}
        }
    }
    fn contains_win_native(e: &Expr) -> bool {
        match e {
            Expr::Native(s) => s.contains("_kbhit") || s.contains("_getch") || s.contains("Sleep"),
            Expr::Block(stmts) => stmts.iter().any(contains_win_native),
            Expr::If { then_body, else_body, .. } => {
                if contains_win_native(then_body) { return true; }
                if let Some(e) = else_body { return contains_win_native(e); }
                false
            }
            Expr::While { body, .. } => contains_win_native(body),
            Expr::Return(Some(v)) => contains_win_native(v),
            _ => false,
        }
    }
    for m in &c.methods {
        collect_types(&m.body, &mut local_includes);
        if contains_win_native(&m.body) { need_win = true; }
    }
    for inc in local_includes {
        if inc.to_lowercase() != c.name.to_lowercase() {
            s.push_str(&format!("#include \"{}.hpp\"\n", inc.to_lowercase()));
        }
    }
    s.push_str("#include <string>\n");
    s.push_str("#include <vector>\n");
    s.push_str("#include <iostream>\n");
    if need_win {
        s.push_str("#ifdef _WIN32\n");
        s.push_str("#include <conio.h>\n");
        s.push_str("#include <windows.h>\n");
        s.push_str("#endif\n");
    }
    
    if let Some(ns) = &c.namespace {
        s.push_str(&format!("namespace {} {{\n", ns));
    }
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
             s.push_str(&gen_stmt(body, c, 1));
        }
        s.push_str("}\n");
    }
    for m in &c.methods {
        s.push_str(&method_impl(c, m));
    }
    if c.namespace.is_some() {
        s.push_str("}\n");
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
    
    out.push_str(&gen_stmt(&m.body, c, 1));
    
    out.push_str("}\n");
    out
}
