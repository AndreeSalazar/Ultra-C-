use crate::{Class, Expr, Field, Method, Param};

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
            if cls.ends_with(':') {
                cls.pop();
            }
            name = trim(&cls);
            i += 1;
            // Parse class block
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
                if c.starts_with("def ") {
                    let def_indent = ind;
                    // def name(self, a: T) -> R:
                    let sig = c["def ".len()..].to_string();
                    let mut mname = String::new();
                    let mut params: Vec<Param> = Vec::new();
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
                        }
                        i += 1;
                    }
                    methods.push(Method {
                        name: mname,
                        return_type: ret_type,
                        params,
                        body: body_expr,
                    });
                    continue;
                }
                // Field line: name: Type
                if let Some(colon_idx) = c.find(':') {
                    let n = trim(&c[..colon_idx]);
                    let t = trim(&c[colon_idx + 1..]);
                    if !n.is_empty() && !t.is_empty() {
                        fields.push(Field { name: n, ty: t });
                    }
                }
                i += 1;
            }
            continue;
        }
        i += 1;
    }
    Class { name, fields, methods }
}

fn parse_expr(s: &str) -> Expr {
    let s = s.trim();
    if let Some(idx) = s.find('+') {
        let left = trim(&s[..idx]);
        let right = trim(&s[idx + 1..]);
        return Expr::Concat(Box::new(parse_expr(&left)), Box::new(parse_expr(&right)));
    }
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return Expr::LiteralString(s[1..s.len() - 1].to_string());
    }
    if s.starts_with("self.") {
        return Expr::SelfField(s["self.".len()..].to_string());
    }
    Expr::LiteralString(s.to_string())
}
