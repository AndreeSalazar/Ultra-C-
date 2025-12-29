use crate::{Class, Expr, Method, Visibility};

fn is_builtin_ultra(t: &str) -> bool {
    matches!(
        t.trim(),
        "Int" | "Float" | "Bool" | "String" | "Void" | "Auto" | "Double"
    )
}

fn is_builtin_cpp(t: &str) -> bool {
    matches!(t.trim(), "int" | "float" | "double" | "bool" | "void")
}

fn is_std_like(t: &str) -> bool {
    let tt = t.trim();
    tt.starts_with("std::") || tt == "std::string"
}

fn is_container_cpp(t: &str) -> bool {
    let tt = t.trim();
    tt.starts_with("std::vector<")
        || tt.starts_with("std::map<")
        || tt.starts_with("std::list<")
        || tt.starts_with("std::optional<")
}

fn cpp_type(t: &str) -> String {
    let t = t.trim();
    if t.starts_with("Vector<") && t.ends_with(">") {
        let inner = &t[7..t.len() - 1];
        return format!("std::vector<{}>", cpp_type(inner));
    }
    if t.starts_with("Map<") && t.ends_with(">") {
        let inner = &t[4..t.len() - 1];
        // Split by comma respecting nesting (simplistic)
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() >= 2 {
            return format!("std::map<{}, {}>", cpp_type(parts[0]), cpp_type(parts[1]));
        }
    }
    if t.starts_with("List<") && t.ends_with(">") {
        let inner = &t[5..t.len() - 1];
        return format!("std::list<{}>", cpp_type(inner));
    }
    if t.starts_with("Optional<") && t.ends_with(">") {
        let inner = &t[9..t.len() - 1];
        return format!("std::optional<{}>", cpp_type(inner));
    }
    match t {
        "String" => "std::string".to_string(),
        "Void" => "void".to_string(),
        "Int" => "int".to_string(),
        "Float" => "float".to_string(),
        "Bool" => "bool".to_string(),
        "Thread" => "std::thread".to_string(),
        "Mutex" => "std::mutex".to_string(),
        "LockGuard" => "std::lock_guard<std::mutex>".to_string(),
        "Future" => "std::future".to_string(),
        "Promise" => "std::promise".to_string(),
        "Atomic" => "std::atomic".to_string(),
        "Path" => "std::filesystem::path".to_string(),
        "OfStream" => "std::ofstream".to_string(),
        "IfStream" => "std::ifstream".to_string(),
        "Auto" => "auto".to_string(),
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
        _ => format!("{}()", cpp_type(t)),
    }
}

pub fn unity_build(classes: &Vec<Class>) -> String {
    let mut s = String::new();

    // Standard headers
    let std_headers = vec![
        "iostream",
        "string",
        "vector",
        "memory",
        "algorithm",
        "map",
        "list",
        "optional",
        "thread",
        "mutex",
        "future",
        "atomic",
        "filesystem",
        "fstream",
        "numeric",
        "cmath",
        "cstdio",
    ];

    s.push_str("#include \"pch.hpp\"\n");
    for h in &std_headers {
        s.push_str(&format!("#include <{}>\n", h));
    }

    // Extra includes from all classes
    let mut extra_seen: Vec<String> = std_headers.iter().map(|s| s.to_string()).collect();

    for c in classes {
        for inc in &c.extra_includes {
            if !extra_seen.contains(inc) {
                s.push_str(&format!("#include <{}>\n", inc));
                extra_seen.push(inc.clone());
            }
        }
    }
    s.push_str("\n// Forward Declarations\n");
    for c in classes {
        if let Some(ns) = &c.namespace {
            s.push_str(&format!("namespace {} {{ class {}; }}\n", ns, c.name));
        } else {
            s.push_str(&format!("class {};\n", c.name));
        }
    }

    // Group classes by namespace for definitions
    use std::collections::HashMap;
    let mut ns_map: HashMap<Option<String>, Vec<&Class>> = HashMap::new();
    for c in classes {
        ns_map
            .entry(c.namespace.clone())
            .or_insert(Vec::new())
            .push(c);
    }

    // Sort namespaces to ensure deterministic order (None first, then alphabetical)
    let mut namespaces: Vec<_> = ns_map.keys().cloned().collect();
    namespaces.sort_by(|a, b| match (a, b) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(na), Some(nb)) => na.cmp(nb),
    });

    s.push_str("\n// Class Definitions\n");
    for ns in &namespaces {
        if let Some(n) = ns {
            s.push_str(&format!("namespace {} {{\n", n));
        }

        if let Some(group) = ns_map.get(ns) {
            for c in group {
                let h = header(c);
                // Filter out includes, pragma once, and namespace wrappers
                let lines: Vec<&str> = h
                    .lines()
                    .filter(|l| {
                        !l.starts_with("#include")
                            && !l.starts_with("#pragma")
                            && !l.starts_with("namespace ")
                            && l != &"}"
                    })
                    .collect();
                s.push_str(&lines.join("\n"));
                s.push_str("\n\n");
            }
        }

        if ns.is_some() {
            s.push_str("}\n");
        }
    }

    s.push_str("\n// Class Implementations\n");
    for ns in &namespaces {
        if let Some(n) = ns {
            s.push_str(&format!("namespace {} {{\n", n));
        }

        if let Some(group) = ns_map.get(ns) {
            for c in group {
                let src = source(c);
                // Filter out includes and namespace wrappers
                let mut lines: Vec<&str> = src
                    .lines()
                    .filter(|l| {
                        if l.starts_with("#include <conio.h>")
                            || l.starts_with("#include <windows.h>")
                        {
                            return true;
                        }
                        !l.starts_with("#include") && !l.starts_with("namespace ")
                    })
                    .collect();
                if c.namespace.is_some() && lines.last() == Some(&"}") {
                    lines.pop();
                }
                s.push_str(&lines.join("\n"));
                s.push_str("\n");
            }
        }

        if ns.is_some() {
            s.push_str("}\n");
        }
    }

    s
}

pub fn header(c: &Class) -> String {
    let mut h = String::new();
    h.push_str("#pragma once\n");
    h.push_str("#ifndef UCPP_API\n");
    h.push_str("#  if defined(_WIN32) && defined(UCPP_DLL)\n");
    h.push_str("#    ifdef UCPP_BUILD\n");
    h.push_str("#      define UCPP_API __declspec(dllexport)\n");
    h.push_str("#    else\n");
    h.push_str("#      define UCPP_API __declspec(dllimport)\n");
    h.push_str("#    endif\n");
    h.push_str("#  elif defined(__GNUC__)\n");
    h.push_str("#    define UCPP_API __attribute__((visibility(\"default\")))\n");
    h.push_str("#  else\n");
    h.push_str("#    define UCPP_API\n");
    h.push_str("#  endif\n");
    h.push_str("#endif\n");
    h.push_str("#ifndef UCPP_NOEXCEPT\n");
    h.push_str("#  ifdef _MSC_VER\n");
    h.push_str("#    define UCPP_NOEXCEPT noexcept\n");
    h.push_str("#  else\n");
    h.push_str("#    define UCPP_NOEXCEPT noexcept\n");
    h.push_str("#  endif\n");
    h.push_str("#endif\n");
    h.push_str("#include \"pch.hpp\"\n");
    // Standard includes (dedup aware)
    let std_headers = vec![
        "string",
        "vector",
        "iostream",
        "memory",
        "map",
        "list",
        "optional",
        "thread",
        "mutex",
        "future",
        "atomic",
        "filesystem",
        "fstream",
        "algorithm",
        "numeric",
        "cmath",
        "cstdio",
        "functional",
    ];
    for sh in &std_headers {
        h.push_str(&format!("#include <{}>\n", sh));
    }
    // Collect types used by value (require full include) vs by reference (can forward declare)
    let mut value_types: Vec<String> = Vec::new();
    let mut param_types: Vec<String> = Vec::new();
    for f in &c.fields {
        let t = f.ty.trim().to_string();
        if !is_builtin_ultra(&t) {
            value_types.push(t);
        }
    }
    for m in &c.methods {
        for p in &m.params {
            let t = p.ty.trim().to_string();
            if !is_builtin_ultra(&t) {
                param_types.push(t);
            }
        }
        let rt = m.return_type.trim().to_string();
        if !is_builtin_ultra(&rt) && rt != "Void" {
            value_types.push(rt);
        }
    }
    // Auto-includes for fields
    let mut seen_includes: Vec<String> = std_headers.iter().map(|s| s.to_string()).collect();
    for f in &c.fields {
        let t = f.ty.trim();
        let base_t = t.split('<').next().unwrap_or(t);
        if ![
            "Int",
            "Float",
            "Bool",
            "String",
            "Void",
            "Vector",
            "Map",
            "List",
            "Optional",
            "Thread",
            "Mutex",
            "LockGuard",
            "Future",
            "Promise",
            "Atomic",
            "Path",
            "OfStream",
            "IfStream",
            "Auto",
        ]
        .contains(&base_t)
        {
            let inc = format!("{}.hpp", base_t.to_lowercase());
            if !seen_includes.contains(&inc) && inc != format!("{}.hpp", c.name.to_lowercase()) {
                h.push_str(&format!("#include \"{}\"\n", inc));
                seen_includes.push(inc);
            }
        } else {
            if t.starts_with("Vector<") && t.ends_with(">") {
                let inner = &t[7..t.len() - 1].trim();
                if !["Int", "Float", "Bool", "String", "Void", "Auto"].contains(&inner) {
                    let inc = format!("{}.hpp", inner.to_lowercase());
                    if !seen_includes.contains(&inc)
                        && inc != format!("{}.hpp", c.name.to_lowercase())
                    {
                        h.push_str(&format!("#include \"{}\"\n", inc));
                        seen_includes.push(inc);
                    }
                }
            } else if t.starts_with("List<") && t.ends_with(">") {
                let inner = &t[5..t.len() - 1].trim();
                if !["Int", "Float", "Bool", "String", "Void", "Auto"].contains(&inner) {
                    let inc = format!("{}.hpp", inner.to_lowercase());
                    if !seen_includes.contains(&inc)
                        && inc != format!("{}.hpp", c.name.to_lowercase())
                    {
                        h.push_str(&format!("#include \"{}\"\n", inc));
                        seen_includes.push(inc);
                    }
                }
            } else if t.starts_with("Optional<") && t.ends_with(">") {
                let inner = &t[9..t.len() - 1].trim();
                if !["Int", "Float", "Bool", "String", "Void", "Auto"].contains(&inner) {
                    let inc = format!("{}.hpp", inner.to_lowercase());
                    if !seen_includes.contains(&inc)
                        && inc != format!("{}.hpp", c.name.to_lowercase())
                    {
                        h.push_str(&format!("#include \"{}\"\n", inc));
                        seen_includes.push(inc);
                    }
                }
            } else if t.starts_with("Map<") && t.ends_with(">") {
                let inner = &t[4..t.len() - 1];
                let parts: Vec<&str> = inner.split(',').collect();
                for part in parts {
                    let typ = part.trim();
                    if !["Int", "Float", "Bool", "String", "Void", "Auto"].contains(&typ) {
                        let inc = format!("{}.hpp", typ.to_lowercase());
                        if !seen_includes.contains(&inc)
                            && inc != format!("{}.hpp", c.name.to_lowercase())
                        {
                            h.push_str(&format!("#include \"{}\"\n", inc));
                            seen_includes.push(inc);
                        }
                    }
                }
            }
        }
    }

    // Auto-includes for methods (params used by value only)
    for m in &c.methods {
        for p in &m.params {
            let t = p.ty.trim();
            if !["Int", "Float", "Bool", "String", "Void", "Auto"].contains(&t) {
                // If the type will be passed by const-ref, we prefer forward declaration
                if value_types.iter().any(|vt| vt == t) {
                    let inc = format!("{}.hpp", t.to_lowercase());
                    if !seen_includes.contains(&inc)
                        && inc != format!("{}.hpp", c.name.to_lowercase())
                    {
                        h.push_str(&format!("#include \"{}\"\n", inc));
                        seen_includes.push(inc);
                    }
                }
            }
        }
        let rt = m.return_type.trim();
        if !["Int", "Float", "Bool", "String", "Void", "Auto"].contains(&rt) {
            let inc = format!("{}.hpp", rt.to_lowercase());
            if !seen_includes.contains(&inc) && inc != format!("{}.hpp", c.name.to_lowercase()) {
                h.push_str(&format!("#include \"{}\"\n", inc));
                seen_includes.push(inc);
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
    // Forward declarations for param-only types
    let mut fwd: Vec<String> = Vec::new();
    for t in &param_types {
        // Handle generics: Vector<Rect> -> Rect
        let clean = t.replace('<', " ").replace('>', " ").replace(',', " ");
        for part in clean.split_whitespace() {
            let p = part.trim();
            if [
                "Int",
                "Float",
                "Bool",
                "String",
                "Void",
                "Auto",
                "Double",
                "Vector",
                "Map",
                "List",
                "Optional",
                "Thread",
                "Mutex",
                "LockGuard",
                "Future",
                "Promise",
                "Atomic",
                "Path",
                "OfStream",
                "IfStream",
            ]
            .contains(&p)
            {
                continue;
            }
            // If already included, no need to forward declare
            let inc_name = format!("{}.hpp", p.to_lowercase());
            if !seen_includes.contains(&inc_name) && p != c.name {
                if !fwd.contains(&p.to_string()) {
                    fwd.push(p.to_string());
                }
            }
        }
    }
    if !fwd.is_empty() {
        if let Some(ns) = &c.namespace {
            h.push_str(&format!("namespace {} {{\n", ns));
            for t in &fwd {
                h.push_str(&format!("class {};\n", t));
            }
            h.push_str("}\n");
        } else {
            for t in &fwd {
                h.push_str(&format!("class {};\n", t));
            }
        }
    }
    if let Some(ns) = &c.namespace {
        h.push_str(&format!("namespace {} {{\n", ns));
    }
    if let Some(b) = &c.base {
        h.push_str(&format!("class UCPP_API {} : public {} {{\n", c.name, b));
    } else {
        h.push_str(&format!("class UCPP_API {} {{\n", c.name));
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
            h.push_str(&format!(
                "  static {} {}(",
                cpp_type(&m.return_type),
                m.name
            ));
        } else {
            h.push_str(&format!("  {} {}(", cpp_type(&m.return_type), m.name));
        }
        let mut params: Vec<String> = Vec::new();
        for p in &m.params {
            let ct = cpp_type(&p.ty);
            let pass_const_ref =
                !is_builtin_ultra(&p.ty) || is_std_like(&ct) || is_container_cpp(&ct);
            if pass_const_ref && !is_builtin_cpp(&ct) {
                params.push(format!("const {}& {}", ct, p.name));
            } else {
                params.push(format!("{} {}", ct, p.name));
            }
        }
        h.push_str(&params.join(", "));
        // Const-correctness: mark known read-only methods as const
        let needs_const = !m.is_static
            && ((c.name == "Rect" && m.name == "collides")
                || (c.name == "Player" && m.name == "get_rect")
                || (c.name == "Strings" && m.name == "get")
                || (c.name == "ResourceManager" && m.name == "get"));
        let noexc = (!m.is_static)
            && ((m.name == "render")
                || (m.name == "collides")
                || (m.name == "get_rect")
                || (m.name == "get")
                || (m.name == "play")
                || (m.name == "release"));
        if needs_const {
            if noexc {
                h.push_str(") const UCPP_NOEXCEPT;\n");
            } else {
                h.push_str(") const;\n");
            }
        } else {
            if noexc {
                h.push_str(") UCPP_NOEXCEPT;\n");
            } else {
                h.push_str(");\n");
            }
        }
    }
    let priv_fields: Vec<_> = c
        .fields
        .iter()
        .filter(|f| f.vis == Visibility::Private)
        .collect();
    let priv_methods: Vec<_> = c
        .methods
        .iter()
        .filter(|m| m.vis == Visibility::Private)
        .collect();
    if !priv_fields.is_empty() || !priv_methods.is_empty() {
        h.push_str("private:\n");
        for f in priv_fields {
            h.push_str(&format!("  {} {};\n", cpp_type(&f.ty), f.name));
        }
        for m in priv_methods {
            if m.is_static {
                h.push_str(&format!(
                    "  static {} {}(",
                    cpp_type(&m.return_type),
                    m.name
                ));
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
        Expr::LiteralString(s) => format!("\"{}\"", s.replace('"', "\\\"")),
        Expr::LiteralInt(n) => format!("{}", n),
        Expr::LiteralFloat(f) => format!("{}", f),
        Expr::LiteralBool(b) => {
            if *b {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        Expr::UnaryOp(op, x) => {
            let cpp = match op.as_str() {
                "not" => "!",
                _ => op.as_str(),
            };
            format!("({}{})", cpp, gen_expr(x, c))
        }
        Expr::Variable(s) => {
            // Map generics like Vector<Rect>() into std::vector<Rect>()
            let t = s.replace(' ', "");
            if t.ends_with("()") {
                let typ = &t[..t.len() - 2];
                if typ.starts_with("Vector<")
                    || typ.starts_with("Map<")
                    || typ.starts_with("List<")
                    || typ.starts_with("Optional<")
                {
                    return format!("{}()", cpp_type(typ));
                }
            }
            s.clone()
        }
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
            if name == "print" {
                let mut s = "std::cout".to_string();
                for arg in args {
                    s.push_str(" << ");
                    s.push_str(&gen_expr(arg, c));
                }
                s.push_str(" << std::endl");
                return s;
            }
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
            if op == "+" {
                match (&**l, &**r) {
                    (Expr::LiteralString(_), _) => {
                        format!("std::string({}) + {}", gen_expr(l, c), gen_expr(r, c))
                    }
                    (_, Expr::LiteralString(_)) => {
                        format!("{} + std::string({})", gen_expr(l, c), gen_expr(r, c))
                    }
                    _ => format!("{} + {}", gen_expr(l, c), gen_expr(r, c)),
                }
            } else {
                format!("{} {} {}", gen_expr(l, c), cpp_op, gen_expr(r, c))
            }
        }
        Expr::Concat(l, r) => format!("std::string({}) + {}", gen_expr(l, c), gen_expr(r, c)),
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
            // Check if there is a method in this class with the name "{name}_upp" or "{name}"
            let target_method_upp = format!("{}_upp", name.replace('.', "_"));
            let target_method_std = name.replace('.', "_");

            let has_upp = c.methods.iter().any(|m| m.name == target_method_upp);
            let has_std = c.methods.iter().any(|m| m.name == target_method_std);

            if has_upp {
                // Call the method
                let is_static = c
                    .methods
                    .iter()
                    .find(|m| m.name == target_method_upp)
                    .unwrap()
                    .is_static;
                if is_static {
                    out.push_str(&format!("{}{}::{}();\n", prefix, c.name, target_method_upp));
                } else {
                    out.push_str(&format!("{}{}();\n", prefix, target_method_upp));
                }
            } else if has_std {
                let is_static = c
                    .methods
                    .iter()
                    .find(|m| m.name == target_method_std)
                    .unwrap()
                    .is_static;
                if is_static {
                    out.push_str(&format!("{}{}::{}();\n", prefix, c.name, target_method_std));
                } else {
                    out.push_str(&format!("{}{}();\n", prefix, target_method_std));
                }
            } else {
                // Fallback: Just print for now, or assume it's an external call
                if name.to_lowercase() == "hola" {
                    out.push_str(&format!(
                        "{}std::cout << \"Hola mundo\" << std::endl;\n",
                        prefix
                    ));
                } else {
                    out.push_str(&format!(
                        "{}std::cout << \"Run {}.upp\" << std::endl;\n",
                        prefix, name
                    ));
                }
            }
            out
        }
        Expr::If {
            cond,
            then_body,
            else_body,
        } => {
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
                format!(
                    "{}{} {} = {};\n",
                    prefix,
                    cpp_type(ty),
                    name,
                    gen_expr(v, c)
                )
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
    s.push_str("#include \"pch.hpp\"\n");
    s.push_str(&format!("#include \"{}.hpp\"\n", c.name.to_lowercase()));
    let mut sig_includes: Vec<String> = Vec::new();
    fn is_builtin_sig(t: &str) -> bool {
        matches!(
            t.trim(),
            "Int"
                | "Float"
                | "Bool"
                | "String"
                | "Void"
                | "int"
                | "float"
                | "bool"
                | "double"
                | "Double"
                | "Auto"
                | "Vector"
                | "Map"
                | "List"
                | "Optional"
                | "Thread"
                | "Mutex"
                | "LockGuard"
                | "Future"
                | "Promise"
                | "Atomic"
        )
    }
    fn extract_types(t: &str) -> Vec<String> {
        let clean = t
            .replace('<', " ")
            .replace('>', " ")
            .replace(',', " ");
        let mut out: Vec<String> = Vec::new();
        for part in clean.split_whitespace() {
            let p = part.trim();
            if p.is_empty() {
                continue;
            }
            if !is_builtin_sig(p) {
                out.push(p.to_string());
            }
        }
        out
    }
    for m in &c.methods {
        for p in &m.params {
            for ty in extract_types(&p.ty) {
                let low = ty.to_lowercase();
                if low != c.name.to_lowercase()
                    && !sig_includes.iter().any(|x| x == &low)
                {
                    sig_includes.push(low);
                }
            }
        }
        let rt = m.return_type.trim();
        if !is_builtin_sig(rt) && rt != "Void" {
            for ty in extract_types(rt) {
                let low = ty.to_lowercase();
                if low != c.name.to_lowercase()
                    && !sig_includes.iter().any(|x| x == &low)
                {
                    sig_includes.push(low);
                }
            }
        }
    }
    for inc in sig_includes {
        s.push_str(&format!("#include \"{}.hpp\"\n", inc));
    }
    // Auto-includes for local VarDecl types used inside methods
    let mut local_includes: Vec<String> = Vec::new();
    let mut need_win = false;
    fn is_builtin(t: &str) -> bool {
        matches!(
            t.trim(),
            "Int"
                | "Float"
                | "Bool"
                | "String"
                | "Void"
                | "int"
                | "float"
                | "bool"
                | "double"
                | "Double"
                | "Auto"
        )
    }
    fn is_builtin_class_name(n: &str) -> bool {
        matches!(
            n.trim(),
            "Int"
                | "Float"
                | "Bool"
                | "String"
                | "Void"
                | "Vector"
                | "Map"
                | "List"
                | "Optional"
                | "Thread"
                | "Mutex"
                | "LockGuard"
                | "Future"
                | "Promise"
                | "Atomic"
                | "Path"
                | "OfStream"
                | "IfStream"
                | "Auto"
                | "Double"
        )
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
            Expr::If {
                then_body,
                else_body,
                ..
            } => {
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
    fn collect_class_refs(e: &Expr, acc: &mut Vec<String>, self_lower: &str) {
        match e {
            Expr::FunctionCall { name, args } => {
                // Consider names that look like class usage
                let mut candidates: Vec<String> = Vec::new();
                if name.contains('.') {
                    let parts: Vec<&str> = name.split('.').collect();
                    if parts.len() >= 2 {
                        candidates.push(parts[parts.len() - 2].to_string());
                    } else {
                        candidates.push(parts[0].to_string());
                    }
                } else {
                    candidates.push(name.clone());
                }
                for cand in candidates {
                    if let Some(ch) = cand.chars().next() {
                        if ch.is_uppercase() && !is_builtin_class_name(&cand) {
                            let low = cand.to_lowercase();
                            if !acc.iter().any(|x| x == &low) && low != self_lower {
                                acc.push(low);
                            }
                        }
                    }
                }
                for a in args {
                    collect_class_refs(a, acc, self_lower);
                }
            }
            Expr::Concat(l, r) => {
                collect_class_refs(l, acc, self_lower);
                collect_class_refs(r, acc, self_lower);
            }
            Expr::Block(stmts) => {
                for s in stmts {
                    collect_class_refs(s, acc, self_lower);
                }
            }
            Expr::If {
                then_body,
                else_body,
                ..
            } => {
                collect_class_refs(then_body, acc, self_lower);
                if let Some(e) = else_body {
                    collect_class_refs(e, acc, self_lower);
                }
            }
            Expr::While { body, .. } => collect_class_refs(body, acc, self_lower),
            Expr::Return(Some(v)) => collect_class_refs(v, acc, self_lower),
            Expr::VarDecl { value, .. } => {
                if let Some(v) = value {
                    collect_class_refs(v, acc, self_lower);
                }
            }
            _ => {}
        }
    }
    fn contains_win_native(e: &Expr) -> bool {
        match e {
            Expr::Native(s) => s.contains("_kbhit") || s.contains("_getch") || s.contains("Sleep"),
            Expr::Block(stmts) => stmts.iter().any(contains_win_native),
            Expr::If {
                then_body,
                else_body,
                ..
            } => {
                if contains_win_native(then_body) {
                    return true;
                }
                if let Some(e) = else_body {
                    return contains_win_native(e);
                }
                false
            }
            Expr::While { body, .. } => contains_win_native(body),
            Expr::Return(Some(v)) => contains_win_native(v),
            _ => false,
        }
    }
    for m in &c.methods {
        collect_types(&m.body, &mut local_includes);
        if contains_win_native(&m.body) {
            need_win = true;
        }
        let mut refs: Vec<String> = Vec::new();
        collect_class_refs(&m.body, &mut refs, &c.name.to_lowercase());
        for r in refs {
            if r != c.name.to_lowercase() {
                s.push_str(&format!("#include \"{}.hpp\"\n", r));
            }
        }
        // Fallback: scan generated code for static class usages like Utils::Version::...
        let body_code = gen_stmt(&m.body, c, 1);
        let mut scan_refs: Vec<String> = Vec::new();
        let bytes = body_code.as_bytes();
        let mut i = 0usize;
        while i + 1 < bytes.len() {
            if bytes[i] == b':' && bytes[i + 1] == b':' {
                // Capture name after :: (method or constant)
                let mut j = i + 2;
                let mut name = String::new();
                while j < bytes.len() {
                    let ch = bytes[j] as char;
                    if ch.is_alphanumeric() || ch == '_' {
                        name.push(ch);
                        j += 1;
                    } else {
                        break;
                    }
                }
                // Capture class name before :: (e.g., Sound::play -> Sound)
                let mut k: isize = (i as isize) - 1;
                let mut pre = String::new();
                while k >= 0 {
                    let ch = bytes[k as usize] as char;
                    if ch.is_alphanumeric() || ch == '_' {
                        pre.insert(0, ch);
                        k -= 1;
                    } else {
                        break;
                    }
                }
                // Prefer 'name' when it looks like a type; otherwise fall back to 'pre'
                let mut candidate: Option<String> = None;
                if !name.is_empty() {
                    if let Some(first) = name.chars().next() {
                        if first.is_uppercase() && !is_builtin_class_name(&name) {
                            candidate = Some(name.to_lowercase());
                        }
                    }
                }
                if candidate.is_none() && !pre.is_empty() {
                    if let Some(first) = pre.chars().next() {
                        if first.is_uppercase() && !is_builtin_class_name(&pre) {
                            candidate = Some(pre.to_lowercase());
                        }
                    }
                }
                if let Some(low) = candidate {
                    if low != c.name.to_lowercase() && !scan_refs.iter().any(|x| x == &low) {
                        scan_refs.push(low);
                    }
                }
                i = j;
            } else {
                i += 1;
            }
        }
        for r in scan_refs {
            s.push_str(&format!("#include \"{}.hpp\"\n", r));
        }
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
        // Ensure method is closed properly if method_impl didn't close it (it does now)
    }
    if c.namespace.is_some() {
        s.push_str("}\n");
    }
    s
}

fn method_impl(c: &Class, m: &Method) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "{} {}::{}(",
        cpp_type(&m.return_type),
        c.name,
        m.name
    ));
    let mut params: Vec<String> = Vec::new();
    for p in &m.params {
        let ct = cpp_type(&p.ty);
        let pass_const_ref = !is_builtin_ultra(&p.ty) || is_std_like(&ct) || is_container_cpp(&ct);
        if pass_const_ref && !is_builtin_cpp(&ct) {
            params.push(format!("const {}& {}", ct, p.name));
        } else {
            params.push(format!("{} {}", ct, p.name));
        }
    }
    out.push_str(&params.join(", "));
    // Const-correctness in implementation
    let needs_const = !m.is_static
        && ((c.name == "Rect" && m.name == "collides")
            || (c.name == "Player" && m.name == "get_rect")
            || (c.name == "Strings" && m.name == "get")
            || (c.name == "ResourceManager" && m.name == "get"));
    let noexc = (!m.is_static)
        && ((m.name == "render")
            || (m.name == "collides")
            || (m.name == "get_rect")
            || (m.name == "get")
            || (m.name == "play")
            || (m.name == "release"));
    if needs_const {
        if noexc {
            out.push_str(") const UCPP_NOEXCEPT {\n");
        } else {
            out.push_str(") const {\n");
        }
    } else if noexc {
        out.push_str(") UCPP_NOEXCEPT {\n");
    } else {
        out.push_str(") {\n");
    }
    out.push_str(&gen_stmt(&m.body, c, 1));
    out.push_str("}\n");
    out
}
