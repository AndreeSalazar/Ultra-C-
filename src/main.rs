use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use ultracpp::{codegen, parser, tool_detector};
use ultracpp::Directives;
use std::time::Instant;

fn read_to_string(path: &str) -> String {
    fs::read_to_string(path).expect("read failed")
}

fn write(path: &str, contents: &str) {
    fs::write(path, contents).expect("write failed")
}

fn stem(p: &Path) -> String {
    p.file_stem().unwrap().to_string_lossy().to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: ultracpp <input.upp> [outdir] [--compile] [--no-main] [--compiler cl|g++|clang++] [--std c++17|c++20]");
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
    for a in args.iter().skip(2) {
        if a.starts_with("--compile") {
            do_compile = true;
        } else if a.starts_with("--no-main") {
            no_main = true;
        } else if a.starts_with("--compiler ") {
            let v = a["--compiler ".len()..].to_string();
            if !v.is_empty() {
                compiler = Some(v);
            }
        } else if a.starts_with("--std ") {
            let v = a["--std ".len()..].to_string();
            if !v.is_empty() {
                stdver = v;
            }
        } else if a.starts_with("--bench") {
            bench = true;
        } else if a.starts_with("--staging") {
            staging = true;
        } else if !a.starts_with("--") && outdir_root == "dist" {
            outdir_root = a.to_string();
        }
    }
    if staging {
        outdir_root = "staging".to_string();
    }
    let src = read_to_string(input_path);
    let t0 = Instant::now();
    let directives = parser::scan_directives(&src);
    let classes = parser::parse_all(&src);
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

    // Write README.md
    let readme_content = r#"# Estructura del Proyecto

Este proyecto ha sido generado con una estructura organizada:

- **/src**: Contiene los archivos de código fuente C++ (.cpp).
- **/include**: Contiene los archivos de cabecera (.hpp).
- **/build**: Directorio para archivos generados durante la compilación.
  - **/obj**: Archivos objeto (.obj) temporales.
  - **/bin**: Ejecutable final del juego.

Para compilar, ejecute `build.bat` (Windows) o `./build.sh` (Linux/Mac).
"#;
    let _ = fs::write(dir.join("README.md"), readme_content);

    let mut needs_object_base = directives.global_base;
    if classes.is_empty() {
        let class = parser::parse(&src);
        let mut class = class;
        if directives.global_base && class.base.is_none() {
            class.base = Some("Object".to_string());
            needs_object_base = true;
        }
        class.extra_includes = ultracpp::resolve_includes(&directives);
        let hpp = codegen::header(&class);
        let cpp = codegen::source(&class);
        let hpp_path = include_dir.join(format!("{}.hpp", class.name.to_lowercase()));
        let cpp_path = src_dir.join(format!("{}.cpp", class.name.to_lowercase()));
        write(hpp_path.to_str().unwrap(), &hpp);
        write(cpp_path.to_str().unwrap(), &cpp);
        println!("generated: {}, {}", hpp_path.display(), cpp_path.display());
        let exe_name = if cfg!(windows) { format!("{}.exe", base) } else { base.to_string() };
        let exe_path = bin_dir.join(&exe_name);
        let main_cpp_path = src_dir.join("main.cpp");
        if !no_main {
            let main_cpp = demo_main_cpp(&class, &class.name.to_lowercase());
            write(main_cpp_path.to_str().unwrap(), &main_cpp);
        }
        if needs_object_base {
            write_object_base(&src_dir, &include_dir);
        }
        let _ = tool_detector::write_build_script(&dir, &base);
        if staging {
            do_compile = false;
        }
        let mut compiled = false;
        if no_main && !do_compile {
            if bench || staging {
                let report_path = dir.join("report.json");
                let mut report = String::new();
                report.push_str("{");
                report.push_str(&format!("\"classes\":{},", 1));
                report.push_str(&format!("\"fields\":{},", class.fields.len()));
                report.push_str(&format!("\"methods\":{},", class.methods.len()));
                report.push_str(&format!("\"includes\":{},", class.extra_includes.len()));
                report.push_str(&format!("\"parse_ms\":{},", parse_ms));
                report.push_str(&format!("\"codegen_ms\":{}", 0));
                report.push_str("}");
                let _ = fs::write(report_path, report);
            }
            return;
        }
        let t1 = Instant::now();
        match compile_cpp(&dir, &base, compiler.as_deref(), &stdver, !no_main) {
            Ok(()) => {
                println!("compiled: {}", exe_path.display());
                compiled = true;
                let _ = Command::new(exe_path.to_str().unwrap()).current_dir(&dir).status();
            }
            Err(e) => {
                eprintln!("skip compile: {}", e);
            }
        }
        if bench || staging {
            let report_path = dir.join("report.json");
            let mut report = String::new();
            report.push_str("{");
            report.push_str(&format!("\"classes\":{},", 1));
            report.push_str(&format!("\"fields\":{},", class.fields.len()));
            report.push_str(&format!("\"methods\":{},", class.methods.len()));
            report.push_str(&format!("\"includes\":{},", class.extra_includes.len()));
            report.push_str(&format!("\"parse_ms\":{},", parse_ms));
            report.push_str(&format!("\"codegen_ms\":{},", t1.elapsed().as_millis()));
            report.push_str(&format!("\"compiled\":{}", if compiled { "true" } else { "false" }));
            report.push_str("}");
            let _ = fs::write(report_path, report);
        }
        return;
    }
    let t1 = Instant::now();
    for class in &classes {
        let mut class = class.clone();
        if directives.global_base && class.base.is_none() {
            class.base = Some("Object".to_string());
            needs_object_base = true;
        }
        class.extra_includes = ultracpp::resolve_includes(&directives);
        let hpp = codegen::header(&class);
        let cpp = codegen::source(&class);
        let hpp_path = include_dir.join(format!("{}.hpp", class.name.to_lowercase()));
        let cpp_path = src_dir.join(format!("{}.cpp", class.name.to_lowercase()));
        write(hpp_path.to_str().unwrap(), &hpp);
        write(cpp_path.to_str().unwrap(), &cpp);
        println!("generated: {}, {}", hpp_path.display(), cpp_path.display());
    }
    if needs_object_base {
        write_object_base(&src_dir, &include_dir);
    }

    let exe_name = if cfg!(windows) { format!("{}.exe", base) } else { base.to_string() };
    let exe_path = bin_dir.join(&exe_name);
    let main_cpp_path = src_dir.join("main.cpp");
    if !no_main {
        let target = select_entry_target(&classes, &directives);
        let main_cpp = demo_main_cpp(target, &target.name.to_lowercase());
        write(main_cpp_path.to_str().unwrap(), &main_cpp);
    }
    let _ = tool_detector::write_build_script(&dir, &base);
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
            report.push_str("{");
            report.push_str(&format!("\"classes\":{},", classes.len()));
            report.push_str(&format!("\"fields\":{},", fields));
            report.push_str(&format!("\"methods\":{},", methods));
            report.push_str(&format!("\"includes\":{},", ultracpp::resolve_includes(&directives).len()));
            report.push_str(&format!("\"parse_ms\":{},", parse_ms));
            report.push_str(&format!("\"codegen_ms\":{}", t1.elapsed().as_millis()));
            report.push_str("}");
            let _ = fs::write(report_path, report);
        }
        return;
    }
    let t2 = Instant::now();
    let mut compiled = false;
    match compile_cpp(&dir, &base, compiler.as_deref(), &stdver, !no_main) {
        Ok(()) => {
            println!("compiled: {}", exe_path.display());
            compiled = true;
            let _ = Command::new(exe_path.to_str().unwrap()).current_dir(&dir).status();
        }
        Err(e) => {
            eprintln!("skip compile: {}", e);
        }
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
        report.push_str("{");
        report.push_str(&format!("\"classes\":{},", classes.len()));
        report.push_str(&format!("\"fields\":{},", fields));
        report.push_str(&format!("\"methods\":{},", methods));
        report.push_str(&format!("\"includes\":{},", ultracpp::resolve_includes(&directives).len()));
        report.push_str(&format!("\"parse_ms\":{},", parse_ms));
        report.push_str(&format!("\"codegen_ms\":{},", t1.elapsed().as_millis()));
        report.push_str(&format!("\"compile_ms\":{},", t2.elapsed().as_millis()));
        report.push_str(&format!("\"compiled\":{}", if compiled { "true" } else { "false" }));
        report.push_str("}");
        let _ = fs::write(report_path, report);
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
    s.push_str(&format!("#include \"{}.hpp\"\n", base));
    s.push_str("#include <iostream>\n");
    s.push_str("int main() {\n");
    
    // Check for explicit entry points first: run_loop, start, main, run
    let entry_method = class.methods.iter().find(|m| 
        m.name == "run_loop" || m.name == "start" || m.name == "main" || m.name == "run"
    );

    if let Some(m) = entry_method {
        if !m.is_static {
            if ctor_args.is_empty() {
                s.push_str(&format!("  {} obj{{}};\n", class.name));
            } else {
                s.push_str(&format!("  {} obj({});\n", class.name, ctor_args));
            }
            s.push_str(&format!("  obj.{}(", m.name));
        } else {
            s.push_str(&format!("  {}::{}(", class.name, m.name));
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
            if !m.is_static {
                if ctor_args.is_empty() {
                    s.push_str(&format!("  {} obj{{}};\n", class.name));
                } else {
                    s.push_str(&format!("  {} obj({});\n", class.name, ctor_args));
                }
            }
            if m.is_static {
                s.push_str(&format!("  std::cout << {}::{}(", class.name, m.name));
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

fn compile_cpp(dir: &Path, base: &str, compiler: Option<&str>, stdver: &str, has_main: bool) -> Result<(), String> {
    let exe_name = if cfg!(windows) { format!("{}.exe", base) } else { base.to_string() };
    let bin_dir = dir.join("build").join("bin");
    let exe = bin_dir.join(&exe_name);
    
    if compiler.is_none() || compiler == Some("cl") {
        if let Ok(()) = tool_detector::compile_with_msvc(dir, base) {
            return Ok(());
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
                args.push("-I".to_string());
                args.push("include".to_string());
                for f in files {
                    args.push(f);
                }
                args.push("-o".to_string());
                args.push(format!("build/bin/{}", exe_name));
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                if let Ok(out) = Command::new("g++").current_dir(dir).args(&arg_refs).output() {
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
                args.push("-I".to_string());
                args.push("include".to_string());
                for f in files {
                    args.push(f);
                }
                args.push("-o".to_string());
                args.push(format!("build/bin/{}", exe_name));
                let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                if let Ok(out) = Command::new("clang++").current_dir(dir).args(&arg_refs).output() {
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
