use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use ultracpp::{codegen, parser, tool_detector};

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
        eprintln!("usage: ultracpp <input.upp> [outdir]");
        std::process::exit(1);
    }
    let input_path = &args[1];
    let outdir_root = if args.len() >= 3 { &args[2] } else { "dist" };
    let src = read_to_string(input_path);
    let class = parser::parse(&src);
    let hpp = codegen::header(&class);
    let cpp = codegen::source(&class);
    let base = stem(Path::new(input_path)).to_lowercase();
    let dir = Path::new(outdir_root).join(&base);
    fs::create_dir_all(&dir).expect("create output dir failed");
    let hpp_path = dir.join(format!("{}.hpp", base));
    let cpp_path = dir.join(format!("{}.cpp", base));
    write(hpp_path.to_str().unwrap(), &hpp);
    write(cpp_path.to_str().unwrap(), &cpp);
    println!("generated: {}, {}", hpp_path.display(), cpp_path.display());

    // Auto-compile C++ if a compiler is available
    let exe_name = format!("{}.exe", base);
    let exe_path = dir.join(&exe_name);
    let main_cpp_path = dir.join("main.cpp");
    let main_cpp = demo_main_cpp(&class, &base);
    write(main_cpp_path.to_str().unwrap(), &main_cpp);
    let _ = tool_detector::write_build_script(&dir, &base);
    match compile_cpp(&dir, &base) {
        Ok(()) => {
            println!("compiled: {}", exe_path.display());
            let _ = Command::new(exe_path.to_str().unwrap()).current_dir(&dir).status();
        }
        Err(e) => {
            eprintln!("skip compile: {}", e);
        }
    }
}

fn demo_main_cpp(class: &ultracpp::Class, base: &str) -> String {
    let mut args: Vec<String> = Vec::new();
    for f in &class.fields {
        args.push(default_cpp_value(&f.ty));
    }
    let ctor_args = args.join(", ");
    let mut s = String::new();
    s.push_str(&format!("#include \"{}.hpp\"\n", base));
    s.push_str("#include <iostream>\n");
    s.push_str(&format!("int main() {{\n  {} obj({});\n", class.name, ctor_args));
    // find a method that returns String if exists
    if let Some(m) = class.methods.iter().find(|m| m.return_type == "String") {
        s.push_str(&format!("  std::cout << obj.{}(", m.name));
        let mut call_args: Vec<String> = Vec::new();
        for p in &m.params {
            call_args.push(default_cpp_value(&p.ty));
        }
        s.push_str(&call_args.join(", "));
        s.push_str(") << std::endl;\n");
    }
    s.push_str("  return 0;\n}\n");
    s
}

fn default_cpp_value(ty: &str) -> String {
    match ty {
        "String" => "std::string(\"Mundo\")".to_string(),
        "Int" | "int" => "0".to_string(),
        "Bool" | "bool" => "false".to_string(),
        _ => "{}".to_string(),
    }
}

fn compile_cpp(dir: &Path, base: &str) -> Result<(), String> {
    let cpp = format!("{}.cpp", base);
    let exe = format!("{}.exe", base);
    if let Ok(()) = tool_detector::compile_with_msvc(dir, base) {
        return Ok(());
    }
    // Try MSVC cl.exe
    let fe = format!("/Fe:{}", exe);
    let args_cl: Vec<&str> = vec!["/nologo", "/std:c++17", "/EHsc", cpp.as_str(), "main.cpp", fe.as_str()];
    if let Ok(out) = Command::new("cl.exe").current_dir(dir).args(&args_cl).output() {
        if out.status.success() {
            return Ok(());
        }
    }
    // Try g++
    let args_gpp: Vec<&str> = vec!["-std=c++17", cpp.as_str(), "main.cpp", "-o", exe.as_str()];
    if let Ok(out) = Command::new("g++").current_dir(dir).args(&args_gpp).output() {
        if out.status.success() {
            return Ok(());
        }
    }
    // Try clang++
    let args_clang: Vec<&str> = vec!["-std=c++17", cpp.as_str(), "main.cpp", "-o", exe.as_str()];
    if let Ok(out) = Command::new("clang++").current_dir(dir).args(&args_clang).output() {
        if out.status.success() {
            return Ok(());
        }
    }
    Err("no C++ compiler found. Tried: cl.exe, g++, clang++. Ensure one is on PATH.".to_string())
}
