use crate::{Class, Expr, Field, Method, Param, Visibility};
use crate::Directives;

fn trim(s: &str) -> String {
    s.trim().to_string()
}

fn indent_of(line: &str) -> usize {
    let mut count = 0;
    for ch in line.chars() {
        match ch {
            ' ' => count += 1,
            '\t' => count += 4,
            _ => break,
        }
    }
    count
}

pub fn parse(input: &str) -> Class {
    let raw_lines: Vec<&str> = input.lines().collect();
    let mut i = 0usize;
    let mut name = String::new();
    let mut fields: Vec<Field> = Vec::new();
    let mut methods: Vec<Method> = Vec::new();
    let mut base: Option<String> = None;

    while i < raw_lines.len() {
        let line = raw_lines[i];
        let content = line.trim();
        if content.is_empty() {
            i += 1;
            continue;
        }
        if content.starts_with("class ") {
            let class_indent = indent_of(line);
            let mut cls = content["class ".len()..].to_string();
            if let Some(paren_start) = cls.find('(') {
                let mut nm = cls[..paren_start].to_string();
                if nm.ends_with(':') {
                    nm.pop();
                }
                name = trim(&nm);
                let rest = &cls[paren_start + 1..];
                if let Some(paren_end_rel) = rest.find(')') {
                    let paren_end = paren_start + 1 + paren_end_rel;
                    let base_part = &cls[paren_start + 1..paren_end];
                    let mut b = base_part.to_string();
                    if b.ends_with(':') {
                        b.pop();
                    }
                    let b = trim(&b);
                    if !b.is_empty() {
                        base = Some(b);
                    }
                }
            } else {
                if cls.ends_with(':') {
                    cls.pop();
                }
                name = trim(&cls);
            }
            i += 1;
            // Parse class block
            let mut current_vis = Visibility::Public;
            let mut ctor_params: Option<Vec<Param>> = None;
            let mut ctor_body: Option<Expr> = None;
            while i < raw_lines.len() {
                let l = raw_lines[i];
                let c = l.trim();
                if c.is_empty() {
                    i += 1;
                    continue;
                }
                let ind = indent_of(l);
                if ind <= class_indent {
                    break;
                }
                if c == "public:" {
                    current_vis = Visibility::Public;
                    i += 1;
                    continue;
                }
                if c == "private:" {
                    current_vis = Visibility::Private;
                    i += 1;
                    continue;
                }
                if c.starts_with("def ") {
                    let def_indent = ind;
                    // def name(self, a: T) -> R:
                    let sig = c["def ".len()..].to_string();
                    let mut mname = String::new();
                    let mut params: Vec<Param> = Vec::new();
                    let mut has_self = false;
                    let mut ret_type = String::from("Void");
                    if let Some(paren_start) = sig.find('(') {
                        mname = trim(&sig[..paren_start]);
                        let mut rest = &sig[paren_start + 1..];
                        if let Some(paren_end_rel) = rest.find(')') {
                            let paren_end = paren_start + 1 + paren_end_rel;
                            let params_part = &sig[paren_start + 1..paren_end];
                            for p in params_part.split(',') {
                                let p = p.trim();
                                if p.is_empty() {
                                    continue;
                                }
                                if p == "self" {
                                    has_self = true;
                                    continue;
                                }
                                let mut kv = p.split(':');
                                let pn = trim(kv.next().unwrap_or(""));
                                let pt = trim(kv.next().unwrap_or(""));
                                if !pn.is_empty() && !pt.is_empty() {
                                    params.push(Param { name: pn, ty: pt });
                                }
                            }
                            rest = &sig[paren_end + 1..];
                        }
                        if let Some(arrow_idx) = rest.find("->") {
                            let mut rt = trim(&rest[arrow_idx + 2..]);
                            if rt.ends_with(':') {
                                rt.pop();
                            }
                            ret_type = trim(&rt);
                        } else {
                            if rest.ends_with(':') {
                                // no return type, still valid
                            }
                        }
                    } else {
                        // no params
                        let mut n = sig.clone();
                        if n.ends_with(':') {
                            n.pop();
                        }
                        mname = trim(&n);
                    }
                    // Parse method body: find first line with indent > def_indent starting with return
                    i += 1;
                    let mut body_expr = Expr::LiteralString(String::new());
                    while i < raw_lines.len() {
                        let bl = raw_lines[i];
                        let bc = bl.trim();
                        if bc.is_empty() {
                            i += 1;
                            continue;
                        }
                        let bind = indent_of(bl);
                        if bind <= def_indent {
                            break;
                        }
                        if bc.starts_with("return ") {
                            let expr_str = trim(&bc["return ".len()..]);
                            body_expr = parse_expr(&expr_str);
                            // consume rest of method body lines at same or greater indent until block ends
                            // but for our simple parser, break after capturing return
                            // advance to end of block
                        } else if bc.starts_with("native ") {
                            let mut code = trim(&bc["native ".len()..]);
                            if code.starts_with('"') && !code[1..].contains('"') {
                                // Multiline native block start
                                let mut content = code[1..].to_string(); // Remove opening "
                                content.push('\n');
                                i += 1;
                                while i < raw_lines.len() {
                                    let next_line = raw_lines[i];
                                    let trimmed_next = next_line.trim();
                                    if trimmed_next.ends_with('"') {
                                        content.push_str(&trimmed_next[..trimmed_next.len()-1]); // Remove closing "
                                        break;
                                    } else {
                                        content.push_str(trimmed_next);
                                        content.push('\n');
                                    }
                                    i += 1;
                                }
                                body_expr = Expr::Native(content);
                            } else {
                                let content = if code.starts_with('"') && code.ends_with('"') && code.len() >= 2 {
                                    code[1..code.len() - 1].to_string()
                                } else {
                                    code
                                };
                                body_expr = Expr::Native(content);
                            }
                        } else {
                            if matches!(body_expr, Expr::LiteralString(ref s) if s.is_empty()) {
                                body_expr = parse_expr(bc);
                            }
                        }
                        i += 1;
                    }
                    if mname == "__init__" {
                        ctor_params = Some(params);
                        ctor_body = Some(body_expr);
                        continue;
                    }
                    methods.push(Method {
                        name: mname,
                        return_type: ret_type,
                        params,
                        body: body_expr,
                        is_static: !has_self,
                        vis: current_vis.clone(),
                    });
                    continue;
                }
                // Alt method syntax: "name -> Ret" (opcional cuerpo en la siguiente línea)
                if let Some(arrow_idx) = c.find("->") {
                    let def_indent = ind;
                    let left = trim(&c[..arrow_idx]);
                    let mut mname = left.clone();
                    let mut params: Vec<Param> = Vec::new();
                    // parámetros opcionales: nombre(p: T, q: U)
                    if let Some(paren_start) = left.find('(') {
                        mname = trim(&left[..paren_start]);
                        if let Some(paren_end_rel) = left[paren_start + 1..].find(')') {
                            let paren_end = paren_start + 1 + paren_end_rel;
                            let params_part = &left[paren_start + 1..paren_end];
                            for p in params_part.split(',') {
                                let p = p.trim();
                                if p.is_empty() {
                                    continue;
                                }
                                let mut kv = p.split(':');
                                let pn = trim(kv.next().unwrap_or(""));
                                let pt = trim(kv.next().unwrap_or(""));
                                if !pn.is_empty() && !pt.is_empty() {
                                    params.push(Param { name: pn, ty: pt });
                                }
                            }
                        }
                    }
                    let mut rt = trim(&c[arrow_idx + 2..]);
                    if rt.ends_with(':') {
                        rt.pop();
                        rt = trim(&rt);
                    }
                    let mut body_expr = Expr::LiteralString(String::new());
                    i += 1;
                    while i < raw_lines.len() {
                        let bl = raw_lines[i];
                        let bc = bl.trim();
                        if bc.is_empty() {
                            i += 1;
                            continue;
                        }
                        let bind = indent_of(bl);
                        if bind <= def_indent {
                            break;
                        }
                        if bc.starts_with("return ") {
                            let expr_str = trim(&bc["return ".len()..]);
                            body_expr = parse_expr(&expr_str);
                        } else if bc.starts_with("native ") {
                            let code = trim(&bc["native ".len()..]);
                            let content = if code.starts_with('"') && code.ends_with('"') && code.len() >= 2 {
                                code[1..code.len() - 1].to_string()
                            } else {
                                code
                            };
                            body_expr = Expr::Native(content);
                        } else {
                            if matches!(body_expr, Expr::LiteralString(ref s) if s.is_empty()) {
                                body_expr = parse_expr(bc);
                            }
                        }
                        i += 1;
                    }
                    methods.push(Method {
                        name: mname,
                        return_type: rt,
                        params,
                        body: body_expr,
                        is_static: false,
                        vis: current_vis.clone(),
                    });
                    continue;
                }
                // Field line: name: Type
                if let Some(colon_idx) = c.find(':') {
                    let n = trim(&c[..colon_idx]);
                    let raw_t = &c[colon_idx + 1..];
                    let t_part = if let Some(hash) = raw_t.find('#') {
                        &raw_t[..hash]
                    } else {
                        raw_t
                    };
                    let t = trim(t_part);
                    if !n.is_empty() && !t.is_empty() {
                        fields.push(Field { name: n, ty: t, vis: current_vis.clone() });
                    }
                }
                // Alt field syntax: "name Tipo"
                else {
                    let parts: Vec<&str> = c.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let n = trim(parts.first().unwrap());
                        let t = trim(parts.last().unwrap());
                        if !n.is_empty() && !t.is_empty() {
                            fields.push(Field { name: n, ty: t, vis: current_vis.clone() });
                        }
                    }
                }
                i += 1;
            }
            return Class { name, base, fields, methods, ctor_params, ctor_body, extra_includes: Vec::new() };
        }
        i += 1;
    }
    Class { name, base, fields, methods, ctor_params: None, ctor_body: None, extra_includes: Vec::new() }
}

pub fn parse_all(input: &str) -> Vec<Class> {
    if input.contains("class ") {
        let mut src = input.to_string();
        let mut out: Vec<Class> = Vec::new();
        loop {
            let c = parse(&src);
            if c.name.is_empty() {
                break;
            }
            out.push(c.clone());
            if let Some(pos) = src.find("class ") {
                let rest = &src[pos + 6..];
                if let Some(next_pos) = rest.find("class ") {
                    let cut = pos + 6 + next_pos;
                    src = src[cut..].to_string();
                    continue;
                }
            }
            break;
        }
        out
    } else {
        parse_all_alt(input)
    }
}
pub fn scan_directives(input: &str) -> Directives {
    let mut d = Directives::default();
    for line in input.lines() {
        let c = line.trim();
        if c.is_empty() {
            continue;
        }
        if let Some(rest) = c.strip_prefix("use ") {
            let v = rest.trim().to_string();
            if !v.is_empty() {
                d.uses.push(v);
            }
            continue;
        }
        if let Some(rest) = c.strip_prefix("profile ") {
            let v = rest.trim().to_string();
            if !v.is_empty() {
                d.profiles.push(v);
            }
            continue;
        }
        if let Some(rest) = c.strip_prefix("capability ") {
            let v = rest.trim().to_string();
            if !v.is_empty() {
                d.capabilities.push(v);
            }
            continue;
        }
        if let Some(rest) = c.strip_prefix("entry ") {
            let v = rest.trim().to_string();
            if !v.is_empty() {
                d.entry = Some(v);
            }
            continue;
        }
        if let Some(rest) = c.strip_prefix("run ") {
            let v = rest.trim().to_string();
            if !v.is_empty() {
                d.entry = Some(v);
            }
            continue;
        }
        match c {
            "std" => d.profiles.push("std".to_string()),
            "math" => d.profiles.push("math".to_string()),
            "io" => d.capabilities.push("io".to_string()),
            "string" => d.capabilities.push("string".to_string()),
            "vector" => d.capabilities.push("vector".to_string()),
            "global" => { d.global_base = true; d.profiles.push("std".to_string()); }
            _ => {}
        }
    }
    d
}
fn parse_expr(s: &str) -> Expr {
    let s = s.trim();
    if let Some(idx) = s.find('+') {
        let left = trim(&s[..idx]);
        let right = trim(&s[idx + 1..]);
        return Expr::Concat(Box::new(parse_expr(&left)), Box::new(parse_expr(&right)));
    }
    if s.starts_with("self.") {
        if let Some(paren_pos) = s.find('(') {
            let name = trim(&s["self.".len()..paren_pos]);
            if let Some(end_pos_rel) = s[paren_pos + 1..].find(')') {
                let end_pos = paren_pos + 1 + end_pos_rel;
                let args_src = &s[paren_pos + 1..end_pos];
                let mut args: Vec<Expr> = Vec::new();
                for a in args_src.split(',') {
                    let a = a.trim();
                    if !a.is_empty() {
                        args.push(parse_expr(a));
                    }
                }
                return Expr::SelfCall { name, args };
            }
        }
    }
    if s.starts_with("super().") {
        if let Some(dot_pos) = s.find("super().") {
            let rest = &s[dot_pos + "super().".len()..];
            if let Some(paren_pos_rel) = rest.find('(') {
                let paren_pos = dot_pos + "super().".len() + paren_pos_rel;
                let name = trim(&s[dot_pos + "super().".len()..paren_pos]);
                if let Some(end_pos_rel) = s[paren_pos + 1..].find(')') {
                    let end_pos = paren_pos + 1 + end_pos_rel;
                    let args_src = &s[paren_pos + 1..end_pos];
                    let mut args: Vec<Expr> = Vec::new();
                    for a in args_src.split(',') {
                        let a = a.trim();
                        if !a.is_empty() {
                            args.push(parse_expr(a));
                        }
                    }
                    return Expr::SuperCall { name, args };
                }
            }
        }
    }
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return Expr::LiteralString(s[1..s.len() - 1].to_string());
    }
    if s == "true" {
        return Expr::LiteralBool(true);
    }
    if s == "false" {
        return Expr::LiteralBool(false);
    }
    if let Ok(n) = s.parse::<i64>() {
        return Expr::LiteralInt(n);
    }
    if s.contains('.') {
        if let Ok(f) = s.parse::<f64>() {
            return Expr::LiteralFloat(f);
        }
    }
    if s.starts_with("self.") {
        return Expr::SelfField(s["self.".len()..].to_string());
    }
    if s.chars().all(|ch| ch.is_alphanumeric() || ch == '_') {
        return Expr::SelfField(s.to_string());
    }
    Expr::LiteralString(s.to_string())
}
fn parse_all_alt(input: &str) -> Vec<Class> {
    let raw_lines: Vec<&str> = input.lines().collect();
    let mut out: Vec<Class> = Vec::new();
    let mut i = 0usize;
    while i < raw_lines.len() {
        let line = raw_lines[i];
        let c = line.trim();
        if c.is_empty() {
            i += 1;
            continue;
        }
        if c == "std" || c == "math" || c == "io" || c == "string" || c == "vector" || c == "global" {
            i += 1;
            continue;
        }
        if c.starts_with("use ") || c.starts_with("profile ") || c.starts_with("capability ") || c.starts_with("entry ") || c.starts_with("run ") {
            i += 1;
            continue;
        }
        let class_indent = indent_of(line);
        let name = trim(c);
        let mut fields: Vec<Field> = Vec::new();
        let mut methods: Vec<Method> = Vec::new();
        let base: Option<String> = None;
        let mut ctor_params: Option<Vec<Param>> = None;
        let mut current_vis = Visibility::Public;
        i += 1;
        while i < raw_lines.len() {
            let l = raw_lines[i];
            let t = l.trim();
            if t.is_empty() {
                i += 1;
                continue;
            }
            let ind = indent_of(l);
            if ind <= class_indent {
                break;
            }
            if t == "public:" {
                current_vis = Visibility::Public;
                i += 1;
                continue;
            }
            if t == "private:" {
                current_vis = Visibility::Private;
                i += 1;
                continue;
            }
            if let Some(arrow_idx) = t.find("->") {
                let def_indent = ind;
                let left = trim(&t[..arrow_idx]);
                let mut mname = left.clone();
                let mut params: Vec<Param> = Vec::new();
                if let Some(paren_start) = left.find('(') {
                    mname = trim(&left[..paren_start]);
                    if let Some(paren_end_rel) = left[paren_start + 1..].find(')') {
                        let paren_end = paren_start + 1 + paren_end_rel;
                        let params_part = &left[paren_start + 1..paren_end];
                        for p in params_part.split(',') {
                            let p = p.trim();
                            if p.is_empty() {
                                continue;
                            }
                            let mut kv = p.split(':');
                            let pn = trim(kv.next().unwrap_or(""));
                            let pt = trim(kv.next().unwrap_or(""));
                            if !pn.is_empty() && !pt.is_empty() {
                                params.push(Param { name: pn, ty: pt });
                            }
                        }
                    }
                }
                let mut rt = trim(&t[arrow_idx + 2..]);
                if rt.ends_with(':') {
                    rt.pop();
                    rt = trim(&rt);
                }
                let mut body_expr = Expr::LiteralString(String::new());
                i += 1;
                while i < raw_lines.len() {
                    let bl = raw_lines[i];
                    let bc = bl.trim();
                    if bc.is_empty() {
                        i += 1;
                        continue;
                    }
                    let bind = indent_of(bl);
                    if bind <= def_indent {
                        break;
                    }
                    if bc.starts_with("return ") {
                        let expr_str = trim(&bc["return ".len()..]);
                        body_expr = parse_expr(&expr_str);
                    } else {
                        if matches!(body_expr, Expr::LiteralString(ref s) if s.is_empty()) {
                            body_expr = parse_expr(bc);
                        }
                    }
                    i += 1;
                }
                methods.push(Method {
                    name: mname,
                    return_type: rt,
                    params,
                    body: body_expr,
                    is_static: false,
                    vis: current_vis.clone(),
                });
                continue;
            }
            if let Some(colon_idx) = t.find(':') {
                let n = trim(&t[..colon_idx]);
                let ty = trim(&t[colon_idx + 1..]);
                if !n.is_empty() && !ty.is_empty() {
                    fields.push(Field { name: n, ty, vis: current_vis.clone() });
                }
            } else {
                let parts: Vec<&str> = t.split_whitespace().collect();
                if parts.len() >= 2 {
                    let n = trim(parts.first().unwrap());
                    let ty = trim(parts.last().unwrap());
                    if !n.is_empty() && !ty.is_empty() {
                        fields.push(Field { name: n, ty, vis: current_vis.clone() });
                    }
                }
            }
            i += 1;
        }
        out.push(Class {
            name,
            base,
            fields,
            methods,
            ctor_params,
            ctor_body: None,
            extra_includes: Vec::new(),
        });
    }
    out
}
