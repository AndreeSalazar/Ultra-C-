use crate::Directives;
use crate::{Class, Expr, Field, Method, Param, Visibility};

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

fn parse_expr(s: &str) -> Expr {
    let s = s.trim();
    if s.is_empty() {
        return Expr::LiteralString("".to_string());
    }

    if s.ends_with("()") && s.contains(".upp") {
        let base = s.trim_end_matches("()").trim();
        if let Some(pos) = base.find(".upp") {
            let name = trim(&base[..pos]);
            return Expr::FileCall(name);
        }
    }

    if let Some(rest) = s.strip_prefix("not ") {
        let rest = rest.trim();
        return Expr::UnaryOp("not".to_string(), Box::new(parse_expr(rest)));
    }

    // Binary Operators in order of precedence (lowest first)
    // NOTE: <= must be checked before < if we iterate naively?
    // Actually, the loop order here determines precedence of grouping.
    // If we have "a < b = c", and we check "=" first, we get "(a < b) = c".
    // If we check "<" first, we get "a < (b = c)".
    // Standard precedence: = is lowest (right associative usually, but here left associative by this loop is fine for now if we don't chain).
    // ||, &&, ==, <, +, *

    // The issue seen: "n <= 1" became "(n < "") = 1" or similar mess?
    // "n <= 1" -> parsed as BinaryOp(n, "<=", 1) if <= is matched.
    // But if we match "=" first? "n <= 1". "=" is at index 2? No.
    // "<=" is 2 chars.

    // The previous error in mathutils.cpp:
    // if (((n < std::string("")) = 1))
    // This implies "n <= 1" was parsed as:
    // It found "=" at index 2 (inside <=).
    // Because we added "=" to the first level.
    // And my check `if *op == "=" && next == '=' { false }` prevents `==`, but NOT `<=`.
    // It doesn't check if previous char is `<`.

    let ops = [
        vec!["="],
        vec!["||", "or"],
        vec!["&&", "and"],
        vec!["==", "!="],
        vec!["<=", ">=", "<", ">"],
        vec!["+", "-"],
        vec!["*", "/", "%"],
    ];

    for level in &ops {
        for op in level {
            // Find op not in parens/quotes
            let mut depth = 0;
            let mut in_quote = false;
            let char_indices: Vec<(usize, char)> = s.char_indices().collect();
            let mut k = 0;
            while k < char_indices.len() {
                let (byte_idx, c) = char_indices[k];
                if c == '"' {
                    in_quote = !in_quote;
                } else if !in_quote {
                    if c == '(' {
                        depth += 1;
                    } else if c == ')' {
                        depth -= 1;
                    } else if depth == 0 && s[byte_idx..].starts_with(op) {
                        let is_match = if op.len() == 1 {
                            if k + 1 < char_indices.len() {
                                let (_, next) = char_indices[k + 1];
                                !(next == '=' && (*op == "<" || *op == ">" || *op == "!" || *op == "="))
                            } else {
                                true
                            }
                        } else {
                            true
                        };
                        let is_alpha = op.chars().all(|ch| ch.is_ascii_alphabetic());
                        let left_ok = if is_alpha {
                            if k == 0 {
                                true
                            } else {
                                let (_, prev) = char_indices[k - 1];
                                !(prev.is_ascii_alphanumeric() || prev == '_')
                            }
                        } else {
                            true
                        };
                        let right_ok = if is_alpha {
                            let j = k + op.len();
                            if j >= char_indices.len() {
                                true
                            } else {
                                let (_, next) = char_indices[j];
                                !(next.is_ascii_alphanumeric() || next == '_')
                            }
                        } else {
                            true
                        };

                        let is_safe = if *op == "=" {
                            if k > 0 {
                                let (_, prev) = char_indices[k - 1];
                                !(prev == '<' || prev == '>' || prev == '!' || prev == '=')
                            } else {
                                true
                            }
                        } else {
                            true
                        };

                        if is_match && is_safe && left_ok && right_ok {
                            let left = trim(&s[..byte_idx]);
                            let right = trim(&s[byte_idx + op.len()..]);
                            return Expr::BinaryOp(
                                Box::new(parse_expr(&left)),
                                op.to_string(),
                                Box::new(parse_expr(&right)),
                            );
                        }
                    }
                }
                k += 1;
            }
        }
    }

    // Function call / Method call
    if s.ends_with(')') {
        // Find matching opening parenthesis from the end
        let mut depth = 0;
        let mut open_pos = None;
        let char_indices: Vec<(usize, char)> = s.char_indices().collect();
        for k in (0..char_indices.len()).rev() {
            let (byte_idx, c) = char_indices[k];
            if c == ')' {
                depth += 1;
            } else if c == '(' {
                depth -= 1;
                if depth == 0 {
                    open_pos = Some(byte_idx);
                    break;
                }
            }
        }

        if let Some(paren_pos) = open_pos {
            let name_part = &s[..paren_pos];
            let args_part = &s[paren_pos + 1..s.len() - 1];

            // Check if name_part is valid identifier or dotted (or empty for grouping)
            let is_valid_name = if name_part.is_empty() {
                true // (expr)
            } else {
                // Allow patterns like "self.method" and "super().method" explicitly
                if name_part.starts_with("super().") || name_part.starts_with("self.") {
                    true
                } else {
                    // Otherwise only allow alphanumeric, underscore and dot
                    name_part
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
                }
            };

            if is_valid_name {
                let mut args = Vec::new();
                let mut start = 0;
                let mut depth = 0;
                let mut in_quote = false;
                let a_chars: Vec<char> = args_part.chars().collect();
                for i in 0..a_chars.len() {
                    let c = a_chars[i];
                    if c == '"' {
                        in_quote = !in_quote;
                    } else if !in_quote {
                        if c == '(' {
                            depth += 1;
                        } else if c == ')' {
                            depth -= 1;
                        } else if c == ',' && depth == 0 {
                            args.push(parse_expr(&args_part[start..i]));
                            start = i + 1;
                        }
                    }
                }
                if start < args_part.len() || (start == args_part.len() && !args.is_empty()) {
                    let last = args_part[start..].trim();
                    if !last.is_empty() {
                        args.push(parse_expr(last));
                    }
                }

                if let Some(rest) = name_part.strip_prefix("self.") {
                    let mname = rest.to_string();
                    return Expr::SelfCall { name: mname, args };
                } else if let Some(rest) = name_part.strip_prefix("super().") {
                    let mname = rest.to_string();
                    return Expr::SuperCall { name: mname, args };
                } else {
                    // Includes obj.method(args) and empty name (grouping)
                    return Expr::FunctionCall {
                        name: name_part.to_string(),
                        args,
                    };
                }
            }
        }
    }

    // Literals
    if s.starts_with('"') && s.ends_with('"') {
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
    if let Ok(f) = s.parse::<f64>() {
        return Expr::LiteralFloat(f);
    }

    // Variable / Field
    if let Some(rest) = s.strip_prefix("self.") {
        return Expr::SelfField(rest.to_string());
    }
    // Simple identifier
    Expr::Variable(s.to_string())
}

fn parse_block(lines: &[&str], base_indent: usize) -> (Expr, usize) {
    let mut stmts = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let mut trimmed = line.trim();
        // Remove comments
        if let Some(comment_start) = trimmed.find("//") {
            // ensure not inside string?
            // simplistic approach: split on // if not in string.
            // But for now, let's just assume // starts comment if it's not inside a string logic which is hard to check here easily without full parse.
            // However, `parse_expr` handles strings.
            // Let's just check if it STARTS with //
            if comment_start == 0 {
                i += 1;
                continue;
            }
            // If it has // later, we should probably strip it.
            trimmed = trimmed[..comment_start].trim();
        }

        if trimmed.is_empty() {
            i += 1;
            continue;
        }
        let indent = indent_of(line);
        // Stop body when encountering new method/class definitions
        if trimmed.starts_with("def ") || trimmed.starts_with("class ") {
            break;
        }
        if indent <= base_indent {
            break;
        }

        if let Some(rest) = trimmed.strip_prefix("return ") {
            let val = trim(rest);
            if val.is_empty() {
                stmts.push(Expr::Return(None));
            } else {
                stmts.push(Expr::Return(Some(Box::new(parse_expr(&val)))));
            }
            i += 1;
        } else if let Some(rest) = trimmed.strip_prefix("call ") {
            let name = trim(rest).replace('.', "_");
            // If it's a file call, we might want to map it to the generated method name
            stmts.push(Expr::FileCall(name));
            i += 1;
        } else if let Some(rest) = trimmed.strip_prefix("if ") {
            let cond_str = trim(&rest[..rest.len() - 1]); // assume ends with :
            let cond = parse_expr(&cond_str);
            i += 1;
            let (then_block, consumed) = parse_block(&lines[i..], indent);
            i += consumed;
            let mut elifs: Vec<(Expr, Expr)> = Vec::new();
            while i < lines.len() {
                let nline = lines[i];
                let ntrim = nline.trim();
                if indent_of(nline) == indent && ntrim.starts_with("elif ") {
                    let cstr = trim(&ntrim.strip_prefix("elif ").unwrap()[..ntrim.len() - "elif ".len() - 1]);
                    let cexpr = parse_expr(&cstr);
                    i += 1;
                    let (eblock, econsumed) = parse_block(&lines[i..], indent);
                    i += econsumed;
                    elifs.push((cexpr, eblock));
                } else {
                    break;
                }
            }
            let mut else_block = None;
            if i < lines.len() {
                let next_line = lines[i];
                let next_trim = next_line.trim();
                if indent_of(next_line) == indent && next_trim.starts_with("else:") {
                    i += 1;
                    let (e_block, e_consumed) = parse_block(&lines[i..], indent);
                    else_block = Some(Box::new(e_block));
                    i += e_consumed;
                }
            }
            let mut tail = else_block;
            for (c_if, b_if) in elifs.into_iter().rev() {
                tail = Some(Box::new(Expr::If {
                    cond: Box::new(c_if),
                    then_body: Box::new(b_if),
                    else_body: tail,
                }));
            }
            stmts.push(Expr::If {
                cond: Box::new(cond),
                then_body: Box::new(then_block),
                else_body: tail,
            });
        } else if let Some(rest) = trimmed.strip_prefix("while ") {
            let cond_str = trim(&rest[..rest.len() - 1]);
            let cond = parse_expr(&cond_str);
            i += 1;
            let (body, consumed) = parse_block(&lines[i..], indent);
            i += consumed;
            stmts.push(Expr::While {
                cond: Box::new(cond),
                body: Box::new(body),
            });
        } else if let Some(rest) = trimmed.strip_prefix("native ") {
            let code = trim(rest);
            if code.starts_with("\"\"\"") {
                let mut content = String::new();
                i += 1;
                while i < lines.len() {
                    let nl = lines[i];
                    let nt = nl.trim();
                    if nt.contains("\"\"\"") {
                        if let Some(pos) = nt.find("\"\"\"") {
                            content.push_str(&nt[..pos]);
                        }
                        i += 1;
                        break;
                    } else {
                        content.push_str(nt);
                        content.push('\n');
                        i += 1;
                    }
                }
                stmts.push(Expr::Native(content));
            } else if code.starts_with('"') && !code[1..].contains('"') {
                // Multiline
                let mut content = code[1..].to_string();
                content.push('\n');
                i += 1;
                while i < lines.len() {
                    let nl = lines[i];
                    let nt = nl.trim();
                if let Some(stripped) = nt.strip_suffix('"') {
                        content.push_str(stripped);
                        i += 1;
                        break;
                    } else {
                        content.push_str(nt);
                        content.push('\n');
                        i += 1;
                    }
                }
                stmts.push(Expr::Native(content));
            } else {
                let content = if code.starts_with('"') && code.ends_with('"') && code.len() >= 2 {
                    code[1..code.len() - 1].to_string()
                } else {
                    code
                };
                stmts.push(Expr::Native(content));
                i += 1;
            }
        } else {
            if let Some(rest) = trimmed.strip_prefix("let ") {
                let rest = trim(rest);
                if let Some(colon) = rest.find(':') {
                    let name = trim(&rest[..colon]);
                    let ty_and_val = rest[colon + 1..].to_string();
                    if let Some(eq) = ty_and_val.find('=') {
                        let ty = trim(&ty_and_val[..eq]);
                        let val_str = trim(&ty_and_val[eq + 1..]);
                        stmts.push(Expr::VarDecl {
                            name,
                            ty,
                            value: Some(Box::new(parse_expr(&val_str))),
                        });
                        i += 1;
                        continue;
                    } else {
                        let ty = trim(&ty_and_val);
                        stmts.push(Expr::VarDecl {
                            name,
                            ty,
                            value: None,
                        });
                        i += 1;
                        continue;
                    }
                }
            }
            let mut is_decl = false;

            // Check for := (Type Inference Declaration)
            if let Some(walrus) = trimmed.find(":=") {
                let name = trim(&trimmed[..walrus]);
                let val_str = trim(&trimmed[walrus + 2..]);
                if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    stmts.push(Expr::VarDecl {
                        name,
                        ty: "Auto".to_string(),
                        value: Some(Box::new(parse_expr(&val_str))),
                    });
                    is_decl = true;
                    i += 1;
                }
            }

            // Simple check: colon before equals
            if !is_decl {
                if let Some(colon) = trimmed.find(':') {
                    // Check if it is double colon (scope resolution)
                    let is_double = (colon + 1 < trimmed.len()
                        && trimmed.as_bytes()[colon + 1] == b':')
                        || (colon > 0 && trimmed.as_bytes()[colon - 1] == b':');

                    if !is_double {
                        // Check if name is valid identifier to avoid confusing with method calls containing ::
                        let potential_name = trim(&trimmed[..colon]);
                        let is_ident = !potential_name.is_empty()
                            && potential_name
                                .chars()
                                .all(|c| c.is_ascii_alphanumeric() || c == '_');

                        if is_ident {
                            if let Some(eq) = trimmed.find('=') {
                                if colon < eq && !trimmed[..colon].contains('"') {
                                    let name = potential_name;
                                    let ty = trim(&trimmed[colon + 1..eq]);
                                    let val_str = trim(&trimmed[eq + 1..]);
                                    stmts.push(Expr::VarDecl {
                                        name,
                                        ty,
                                        value: Some(Box::new(parse_expr(&val_str))),
                                    });
                                    is_decl = true;
                                    i += 1;
                                }
                            } else if !trimmed[..colon].contains('"') {
                                let name = potential_name;
                                let ty = trim(&trimmed[colon + 1..]);
                                stmts.push(Expr::VarDecl {
                                    name,
                                    ty,
                                    value: None,
                                });
                                is_decl = true;
                                i += 1;
                            }
                        }
                    }
                }
            }

            if !is_decl {
                stmts.push(parse_expr(trimmed));
                i += 1;
            }
        }
    }
    (Expr::Block(stmts), i)
}

pub fn parse(input: &str) -> Class {
    let raw_lines: Vec<&str> = input.lines().collect();
    let mut i = 0usize;
    let mut name = String::new();
    let mut fields: Vec<Field> = Vec::new();
    let mut methods: Vec<Method> = Vec::new();
    let mut base: Option<String> = None;
    let mut ctor_params: Option<Vec<Param>> = None;
    let mut ctor_body: Option<Expr> = None;

    while i < raw_lines.len() {
        let line = raw_lines[i];
        let content = line.trim();
        if content.is_empty() {
            i += 1;
            continue;
        }

        if let Some(stripped) = content.strip_prefix("class ") {
            let class_indent = indent_of(line);
            let mut cls = stripped.to_string();
            if cls.ends_with(':') {
                cls.pop();
            }
            if let Some(paren) = cls.find('(') {
                name = trim(&cls[..paren]);
                if let Some(end) = cls.find(')') {
                    base = Some(trim(&cls[paren + 1..end]));
                }
            } else if let Some(colon_pos) = cls.find(':') {
                let left = trim(&cls[..colon_pos]);
                let right = trim(&cls[colon_pos + 1..]);
                name = left;
                if !right.is_empty() {
                    base = Some(right.to_string());
                }
            } else {
                name = trim(&cls);
            }
            i += 1;

            let mut current_vis = Visibility::Public;
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

                let paren_pos = c.find('(');
                let colon_pos = c.find(':');

                let is_method_syntax = if let Some(pp) = paren_pos {
                    if let Some(cp) = colon_pos {
                        cp > pp // Colon after paren -> Method (e.g. "run():")
                    } else {
                        false // No colon -> Syntax error or incomplete
                    }
                } else {
                    false // No paren -> Field (e.g. "x: Int")
                };

                if c.starts_with("def ") || is_method_syntax {
                    let def_indent = ind;
                let sig = if let Some(rest) = c.strip_prefix("def ") {
                    rest.to_string()
                } else {
                    c.to_string()
                };

                    // parse signature
                    let mut mname;
                    let mut params = Vec::new();
                    let mut ret_type = "Void".to_string();
                    let mut has_self = false;

                    if let Some(paren) = sig.find('(') {
                        mname = trim(&sig[..paren]).replace('.', "_");
                        if let Some(end) = sig[paren + 1..].find(')') {
                            let end_abs = paren + 1 + end;
                            let args = &sig[paren + 1..end_abs];
                            for arg in args.split(',') {
                                let arg = trim(arg);
                                if arg.is_empty() {
                                    continue;
                                }
                                if arg == "self" {
                                    has_self = true;
                                    continue;
                                }
                                let parts: Vec<&str> = arg.split(':').collect();
                                if parts.len() == 2 {
                                    params.push(Param {
                                        name: trim(parts[0]),
                                        ty: trim(parts[1]),
                                    });
                                }
                            }
                            if let Some(arrow) = sig[end_abs..].find("->") {
                                let r = &sig[end_abs + arrow + 2..];
                                ret_type = trim(r.trim_end_matches(':'));
                            }
                        }
                    } else {
                        mname = trim(sig.trim_end_matches(':')).replace('.', "_");
                    }

                    // Handle 'static' prefix for methods
                    if let Some(rest) = mname.strip_prefix("static ") {
                        mname = trim(rest);
                        // If explicit static, force no self
                        has_self = false;
                    } else if !c.starts_with("def ") {
                        // Implicit self for methods without 'def' and not static
                        has_self = true;
                    }

                    i += 1;
                    // Parse Body using parse_block
                    let (body, consumed) = parse_block(&raw_lines[i..], def_indent);
                    i += consumed;

                    if mname == "__init__" {
                        ctor_params = Some(params);
                        ctor_body = Some(body);
                    } else {
                        methods.push(Method {
                            name: mname,
                            return_type: ret_type,
                            params,
                            body,
                            is_static: !has_self,
                            vis: current_vis.clone(),
                        });
                    }
                } else if let Some(colon) = c.find(':') {
                    // Field
                    let n = trim(&c[..colon]);
                    let mut t_str = c[colon + 1..].to_string();
                    if let Some(hash) = t_str.find('#') {
                        t_str = t_str[..hash].to_string();
                    }
                    let t = trim(&t_str);
                    fields.push(Field {
                        name: n,
                        ty: t,
                        vis: current_vis.clone(),
                    });
                    i += 1;
                } else {
                    i += 1;
                }
            }
            return Class {
                name,
                base,
                fields,
                methods,
                ctor_params,
                ctor_body,
                extra_includes: Vec::new(),
                namespace: None,
                module_version: None,
            };
        }
        i += 1;
    }
    Class {
        name,
        base,
        fields,
        methods,
        ctor_params: None,
        ctor_body: None,
        extra_includes: Vec::new(),
        namespace: None,
        module_version: None,
    }
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
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0usize;
        let mut out: Vec<Class> = Vec::new();
        while i < lines.len() {
            let l = lines[i];
            let c = l.trim();
            if c.is_empty() {
                i += 1;
                continue;
            }
            let ind = indent_of(l);
            if ind == 0 {
                let is_dir = c.starts_with("use ")
                    || c == "std"
                    || c.starts_with("profile ")
                    || c.starts_with("capability ")
                    || c.starts_with("entry ")
                    || c.starts_with("run ")
                    || c == "global";
                if is_dir {
                    i += 1;
                    continue;
                }
                let name = c.to_string();
                let mut fields: Vec<Field> = Vec::new();
                let mut methods: Vec<Method> = Vec::new();
                let mut j = i + 1;
                while j < lines.len() {
                    let lj = lines[j];
                    let cj = lj.trim();
                    if cj.is_empty() {
                        j += 1;
                        continue;
                    }
                    let ij = indent_of(lj);
                    if ij <= ind {
                        break;
                    }
                    if let Some(arr) = cj.find("->") {
                        let mname = trim(&cj[..arr]);
                        let rty = trim(&cj[arr + 2..]);
                        let k = j + 1;
                        let mut body_expr = Expr::LiteralString("".to_string());
                        if k < lines.len() {
                            let bk = lines[k];
                            if indent_of(bk) > ij {
                                let be = parse_expr(bk.trim());
                                body_expr = be;
                                j = k;
                            }
                        }
                        methods.push(Method {
                            name: mname,
                            return_type: rty,
                            params: Vec::new(),
                            body: Expr::Block(vec![Expr::Return(Some(Box::new(body_expr)))]),
                            is_static: true,
                            vis: Visibility::Public,
                        });
                        j += 1;
                    } else {
                        let parts: Vec<&str> = cj.split_whitespace().collect();
                        if parts.len() >= 2 {
                            fields.push(Field {
                                name: trim(parts[0]),
                                ty: trim(parts[1]),
                                vis: Visibility::Public,
                            });
                        }
                        j += 1;
                    }
                }
                out.push(Class {
                    name,
                    base: None,
                    fields,
                    methods,
                    ctor_params: None,
                    ctor_body: None,
                    extra_includes: Vec::new(),
                    namespace: None,
                    module_version: None,
                });
                i = j;
                continue;
            }
            i += 1;
        }
        out
    }
}

pub fn scan_directives(input: &str) -> Directives {
    let mut d = Directives::default();
    for line in input.lines() {
        let mut c = line.trim();
        if let Some(comment_start) = c.find("//") {
            c = c[..comment_start].trim();
        }
        if c.is_empty() {
            continue;
        }
        if let Some(v) = c.strip_prefix("use ") {
            d.uses.push(v.trim().to_string());
        } else if let Some(v) = c.strip_prefix("profile ") {
            d.profiles.push(v.trim().to_string());
        } else if let Some(v) = c.strip_prefix("capability ") {
            d.capabilities.push(v.trim().to_string());
        } else if let Some(v) = c.strip_prefix("entry ") {
            d.entry = Some(v.trim().to_string());
        } else if let Some(v) = c.strip_prefix("run ") {
            d.entry = Some(v.trim().to_string());
        } else if let Some(v) = c.strip_prefix("import ") {
            d.imports.push(v.trim().to_string());
        } else if let Some(v) = c.strip_prefix("namespace ") {
            d.namespace = Some(v.trim().to_string());
        } else {
            match c {
                "std" => d.profiles.push("std".to_string()),
                "math" => d.profiles.push("math".to_string()),
                "io" => d.capabilities.push("io".to_string()),
                "string" => d.capabilities.push("string".to_string()),
                "vector" => d.capabilities.push("vector".to_string()),
                "global" => {
                    d.global_base = true;
                    d.profiles.push("std".to_string());
                }
                _ => {}
            }
        }
    }
    d
}
