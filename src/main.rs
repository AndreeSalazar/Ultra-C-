use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use std::collections::HashMap;
use std::time::Instant;
use ultracpp::Directives;
use ultracpp::{codegen, parser, tool_detector};

fn read_to_string(path: &str) -> String {
    fs::read_to_string(path).expect("read failed")
}

fn write(path: &str, contents: &str) {
    if let Ok(existing) = fs::read_to_string(path) {
        if existing == contents {
            return;
        }
    }
    fs::write(path, contents).expect("write failed")
}

fn is_safe_rel_path(s: &str) -> bool {
    let st = s.trim();
    if st.is_empty() { return false; }
    if st.starts_with('/') || st.starts_with('\\') { return false; }
    if st.contains("..") { return false; }
    if st.contains(':') { return false; }
    true
}

fn stem(p: &Path) -> String {
    p.file_stem().unwrap().to_string_lossy().to_string()
}

fn is_builtin_ty(t: &str) -> bool {
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
    )
}

fn parse_import_spec(s: &str) -> (String, Option<String>) {
    if let Some(pos) = s.rfind('@') {
        let p = s[..pos].trim();
        let v = s[pos + 1..].trim();
        let mut path = p.to_string();
        if !path.ends_with(".upp") {
            path.push_str(".upp");
        }
        return (
            path,
            if v.is_empty() {
                None
            } else {
                Some(v.to_string())
            },
        );
    }
    let mut path = s.trim().to_string();
    if !path.ends_with(".upp") {
        path.push_str(".upp");
    }
    (path, None)
}

fn type_check(classes: &[ultracpp::Class]) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();
    let mut class_map: HashMap<String, ultracpp::Class> = HashMap::new();
    for c in classes {
        class_map.insert(c.name.clone(), c.clone());
    }
    fn check_expr(
        e: &ultracpp::Expr,
        c: &ultracpp::Class,
        method: &str,
        classes: &HashMap<String, ultracpp::Class>,
        errors: &mut Vec<String>,
    ) {
        match e {
            ultracpp::Expr::SelfField(n) => {
                let ok = c.fields.iter().any(|f| &f.name == n);
                if !ok {
                    errors.push(format!(
                        "Campo desconocido '{}' en {}::{}",
                        n, c.name, method
                    ));
                }
            }
            ultracpp::Expr::SelfCall { name, .. } => {
                let ok = c.methods.iter().any(|m| &m.name == name);
                if !ok {
                    errors.push(format!(
                        "Método desconocido '{}' en {}::{}",
                        name, c.name, method
                    ));
                }
            }
            ultracpp::Expr::FileCall(name) => {
                // call hola.upp -> método local hola_upp() o hola()
                let target_method_upp = format!("{}_upp", name.replace('.', "_"));
                let target_method_std = name.replace('.', "_");
                let has_upp = c.methods.iter().any(|m| m.name == target_method_upp);
                let has_std = c.methods.iter().any(|m| m.name == target_method_std);
                if !has_upp && !has_std {
                    errors.push(format!(
                        "Llamada 'call {}' no resuelta en clase {}. Define {}() o {}(). Ejemplo:\n  def {}():\n    print(\"...\")",
                        name, c.name, target_method_upp, target_method_std, target_method_upp
                    ));
                }
            }
            ultracpp::Expr::SuperCall { name, .. } => {
                if let Some(b) = &c.base {
                    if let Some(base_cls) = classes.get(b) {
                        let ok = base_cls.methods.iter().any(|m| &m.name == name);
                        if !ok {
                            errors.push(format!(
                                "Método '{}' no existe en base {} para {}::{}",
                                name, b, c.name, method
                            ));
                        }
                    }
                }
            }
            ultracpp::Expr::FunctionCall { name, .. } => {
                if name.contains('.') {
                    let parts: Vec<&str> = name.split('.').collect();
                    if parts.len() == 2 {
                        let lhs = parts[0].to_string();
                        let m = parts[1].to_string();
                        // Interpret dotted calls as Class.Method only when LHS looks like a type (starts uppercase) and exists
                        if lhs
                            .chars()
                            .next()
                            .map(|ch| ch.is_uppercase())
                            .unwrap_or(false)
                        {
                            if let Some(cc) = classes.get(&lhs) {
                                let ok = cc.methods.iter().any(|mm| mm.name == m);
                                if !ok {
                                    errors
                                        .push(format!("Método '{}' no existe en clase {}", m, lhs));
                                }
                            } else {
                                errors.push(format!(
                                    "Clase '{}' no encontrada para llamada {}",
                                    lhs, name
                                ));
                            }
                        }
                    }
                }
            }
            ultracpp::Expr::VarDecl { ty, .. } => {
                let t = ty.trim().to_string();
                if t != "Auto" && !is_builtin_ty(&t) && !classes.contains_key(&t) {
                    errors.push(format!(
                        "Tipo '{}' no resuelto en {}::{}",
                        t, c.name, method
                    ));
                }
            }
            ultracpp::Expr::BinaryOp(l, _, r) => {
                check_expr(l, c, method, classes, errors);
                check_expr(r, c, method, classes, errors);
            }
            ultracpp::Expr::Block(stmts) => {
                for s in stmts {
                    check_expr(s, c, method, classes, errors);
                }
            }
            ultracpp::Expr::If {
                cond,
                then_body,
                else_body,
            } => {
                check_expr(cond, c, method, classes, errors);
                check_expr(then_body, c, method, classes, errors);
                if let Some(e2) = else_body {
                    check_expr(e2, c, method, classes, errors);
                }
            }
            ultracpp::Expr::While { cond, body } => {
                check_expr(cond, c, method, classes, errors);
                check_expr(body, c, method, classes, errors);
            }
            ultracpp::Expr::Return(Some(v)) => {
                check_expr(v, c, method, classes, errors);
            }
            ultracpp::Expr::Concat(l, r) => {
                check_expr(l, c, method, classes, errors);
                check_expr(r, c, method, classes, errors);
            }
            _ => {}
        }
    }
    for c in classes {
        for m in &c.methods {
            check_expr(&m.body, c, &m.name, &class_map, &mut errors);
        }
    }
    errors
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: ultracpp <input.upp|input_dir> [outdir] [--compile] [--no-main] [--compiler cl|g++|clang++] [--std c++17|c++20]");
        eprintln!("       ultracpp init <filename> [--template game]");
        std::process::exit(1);
    }
    if args[1] == "init" {
        if args.len() < 3 {
            eprintln!("usage: ultracpp init <filename> [--template game]");
            std::process::exit(1);
        }
        let filename = &args[2];
        let mut template = "default";
        for a in args.iter().skip(3) {
            if a.starts_with("--template") {
                let parts: Vec<&str> = a.split_whitespace().collect();
                if parts.len() > 1 {
                    template = parts[1];
                } else if let Some(pos) = args.iter().position(|x| x == a) {
                    if pos + 1 < args.len() {
                        template = &args[pos + 1];
                    }
                }
            }
        }
        // Handle explicit "--template game" separate args
        let mut idx = 3;
        while idx < args.len() {
            if args[idx] == "--template" && idx + 1 < args.len() {
                template = &args[idx + 1];
            }
            idx += 1;
        }

        let content = if template == "game" {
            r#"global
profile math
capability io

class Entity:
    x: Float
    y: Float
    def __init__(self, x: Float, y: Float):
        return ""

class Player(Entity):
    name: String
    score: Int
    def __init__(self, name: String):
        return super().__init__(0.0, 0.0)
    def move(self, dx: Float, dy: Float) -> Void:
        return "Moving"

class Game:
    running: Bool
    def start(self) -> Void:
        return "Game Started"
    def loop(self) -> Void:
        return "Looping..."

run Game
"#
        } else {
            r#"class Main:
    def hello(self) -> String:
        return "Hello World"
run Main
"#
        };
        write(filename, content);
        println!("Generated {} with template '{}'", filename, template);
        return;
    }
    let input_path = &args[1];
    let mut outdir_root = "dist".to_string();
    let mut do_compile = false;
    let mut no_main = false;
    let mut compiler: Option<String> = None;
    let mut stdver: String = "c++17".to_string();
    let mut bench = false;
    let mut staging = false;
    let mut watch = false;
    let mut unity = false;
    let mut hybrid = false;
    let mut emit_cmake = false;
    let mut release = false;
    let mut gpu_backend: Option<String> = None;
    let mut bridge: Option<String> = None;
    let mut lint = false;
    let mut format = false;
    let mut lint_rust = false;
    let mut smoke = false;
    let mut smoke_compilers = false;
    let mut sanitize: Option<String> = None;
    let mut coverage = false;
    for a in args.iter().skip(2) {
        if a.starts_with("--compile") {
            do_compile = true;
        } else if a.starts_with("--no-main") {
            no_main = true;
        } else if let Some(v) = a.strip_prefix("--compiler ") {
            if !v.is_empty() {
                compiler = Some(v.to_string());
            }
        } else if let Some(v) = a.strip_prefix("--std ") {
            if !v.is_empty() {
                stdver = v.to_string();
            }
        } else if a.starts_with("--bench") {
            bench = true;
        } else if a.starts_with("--staging") {
            staging = true;
        } else if a.starts_with("--watch") {
            watch = true;
        } else if a.starts_with("--unity") {
            unity = true;
        } else if a.starts_with("--release") {
            release = true;
        } else if a.starts_with("--emit cmake") {
            emit_cmake = true;
        } else if let Some(v) = a.strip_prefix("--emit ") {
            match v {
                "unity" => {
                    unity = true;
                    hybrid = false;
                }
                "classic" => {
                    unity = false;
                    hybrid = false;
                }
                "hybrid" => {
                    unity = false;
                    hybrid = true;
                }
                "cmake" => {
                    emit_cmake = true;
                }
                _ => {}
            }
        } else if let Some(v) = a.strip_prefix("--gpu ") {
            if !v.is_empty() {
                gpu_backend = Some(v.to_string());
            }
        } else if let Some(v) = a.strip_prefix("--bridge ") {
            if !v.is_empty() {
                bridge = Some(v.to_string());
            }
        } else if a.starts_with("--lint") {
            lint = true;
        } else if a.starts_with("--format") {
            format = true;
        } else if a.starts_with("--lint-rust") {
            lint_rust = true;
        } else if a.starts_with("--smoke") {
            smoke = true;
        } else if a.starts_with("--smoke-compilers") {
            smoke_compilers = true;
        } else if let Some(v) = a.strip_prefix("--sanitize ") {
            if !v.is_empty() {
                sanitize = Some(v.to_string());
            }
        } else if a.starts_with("--coverage") {
            coverage = true;
        } else if !a.starts_with("--") && outdir_root == "dist" {
            outdir_root = a.to_string();
        }
    }
    // Handle "--emit <mode>" split into two args
    let mut idx = 2usize;
    while idx < args.len() {
        if args[idx] == "--emit" && idx + 1 < args.len() {
            match args[idx + 1].as_str() {
                "unity" => {
                    unity = true;
                    hybrid = false;
                }
                "classic" => {
                    unity = false;
                    hybrid = false;
                }
                "hybrid" => {
                    unity = false;
                    hybrid = true;
                }
                "cmake" => {
                    emit_cmake = true;
                }
                _ => {}
            }
        } else if args[idx] == "--gpu" && idx + 1 < args.len() {
            let v = args[idx + 1].to_string();
            if !v.is_empty() {
                gpu_backend = Some(v);
            }
        } else if args[idx] == "--bridge" && idx + 1 < args.len() {
            let v = args[idx + 1].to_string();
            if !v.is_empty() {
                bridge = Some(v);
            }
        } else if args[idx] == "--sanitize" && idx + 1 < args.len() {
            let v = args[idx + 1].to_string();
            if !v.is_empty() {
                sanitize = Some(v);
            }
        } else if args[idx] == "--coverage" {
            coverage = true;
        } else if args[idx] == "--lint" {
            lint = true;
        } else if args[idx] == "--format" {
            format = true;
        } else if args[idx] == "--lint-rust" {
            lint_rust = true;
        } else if args[idx] == "--smoke" {
            smoke = true;
        } else if args[idx] == "--smoke-compilers" {
            smoke_compilers = true;
        }
        idx += 1;
    }
    if staging {
        outdir_root = "staging".to_string();
    }
    let path_meta = std::fs::metadata(input_path);
    if let Ok(md) = path_meta {
        if md.is_dir() {
            let mut outdir_root_arg = outdir_root.clone();
            for a in args.iter().skip(2) {
                if !a.starts_with("--") && outdir_root_arg == "dist" {
                    outdir_root_arg = a.to_string();
                }
            }
            if Path::new(&outdir_root_arg).is_file() {
                eprintln!("Error: Output root '{}' is a file, cannot create directory inside it.", outdir_root_arg);
                std::process::exit(1);
            }
            let dir_path = Path::new(input_path);
            let base = dir_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_lowercase();
            let dir = Path::new(&outdir_root_arg).join(&base);
            let src_dir = dir.join("src");
            let include_dir = dir.join("include");
            let build_dir = dir.join("build");
            let obj_dir = build_dir.join("obj");
            let bin_dir = build_dir.join("bin");
            fs::create_dir_all(&src_dir).expect("create src dir failed");
            fs::create_dir_all(&include_dir).expect("create include dir failed");
            fs::create_dir_all(&obj_dir).expect("create obj dir failed");
            fs::create_dir_all(&bin_dir).expect("create bin dir failed");
            // Write MSVC PCH source stub
            {
                let pch_src = src_dir.join("pch.cpp");
                let _ = fs::write(&pch_src, "#include \"pch.hpp\"\n");
            }
            // Write precompiled header
            {
                let pch = include_dir.join("pch.hpp");
                let mut p = String::new();
                p.push_str("#pragma once\n");
                p.push_str("#include <iostream>\n");
                p.push_str("#include <string>\n");
                p.push_str("#include <vector>\n");
                p.push_str("#include <memory>\n");
                p.push_str("#include <map>\n");
                p.push_str("#include <list>\n");
                p.push_str("#include <optional>\n");
                p.push_str("#include <thread>\n");
                p.push_str("#include <mutex>\n");
                p.push_str("#include <future>\n");
                p.push_str("#include <atomic>\n");
                p.push_str("#include <filesystem>\n");
                p.push_str("#include <fstream>\n");
                p.push_str("#include <algorithm>\n");
                p.push_str("#include <numeric>\n");
                p.push_str("#include <cmath>\n");
                p.push_str("#include <cstdio>\n");
                p.push_str("#include <functional>\n");
                let _ = fs::write(&pch, p);
            }
            let mut files: Vec<(String, String)> = Vec::new();
            if let Ok(rd) = fs::read_dir(dir_path) {
                for e in rd.flatten() {
                    let p = e.path();
                    if let Some(ext) = p.extension() {
                        if ext.to_string_lossy().eq_ignore_ascii_case("upp") {
                            if let Ok(s) = fs::read_to_string(&p) {
                                files.push((
                                    p.file_name().unwrap().to_string_lossy().to_string(),
                                    s,
                                ));
                            }
                        }
                    }
                }
            }
            if files.is_empty() {
                eprintln!("no .upp files in directory");
                std::process::exit(1);
            }
            files.sort_by(|a, b| {
                let sa = a.0.to_lowercase();
                let sb = b.0.to_lowercase();
                if sa == "principal.upp" && sb != "principal.upp" {
                    std::cmp::Ordering::Less
                } else if sb == "principal.upp" && sa != "principal.upp" {
                    std::cmp::Ordering::Greater
                } else {
                    sa.cmp(&sb)
                }
            });
            let mut merged = Directives::default();
            let mut classes: Vec<ultracpp::Class> = Vec::new();
            let mut names: Vec<String> = Vec::new();
            for (fname, content) in &files {
                let d = parser::scan_directives(content);
                for u in d.uses {
                    if !merged.uses.contains(&u) {
                        merged.uses.push(u);
                    }
                }
                for p in d.profiles {
                    if !merged.profiles.contains(&p) {
                        merged.profiles.push(p);
                    }
                }
                for c in d.capabilities {
                    if !merged.capabilities.contains(&c) {
                        merged.capabilities.push(c);
                    }
                }
                if d.global_base {
                    merged.global_base = true;
                }
                if merged.namespace.is_none() {
                    merged.namespace = d.namespace.clone();
                }
                if fname.to_lowercase() == "principal.upp" {
                    merged.entry = d.entry.or(merged.entry.clone());
                } else if merged.entry.is_none() {
                    merged.entry = d.entry;
                }
                let mut parsed = parser::parse_all(content);
                for cls in parsed.iter_mut() {
                    cls.namespace = d.namespace.clone();
                }
                for cls in parsed.into_iter() {
                    if let Some(pos) = names.iter().position(|n| n == &cls.name) {
                        if fname.to_lowercase() == "principal.upp" {
                            classes[pos] = cls.clone();
                        }
                    } else {
                        names.push(cls.name.clone());
                        classes.push(cls.clone());
                    }
                }
            }
            let mut needs_object_base = merged.global_base;
            let type_errors = type_check(&classes);
            if !type_errors.is_empty() {
                for e in type_errors {
                    eprintln!("{}", e);
                }
                std::process::exit(1);
            }
            for class in classes.iter_mut() {
                if merged.global_base && class.base.is_none() {
                    class.base = Some("Object".to_string());
                    needs_object_base = true;
                }
                class.extra_includes = ultracpp::resolve_includes(&merged);
                if class.namespace.is_none() {
                    class.namespace = merged.namespace.clone();
                }
            }

            if unity {
                // Clean up existing .cpp files to avoid duplicates/conflicts
                if let Ok(rd) = fs::read_dir(&src_dir) {
                    for e in rd.flatten() {
                        let p = e.path();
                        if let Some(ext) = p.extension() {
                            if ext.to_string_lossy().eq_ignore_ascii_case("cpp") {
                                let _ = fs::remove_file(p);
                            }
                        }
                    }
                }
                let mut content = codegen::unity_build(&classes);
                if needs_object_base {
                    // In unity build, Object base should be included or defined.
                    // For simplicity, we assume Object.hpp is not needed if we define it inline or use std types.
                    // But if we really need it, we might need to inject it.
                    // Let's just include "Object.hpp" and hope it exists or generated.
                    // Actually, write_object_base generates Object.hpp/cpp.
                    // We should probably inline Object class in unity build if possible, but let's stick to include for now.
                    content.insert_str(0, "#include \"Object.hpp\"\n");
                }

                if !no_main {
                    let target = select_entry_target(&classes, &merged);
                    let qname = if let Some(ns) = &target.namespace {
                        format!("{}::{}", ns, target.name)
                    } else {
                        target.name.clone()
                    };
                    // Append main
                    content.push_str("\n\n");
                    content.push_str("#ifdef _WIN32\n");
                    content.push_str("#include <windows.h>\n");
                    content.push_str("#endif\n");
                    content.push_str("\nint main() {\n");
                    content.push_str("  #ifdef _WIN32\n");
                    content.push_str("    SetConsoleOutputCP(65001);\n");
                    content.push_str("  #endif\n");
                    content.push_str("  try {\n");
                    if target
                        .methods
                        .iter()
                        .any(|m| m.name == "run" && m.is_static)
                    {
                        content.push_str(&format!("    {}::run();\n", qname));
                    } else {
                        content.push_str(&format!("    {} app;\n", qname));
                        content.push_str("    app.run();\n");
                    }
                    content.push_str("  } catch (const std::exception& e) {\n");
                    content.push_str("    std::cerr << e.what() << std::endl;\n");
                    content.push_str("    return 1;\n");
                    content.push_str("  }\n");
                    content.push_str("  return 0;\n");
                    content.push_str("}\n");
                }

                let all_cpp = src_dir.join("all.cpp");
                write(all_cpp.to_str().unwrap(), &content);
                println!("generated unity build: {}", all_cpp.display());
            } else if hybrid {
                // Hybrid: generate headers only, plus a unity all.cpp for sources
                for class in &classes {
                    let hpp = codegen::header(class);
                    let hpp_path = include_dir.join(format!("{}.hpp", class.name.to_lowercase()));
                    write(hpp_path.to_str().unwrap(), &hpp);
                    println!("generated: {}", hpp_path.display());
                }
                if needs_object_base {
                    write_object_base(&src_dir, &include_dir);
                }
                // Unity source with optional main appended
                let mut content = codegen::unity_build(&classes);
                if !no_main {
                    let target = select_entry_target(&classes, &merged);
                    let qname = if let Some(ns) = &target.namespace {
                        format!("{}::{}", ns, target.name)
                    } else {
                        target.name.clone()
                    };
                    content.push_str("\n\n");
                    content.push_str("#ifdef _WIN32\n#include <windows.h>\n#endif\n");
                    content.push_str("int main() {\n");
                    content.push_str("  #ifdef _WIN32\n    // SetConsoleOutputCP(65001); // disabled for broader toolchain compatibility\n  #endif\n");
                    content.push_str("  try {\n");
                    if target
                        .methods
                        .iter()
                        .any(|m| m.name == "run" && m.is_static)
                    {
                        content.push_str(&format!("    {}::run();\n", qname));
                    } else {
                        content.push_str(&format!("    {} app;\n", qname));
                        content.push_str("    app.run();\n");
                    }
                    content.push_str("  } catch (const std::exception& e) {\n    std::cerr << e.what() << std::endl;\n    return 1;\n  }\n  return 0;\n}\n");
                }
                let all_cpp = src_dir.join("all.cpp");
                write(all_cpp.to_str().unwrap(), &content);
                println!("generated hybrid build: {}", all_cpp.display());
            } else {
                for class in &classes {
                    let hpp = codegen::header(class);
                    let cpp = codegen::source(class);
                    let hpp_path = include_dir.join(format!("{}.hpp", class.name.to_lowercase()));
                    let cpp_path = src_dir.join(format!("{}.cpp", class.name.to_lowercase()));
                    write(hpp_path.to_str().unwrap(), &hpp);
                    write(cpp_path.to_str().unwrap(), &cpp);
                    println!("generated: {}, {}", hpp_path.display(), cpp_path.display());
                }
            }

            if needs_object_base {
                write_object_base(&src_dir, &include_dir);
            }
            if emit_cmake {
                let cmake_path = dir.join("CMakeLists.txt");
                let mut cm = String::new();
                cm.push_str("cmake_minimum_required(VERSION 3.15)\n");
                cm.push_str(&format!("project({} LANGUAGES CXX)\n", base));
                cm.push_str("set(CMAKE_CXX_STANDARD 17)\nset(CMAKE_CXX_STANDARD_REQUIRED ON)\n");
                cm.push_str("include_directories(${CMAKE_SOURCE_DIR}/include)\n");
                // Detect unity source
                let use_unity_src = src_dir.join("all.cpp").exists();
                if use_unity_src {
                    cm.push_str(&format!("add_executable({} src/all.cpp)\n", base));
                } else {
                    cm.push_str(&format!(
                        "file(GLOB SRCS \"src/*.cpp\")\nadd_executable({} ${{SRCS}})\n",
                        base
                    ));
                }
                let _ = fs::write(&cmake_path, cm);
                println!("generated cmake file: {}", cmake_path.display());
            }
            if let Some(_br) = bridge.clone() {
                let exp_h = include_dir.join("exports.hpp");
                let mut h = String::new();
                h.push_str("#pragma once\n");
                h.push_str("extern \"C\" {\n");
                h.push_str("void principal_run();\n");
                h.push_str("void hola_greet();\n");
                h.push_str("}\n");
                write(exp_h.to_str().unwrap(), &h);
                let exp_cpp = src_dir.join("exports.cpp");
                let mut cxx = String::new();
                let target = select_entry_target(&classes, &merged);
                let qname = if let Some(ns) = &target.namespace {
                    format!("{}::{}", ns, target.name)
                } else {
                    target.name.clone()
                };
                let hola_q =
                    if let Some(hc) = classes.iter().find(|x| x.name.to_lowercase() == "hola") {
                        if let Some(ns) = &hc.namespace {
                            format!("{}::{}", ns, hc.name)
                        } else {
                            hc.name.clone()
                        }
                    } else {
                        "Hola".to_string()
                    };
                cxx.push_str(&format!(
                    "#include \"{}.hpp\"\n",
                    target.name.to_lowercase()
                ));
                cxx.push_str(&format!(
                    "#include \"{}.hpp\"\n",
                    hola_q.split("::").last().unwrap().to_lowercase()
                ));
                cxx.push_str("extern \"C\" {\n");
                cxx.push_str(&format!(
                    "void principal_run() {{ {} obj; obj.run(); }}\n",
                    qname
                ));
                cxx.push_str(&format!(
                    "void hola_greet() {{ {} h; h.greet(); }}\n",
                    hola_q
                ));
                cxx.push_str("}\n");
                write(exp_cpp.to_str().unwrap(), &cxx);
                println!("generated: {}, {}", exp_h.display(), exp_cpp.display());
            }
            let exe_name = if cfg!(windows) {
                format!("{}.exe", base)
            } else {
                base.to_string()
            };
            let exe_path = bin_dir.join(&exe_name);

            if !unity && !hybrid && !no_main {
                let main_cpp_path = src_dir.join("entry.cpp");
                let target = select_entry_target(&classes, &merged);
                let main_cpp = demo_main_cpp(target, &target.name.to_lowercase());
                write(main_cpp_path.to_str().unwrap(), &main_cpp);
            }
            let _ = tool_detector::write_build_script_opts(
                &dir,
                &base,
                release,
                gpu_backend.as_deref(),
                bridge.as_deref(),
                sanitize.as_deref(),
                coverage,
            );
            if staging {
                do_compile = false;
            }
            if do_compile {
                match compile_cpp(
                    &dir,
                    &base,
                    compiler.as_deref(),
                    &stdver,
                    release,
                    !no_main,
                    sanitize.as_deref(),
                    coverage,
                ) {
                    Ok(()) => {
                        println!("compiled: {}", exe_path.display());
                        let _ = Command::new(exe_path.to_str().unwrap())
                            .current_dir(&dir)
                            .status();
                    }
                    Err(e) => {
                        eprintln!("skip compile: {}", e);
                    }
                }
            }
            if lint {
                let _ = run_clang_tidy(&dir, &stdver);
            }
            if format {
                let _ = run_clang_format(&dir);
            }
            if lint_rust {
                let _ = run_rust_tools();
            }
            if smoke {
                let _ = run_smoke_tests(input_path);
            }
            if smoke_compilers {
                let _ = run_smoke_tests_compilers(input_path);
            }
            if watch {
                if let Ok(_rd) = fs::read_dir(dir_path) {
                    let mut last = std::time::SystemTime::now();
                    if let Ok(md) = std::fs::metadata(dir_path) {
                        if let Ok(m) = md.modified() {
                            last = m;
                        }
                    }
                    loop {
                        std::thread::sleep(std::time::Duration::from_millis(1200));
                        let mut changed = false;
                        if let Ok(rd2) = fs::read_dir(dir_path) {
                            for e in rd2.flatten() {
                                let p = e.path();
                                if let Some(ext) = p.extension() {
                                    if ext.to_string_lossy().eq_ignore_ascii_case("upp") {
                                        if let Ok(md2) = std::fs::metadata(&p) {
                                            if let Ok(now) = md2.modified() {
                                                if now > last {
                                                    changed = true;
                                                    last = now;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if changed {
                            let exe = std::env::current_exe().unwrap();
                            let args_next: Vec<String> = std::env::args().skip(1).collect();
                            let _ = Command::new(exe).args(&args_next).status();
                        }
                    }
                }
            }
            return;
        }
    }
    let src = read_to_string(input_path);
    let t0 = Instant::now();
    let directives = parser::scan_directives(&src);
    let mut classes = parser::parse_all(&src);
    let mut import_cache: Vec<(String, String, String)> = Vec::new();
    if !directives.imports.is_empty() {
        let base_dir = Path::new(input_path).parent().unwrap_or(Path::new("."));
        for imp in &directives.imports {
            let (imp_path, ver) = parse_import_spec(imp);
            if !is_safe_rel_path(&imp_path) {
                eprintln!("import path rejected (unsafe): {}", imp_path);
                continue;
            }
            let ip = base_dir.join(&imp_path);
            if ip.exists() {
                if let Ok(s) = fs::read_to_string(&ip) {
                    let more = parser::parse_all(&s);
                    let di = parser::scan_directives(&s);
                    for mut m in more {
                        m.namespace = di.namespace.clone();
                        m.module_version = ver.clone();
                        if let Some(pos) = classes.iter().position(|c| c.name == m.name) {
                            classes[pos] = m.clone();
                        } else {
                            classes.push(m.clone());
                        }
                    }
                    let stem = Path::new(&imp_path)
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                    let v = ver.clone().unwrap_or_else(|| "latest".to_string());
                    import_cache.push((stem, v, s));
                }
            }
        }
    }
    let parse_ms = t0.elapsed().as_millis();
    let base = stem(Path::new(input_path)).to_lowercase();
    let dir = Path::new(&outdir_root).join(&base);
    let src_dir = dir.join("src");
    let include_dir = dir.join("include");
    let build_dir = dir.join("build");
    let obj_dir = build_dir.join("obj");
    let bin_dir = build_dir.join("bin");

    fs::create_dir_all(&src_dir).expect("create src dir failed");
    fs::create_dir_all(&include_dir).expect("create include dir failed");
    fs::create_dir_all(&obj_dir).expect("create obj dir failed");
    fs::create_dir_all(&bin_dir).expect("create bin dir failed");
    {
        let pch_src = src_dir.join("pch.cpp");
        let _ = fs::write(&pch_src, "#include \"pch.hpp\"\n");
    }
    {
        let pch = include_dir.join("pch.hpp");
        let mut p = String::new();
        p.push_str("#pragma once\n");
        p.push_str("#include <iostream>\n");
        p.push_str("#include <string>\n");
        p.push_str("#include <vector>\n");
        p.push_str("#include <memory>\n");
        p.push_str("#include <map>\n");
        p.push_str("#include <list>\n");
        p.push_str("#include <optional>\n");
        p.push_str("#include <thread>\n");
        p.push_str("#include <mutex>\n");
        p.push_str("#include <future>\n");
        p.push_str("#include <atomic>\n");
        p.push_str("#include <filesystem>\n");
        p.push_str("#include <fstream>\n");
        p.push_str("#include <algorithm>\n");
        p.push_str("#include <numeric>\n");
        p.push_str("#include <cmath>\n");
        p.push_str("#include <cstdio>\n");
        p.push_str("#include <functional>\n");
        let _ = fs::write(&pch, p);
    }
    if !import_cache.is_empty() {
        let cache_dir = build_dir.join("cache");
        fs::create_dir_all(&cache_dir).expect("create cache dir failed");
        for (name, ver, contents) in &import_cache {
            let fname = format!("{}@{}.upp", name.to_lowercase(), ver);
            let fpath = cache_dir.join(fname);
            write(fpath.to_str().unwrap(), contents);
        }
    }

    // Write README.md
    let readme_content = format!(
        r#"# Estructura del Proyecto
 
 Este proyecto ha sido generado con una estructura organizada:
 
 - **/src**: Contiene los archivos de código fuente C++ (.cpp).
 - **/include**: Contiene los archivos de cabecera (.hpp).
 - **/build**: Directorio para archivos generados durante la compilación.
   - **/obj**: Archivos objeto (.obj) temporales.
   - **/bin**: Ejecutable final del juego.
 
 Para compilar, ejecute `build.bat` (Windows) o `./build.sh` (Linux/Mac).
 
 ## Modos y Flags útiles
 - `--emit classic|unity|hybrid` selecciona el modo de emisión.
 - `--release` habilita optimizaciones: `/O2` (MSVC) o `-O2` (g++/clang++).
 - `--watch` recompila cuando cambian los `.upp` (modo carpeta).
 - `--bench`/`--staging` generan reporte de métricas en `build/report.json`.
 
 ## Puentes (Bridge) — DLL/SO para integraciones
 Si se generó con `--bridge`, también se construye una librería compartida:
 - Windows: `build/bin/{}.dll`
 - Linux: `build/bin/lib{}.so`
 
 Ejemplos de uso:
 
 ### Python (ctypes)
 ```python
 import ctypes, os, sys
 libpath = os.path.join('build', 'bin', '{}.dll' if sys.platform.startswith('win') else 'lib{}.so')
 lib = ctypes.CDLL(libpath)
 lib.principal_run()
 lib.hola_greet()
 ```
 
 ### Node.js (ffi-napi o N-API)
 ```js
 const os = require('os');
 const path = require('path');
 const ffi = require('ffi-napi');
 const libname = os.platform().startsWith('win') ? '{}.dll' : 'lib{}.so';
 const libpath = path.join('build', 'bin', libname);
 const lib = ffi.Library(libpath, {{
   'principal_run': [ 'void', [] ],
   'hola_greet': [ 'void', [] ]
 }});
 lib.principal_run();
 lib.hola_greet();
 ```
"#,
        base, base, base, base, base, base
    );
    let _ = fs::write(dir.join("README.md"), readme_content);

    let mut needs_object_base = directives.global_base;
    let type_errors = type_check(&classes);
    if !type_errors.is_empty() {
        for e in type_errors {
            eprintln!("{}", e);
        }
        std::process::exit(1);
    }
    if classes.is_empty() {
        let class = parser::parse(&src);
        let mut class = class;
        if directives.global_base && class.base.is_none() {
            class.base = Some("Object".to_string());
            needs_object_base = true;
        }
        class.extra_includes = ultracpp::resolve_includes(&directives);
        class.namespace = directives.namespace.clone();
        let hpp = codegen::header(&class);
        let cpp = codegen::source(&class);
        let hpp_path = include_dir.join(format!("{}.hpp", class.name.to_lowercase()));
        let cpp_path = src_dir.join(format!("{}.cpp", class.name.to_lowercase()));
        write(hpp_path.to_str().unwrap(), &hpp);
        write(cpp_path.to_str().unwrap(), &cpp);
        println!("generated: {}, {}", hpp_path.display(), cpp_path.display());
        let exe_name = if cfg!(windows) {
            format!("{}.exe", base)
        } else {
            base.to_string()
        };
        let exe_path = bin_dir.join(&exe_name);
        let main_cpp_path = src_dir.join("main.cpp");
        if !no_main {
            let main_cpp = demo_main_cpp(&class, &class.name.to_lowercase());
            write(main_cpp_path.to_str().unwrap(), &main_cpp);
        }
        if needs_object_base {
            write_object_base(&src_dir, &include_dir);
        }
        let _ = tool_detector::write_build_script_opts(
            &dir,
            &base,
            false,
            None,
            None,
            sanitize.as_deref(),
            coverage,
        );
        if staging {
            do_compile = false;
        }
        let mut compiled = false;
        if no_main && !do_compile {
            if bench || staging {
                let report_path = dir.join("report.json");
                let mut report = String::new();
                report.push('{');
                report.push_str(&format!("\"classes\":{},", 1));
                report.push_str(&format!("\"fields\":{},", class.fields.len()));
                report.push_str(&format!("\"methods\":{},", class.methods.len()));
                report.push_str(&format!("\"includes\":{},", class.extra_includes.len()));
                report.push_str(&format!("\"parse_ms\":{},", parse_ms));
                report.push_str(&format!("\"codegen_ms\":{}", 0));
                report.push('}');
                let _ = fs::write(report_path, report);
            }
            return;
        }
        let t1 = Instant::now();
        match compile_cpp(
            &dir,
            &base,
            compiler.as_deref(),
            &stdver,
            release,
            !no_main,
            sanitize.as_deref(),
            coverage,
        ) {
            Ok(()) => {
                println!("compiled: {}", exe_path.display());
                compiled = true;
                let _ = Command::new(exe_path.to_str().unwrap())
                    .current_dir(&dir)
                    .status();
            }
            Err(e) => {
                eprintln!("skip compile: {}", e);
            }
        }
        if lint {
            let _ = run_clang_tidy(&dir, &stdver);
        }
        if format {
            let _ = run_clang_format(&dir);
        }
        if lint_rust {
            let _ = run_rust_tools();
        }
        if smoke {
            let _ = run_smoke_tests(input_path);
        }
        if smoke_compilers {
            let _ = run_smoke_tests_compilers(input_path);
        }
        if bench || staging {
            let report_path = dir.join("report.json");
            let mut report = String::new();
            report.push('{');
            report.push_str(&format!("\"classes\":{},", 1));
            report.push_str(&format!("\"fields\":{},", class.fields.len()));
            report.push_str(&format!("\"methods\":{},", class.methods.len()));
            report.push_str(&format!("\"includes\":{},", class.extra_includes.len()));
            report.push_str(&format!("\"parse_ms\":{},", parse_ms));
            report.push_str(&format!("\"codegen_ms\":{},", t1.elapsed().as_millis()));
            report.push_str(&format!(
                "\"compiled\":{}",
                if compiled { "true" } else { "false" }
            ));
            report.push('}');
            let _ = fs::write(report_path, report);
        }
        return;
    }
    let t1 = Instant::now();
    let mut fixed_classes: Vec<ultracpp::Class> = Vec::new();
    for class in &classes {
        let mut cl = class.clone();
        if directives.global_base && cl.base.is_none() {
            cl.base = Some("Object".to_string());
            needs_object_base = true;
        }
        cl.extra_includes = ultracpp::resolve_includes(&directives);
        if cl.namespace.is_none() {
            cl.namespace = directives.namespace.clone();
        }
        let hpp = codegen::header(&cl);
        let cpp = codegen::source(&cl);
        let hpp_path = include_dir.join(format!("{}.hpp", cl.name.to_lowercase()));
        let cpp_path = src_dir.join(format!("{}.cpp", cl.name.to_lowercase()));
        write(hpp_path.to_str().unwrap(), &hpp);
        write(cpp_path.to_str().unwrap(), &cpp);
        println!("generated: {}, {}", hpp_path.display(), cpp_path.display());
        fixed_classes.push(cl);
    }
    if needs_object_base {
        write_object_base(&src_dir, &include_dir);
    }

    let exe_name = if cfg!(windows) {
        format!("{}.exe", base)
    } else {
        base.to_string()
    };
    let exe_path = bin_dir.join(&exe_name);
    let main_cpp_path = src_dir.join("entry.cpp");
    if !no_main && !unity {
        let target = select_entry_target(&fixed_classes, &directives);
        let main_cpp = demo_main_cpp(target, &target.name.to_lowercase());
        write(main_cpp_path.to_str().unwrap(), &main_cpp);
    }
    let _ = tool_detector::write_build_script_opts(
        &dir,
        &base,
        release,
        gpu_backend.as_deref(),
        bridge.as_deref(),
        sanitize.as_deref(),
        coverage,
    );
    if staging {
        do_compile = false;
    }
    if no_main && !do_compile {
        if bench || staging {
            let mut fields = 0usize;
            let mut methods = 0usize;
            for c in &classes {
                fields += c.fields.len();
                methods += c.methods.len();
            }
            let report_path = dir.join("report.json");
            let mut report = String::new();
            report.push('{');
            report.push_str(&format!("\"classes\":{},", classes.len()));
            report.push_str(&format!("\"fields\":{},", fields));
            report.push_str(&format!("\"methods\":{},", methods));
            report.push_str(&format!(
                "\"includes\":{},",
                ultracpp::resolve_includes(&directives).len()
            ));
            report.push_str(&format!("\"parse_ms\":{},", parse_ms));
            report.push_str(&format!("\"codegen_ms\":{}", t1.elapsed().as_millis()));
            report.push('}');
            let _ = fs::write(report_path, report);
        }
        return;
    }
    let t2 = Instant::now();
    let mut compiled = false;
    match compile_cpp(
        &dir,
        &base,
        compiler.as_deref(),
        &stdver,
        release,
        !no_main,
        sanitize.as_deref(),
        coverage,
    ) {
        Ok(()) => {
            println!("compiled: {}", exe_path.display());
            compiled = true;
            let _ = Command::new(exe_path.to_str().unwrap())
                .current_dir(&dir)
                .status();
        }
        Err(e) => {
            eprintln!("skip compile: {}", e);
        }
    }
    if lint {
        let _ = run_clang_tidy(&dir, &stdver);
    }
    if format {
        let _ = run_clang_format(&dir);
    }
    if lint_rust {
        let _ = run_rust_tools();
    }
    if smoke {
        let _ = run_smoke_tests(input_path);
    }
    if smoke_compilers {
        let _ = run_smoke_tests_compilers(input_path);
    }
    if bench || staging {
        let mut fields = 0usize;
        let mut methods = 0usize;
        for c in &classes {
            fields += c.fields.len();
            methods += c.methods.len();
        }
        let report_path = dir.join("report.json");
        let mut report = String::new();
        report.push('{');
        report.push_str(&format!("\"classes\":{},", classes.len()));
        report.push_str(&format!("\"fields\":{},", fields));
        report.push_str(&format!("\"methods\":{},", methods));
        report.push_str(&format!(
            "\"includes\":{},",
            ultracpp::resolve_includes(&directives).len()
        ));
        report.push_str(&format!("\"parse_ms\":{},", parse_ms));
        report.push_str(&format!("\"codegen_ms\":{},", t1.elapsed().as_millis()));
        report.push_str(&format!("\"compile_ms\":{},", t2.elapsed().as_millis()));
        report.push_str(&format!(
            "\"compiled\":{}",
            if compiled { "true" } else { "false" }
        ));
        report.push('}');
        let _ = fs::write(report_path, report);
    }
    if watch {
        if let Ok(md) = std::fs::metadata(input_path) {
            if let Ok(mut last) = md.modified() {
                loop {
                    std::thread::sleep(std::time::Duration::from_millis(1200));
                    if let Ok(md2) = std::fs::metadata(input_path) {
                        if let Ok(now) = md2.modified() {
                            if now > last {
                                last = now;
                                let exe = std::env::current_exe().unwrap();
                                let args_next: Vec<String> = std::env::args().skip(1).collect();
                                let _ = Command::new(exe).args(&args_next).status();
                            }
                        }
                    }
                }
            }
        }
    }
}

fn demo_main_cpp(class: &ultracpp::Class, base: &str) -> String {
    let ctor_args = if let Some(ps) = &class.ctor_params {
        let mut a: Vec<String> = Vec::new();
        for p in ps {
            a.push(default_cpp_value(&p.ty));
        }
        a.join(", ")
    } else {
        let mut a: Vec<String> = Vec::new();
        for f in &class.fields {
            a.push(default_cpp_value(&f.ty));
        }
        a.join(", ")
    };
    let mut s = String::new();
    s.push_str("#include \"pch.hpp\"\n");
    s.push_str(&format!("#include \"{}.hpp\"\n", base));
    s.push_str("#include <iostream>\n");
    s.push_str("#ifdef _WIN32\n");
    s.push_str("#include <windows.h>\n");
    s.push_str("#endif\n");
    s.push_str("int main() {\n");
    s.push_str("  #ifdef _WIN32\n");
    s.push_str("    SetConsoleOutputCP(65001);\n");
    s.push_str("  #endif\n");

    // Check for explicit entry points first: run_loop, start, main, run, hola_upp
    let entry_method = class.methods.iter().find(|m| {
        m.name == "run_loop"
            || m.name == "start"
            || m.name == "main"
            || m.name == "run"
            || m.name == "hola_upp"
    });

    if let Some(m) = entry_method {
        let qname = if let Some(ns) = &class.namespace {
            format!("{}::{}", ns, class.name)
        } else {
            class.name.clone()
        };
        if !m.is_static {
            if ctor_args.is_empty() {
                s.push_str(&format!("  {} obj{{}};\n", qname));
            } else {
                s.push_str(&format!("  {} obj({});\n", qname, ctor_args));
            }
            s.push_str(&format!("  obj.{}(", m.name));
        } else {
            s.push_str(&format!("  {}::{}(", qname, m.name));
        }

        let mut call_args: Vec<String> = Vec::new();
        for p in &m.params {
            call_args.push(default_cpp_value(&p.ty));
        }
        s.push_str(&call_args.join(", "));
        s.push_str(");\n");
    } else {
        // Fallback: Prefer a method that returns String
        let maybe_m = class.methods.iter().find(|m| m.return_type == "String");
        if let Some(m) = maybe_m {
            // Create object only if method is instance
            let qname = if let Some(ns) = &class.namespace {
                format!("{}::{}", ns, class.name)
            } else {
                class.name.clone()
            };
            if !m.is_static {
                if ctor_args.is_empty() {
                    s.push_str(&format!("  {} obj{{}};\n", qname));
                } else {
                    s.push_str(&format!("  {} obj({});\n", qname, ctor_args));
                }
            }
            if m.is_static {
                s.push_str(&format!("  std::cout << {}::{}(", qname, m.name));
            } else {
                s.push_str(&format!("  std::cout << obj.{}(", m.name));
            }
            let mut call_args: Vec<String> = Vec::new();
            for p in &m.params {
                call_args.push(default_cpp_value(&p.ty));
            }
            s.push_str(&call_args.join(", "));
            s.push_str(") << std::endl;\n");
        }
    }
    s.push_str("  return 0;\n}\n");
    s
}

fn default_cpp_value(ty: &str) -> String {
    match ty {
        "String" => "std::string(\"Mundo\")".to_string(),
        "Int" | "int" => "0".to_string(),
        "Bool" | "bool" => "false".to_string(),
        "Float" | "float" => "0.0f".to_string(),
        "Double" | "double" => "0.0".to_string(),
        _ => "{}".to_string(),
    }
}

fn compile_cpp(
    dir: &Path,
    base: &str,
    compiler: Option<&str>,
    stdver: &str,
    release: bool,
    _has_main: bool,
    san: Option<&str>,
    coverage: bool,
) -> Result<(), String> {
    let exe_name = if cfg!(windows) {
        format!("{}.exe", base)
    } else {
        base.to_string()
    };

    if compiler.is_none() || compiler == Some("cl") {
        match tool_detector::compile_with_msvc(dir, base) {
            Ok(()) => return Ok(()),
            Err(e) => eprintln!("MSVC compile failed: {}", e),
        }
    }
    // Try other compilers if requested explicitly
    if let Some(comp) = compiler {
        let src_dir = dir.join("src");
        match comp {
            "g++" => {
                let mut files: Vec<String> = Vec::new();
                if let Ok(rd) = fs::read_dir(&src_dir) {
                    for e in rd.flatten() {
                        let p = e.path();
                        if let Some(ext) = p.extension() {
                            if ext.to_string_lossy().eq_ignore_ascii_case("cpp") {
                                if let Some(n) = p.file_name() {
                                    files.push(format!("src/{}", n.to_string_lossy()));
                                }
                            }
                        }
                    }
                }
                if files.is_empty() {
                    return Err("no .cpp files to compile".to_string());
                }
                let mut args: Vec<String> = Vec::new();
                args.push(format!("-std={}", stdver));
                args.push("-Wall".to_string());
                args.push("-Wextra".to_string());
                args.push("-Werror".to_string());
                if release {
                    args.push("-O2".to_string());
                }
                if let Some(s) = san {
                    match s {
                        "asan" => args.push("-fsanitize=address".to_string()),
                        "ubsan" => args.push("-fsanitize=undefined".to_string()),
                        "tsan" => args.push("-fsanitize=thread".to_string()),
                        _ => {}
                    }
                }
                if coverage {
                    args.push("-fprofile-arcs".to_string());
                    args.push("-ftest-coverage".to_string());
                }
                args.push("-I".to_string());
                args.push("include".to_string());
                for f in files {
                    args.push(f);
                }
                args.push("-o".to_string());
                args.push(format!("build/bin/{}", exe_name));
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                if let Ok(out) = Command::new("g++")
                    .current_dir(dir)
                    .args(&arg_refs)
                    .output()
                {
                    if out.status.success() {
                        return Ok(());
                    }
                }
            }
            "clang++" => {
                let mut files: Vec<String> = Vec::new();
                if let Ok(rd) = fs::read_dir(&src_dir) {
                    for e in rd.flatten() {
                        let p = e.path();
                        if let Some(ext) = p.extension() {
                            if ext.to_string_lossy().eq_ignore_ascii_case("cpp") {
                                if let Some(n) = p.file_name() {
                                    files.push(format!("src/{}", n.to_string_lossy()));
                                }
                            }
                        }
                    }
                }
                if files.is_empty() {
                    return Err("no .cpp files to compile".to_string());
                }
                let mut args: Vec<String> = Vec::new();
                args.push(format!("-std={}", stdver));
                args.push("-Wall".to_string());
                args.push("-Wextra".to_string());
                args.push("-Werror".to_string());
                if release {
                    args.push("-O2".to_string());
                }
                if let Some(s) = san {
                    match s {
                        "asan" => args.push("-fsanitize=address".to_string()),
                        "ubsan" => args.push("-fsanitize=undefined".to_string()),
                        "tsan" => args.push("-fsanitize=thread".to_string()),
                        _ => {}
                    }
                }
                if coverage {
                    args.push("-fprofile-arcs".to_string());
                    args.push("-ftest-coverage".to_string());
                }
                args.push("-I".to_string());
                args.push("include".to_string());
                for f in files {
                    args.push(f);
                }
                args.push("-o".to_string());
                args.push(format!("build/bin/{}", exe_name));
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                if let Ok(out) = Command::new("clang++")
                    .current_dir(dir)
                    .args(&arg_refs)
                    .output()
                {
                    if out.status.success() {
                        return Ok(());
                    }
                }
            }
            _ => {}
        }
    }
    Err("no available compiler completed successfully".to_string())
}

fn select_entry_target<'a>(classes: &'a [ultracpp::Class], d: &Directives) -> &'a ultracpp::Class {
    if let Some(ref name) = d.entry {
        if let Some(c) = classes.iter().find(|c| c.name == *name) {
            return c;
        }
    }
    classes.last().expect("no classes parsed")
}

fn write_object_base(src_dir: &Path, include_dir: &Path) {
    let hpp = r#"#pragma once
class Object {
public:
  virtual ~Object() = default;
};
"#;
    let cpp = r#"#include "object.hpp"
"#;
    let hpp_path = include_dir.join("object.hpp");
    let cpp_path = src_dir.join("object.cpp");
    let _ = fs::write(&hpp_path, hpp);
    let _ = fs::write(&cpp_path, cpp);
}

fn run_clang_tidy(dir: &Path, stdver: &str) -> bool {
    let src_dir = dir.join("src");
    let mut files: Vec<String> = Vec::new();
    let cfg_project = Path::new(".").join(".clang-tidy");
    let cfg_local = dir.join(".clang-tidy");
    let cfg_path = if cfg_local.exists() { Some(cfg_local) } else if cfg_project.exists() { Some(cfg_project) } else { None };
    if let Ok(rd) = fs::read_dir(&src_dir) {
        for e in rd.flatten() {
            let p = e.path();
            if let Some(ext) = p.extension() {
                if ext.to_string_lossy().eq_ignore_ascii_case("cpp") {
                    if let Some(n) = p.file_name() {
                        files.push(format!("src/{}", n.to_string_lossy()));
                    }
                }
            }
        }
    }
    if files.is_empty() {
        return false;
    }
    let mut ok = true;
    for f in &files {
        let mut args: Vec<String> = Vec::new();
        args.push(f.clone());
        args.push(format!("-extra-arg=-std={}", stdver));
        args.push("-extra-arg=-Iinclude".to_string());
        if let Some(cfg) = &cfg_path {
            args.push(format!("-config-file={}", cfg.display()));
        } else {
            args.push("-checks=*".to_string());
            args.push("-warnings-as-errors=*".to_string());
        }
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        if let Ok(out) = Command::new("clang-tidy").current_dir(dir).args(&arg_refs).output() {
            if !out.status.success() {
                ok = false;
            }
            let stdout = String::from_utf8_lossy(&out.stdout);
            let stderr = String::from_utf8_lossy(&out.stderr);
            if !stdout.is_empty() {
                println!("{}", stdout);
            }
            if !stderr.is_empty() {
                eprintln!("{}", stderr);
            }
        } else {
            ok = false;
            break;
        }
    }
    ok
}

fn run_clang_format(dir: &Path) -> bool {
    let src_dir = dir.join("src");
    let include_dir = dir.join("include");
    let mut files: Vec<String> = Vec::new();
    for d in [&src_dir, &include_dir] {
        if let Ok(rd) = fs::read_dir(d) {
            for e in rd.flatten() {
                let p = e.path();
                if let Some(ext) = p.extension() {
                    let ext_l = ext.to_string_lossy().to_lowercase();
                    if ext_l == "cpp" || ext_l == "hpp" || ext_l == "h" || ext_l == "cc" {
                        files.push(p.strip_prefix(dir).unwrap().to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    if files.is_empty() {
        return false;
    }
    let mut ok = true;
    for f in files {
        if let Ok(out) = Command::new("clang-format")
            .current_dir(dir)
            .args(["-i", f.as_str()])
            .output()
        {
            if !out.status.success() {
                ok = false;
            }
        } else {
            ok = false;
            break;
        }
    }
    ok
}

fn run_rust_tools() -> bool {
    let cwd = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
    let mut ok = true;
    if let Ok(out) = Command::new("cargo")
        .current_dir(&cwd)
        .args(["clippy", "--all-targets", "--all-features"])
        .output()
    {
        if !out.status.success() {
            ok = false;
        }
        let s = String::from_utf8_lossy(&out.stdout);
        if !s.is_empty() {
            println!("{}", s);
        }
        let e = String::from_utf8_lossy(&out.stderr);
        if !e.is_empty() {
            eprintln!("{}", e);
        }
    } else {
        ok = false;
    }
    if let Ok(out) = Command::new("cargo")
        .current_dir(&cwd)
        .args(["fmt", "--all"])
        .output()
    {
        if !out.status.success() {
            ok = false;
        }
    } else {
        ok = false;
    }
    ok
}

fn run_smoke_tests(input_dir: &str) -> bool {
    let exe = std::env::current_exe().unwrap();
    let modes = ["classic", "unity", "hybrid"];
    let mut ok = true;
    for m in &modes {
        let mut args: Vec<String> = Vec::new();
        args.push("combine".to_string());
        args.push(input_dir.to_string());
        args.push("--emit".to_string());
        args.push(m.to_string());
        args.push("--compile".to_string());
        if let Ok(st) = Command::new(&exe).args(&args).status() {
            if !st.success() {
                ok = false;
            }
        } else {
            ok = false;
        }
    }
    ok
}

fn run_smoke_tests_compilers(input_dir: &str) -> bool {
    let exe = std::env::current_exe().unwrap();
    let modes = ["classic", "unity", "hybrid"];
    let comps = ["cl", "g++", "clang++"];
    let mut ok = true;
    for comp in &comps {
        for m in &modes {
            let mut args: Vec<String> = Vec::new();
            args.push("combine".to_string());
            args.push(input_dir.to_string());
            args.push("--emit".to_string());
            args.push(m.to_string());
            args.push("--compiler".to_string());
            args.push(comp.to_string());
            args.push("--compile".to_string());
            if let Ok(st) = Command::new(&exe).args(&args).status() {
                if !st.success() {
                    ok = false;
                }
            } else {
                ok = false;
            }
        }
    }
    ok
}
