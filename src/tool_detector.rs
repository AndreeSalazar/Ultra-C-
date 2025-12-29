use std::path::{Path, PathBuf};
use std::process::Command;

fn program_files_x86() -> PathBuf {
    std::env::var_os("ProgramFiles(x86)")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Program Files (x86)"))
}

fn program_files() -> PathBuf {
    std::env::var_os("ProgramFiles")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Program Files"))
}

pub fn find_vs_dev_cmd() -> Option<PathBuf> {
    let base_x86 = program_files_x86();
    let base_pf = program_files();
    let candidates = [
        base_pf.join(r"Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"),
        base_pf.join(r"Microsoft Visual Studio\2019\BuildTools\Common7\Tools\VsDevCmd.bat"),
        base_pf.join(r"Microsoft Visual Studio\2017\BuildTools\Common7\Tools\VsDevCmd.bat"),
        base_pf.join(r"Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat"),
        base_pf.join(r"Microsoft Visual Studio\2019\Community\Common7\Tools\VsDevCmd.bat"),
        base_pf.join(r"Microsoft Visual Studio\2017\Community\Common7\Tools\VsDevCmd.bat"),
        base_pf.join(r"Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat"),
        base_pf.join(r"Microsoft Visual Studio\2019\Professional\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2019\BuildTools\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2017\BuildTools\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2019\Community\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2017\Community\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2022\Professional\Common7\Tools\VsDevCmd.bat"),
        base_x86.join(r"Microsoft Visual Studio\2019\Professional\Common7\Tools\VsDevCmd.bat"),
    ];
    for c in candidates.iter() {
        if c.exists() {
            return Some(c.clone());
        }
    }
    let vswhere_pf = base_pf.join(r"Microsoft Visual Studio\Installer\vswhere.exe");
    let vswhere_x86 = base_x86.join(r"Microsoft Visual Studio\Installer\vswhere.exe");
    let vswhere = if vswhere_pf.exists() {
        vswhere_pf
    } else {
        vswhere_x86
    };
    if vswhere.exists() {
        if let Ok(out) = Command::new(vswhere)
            .args([
                "-latest",
                "-products",
                "*",
                "-requires",
                "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
                "-property",
                "installationPath",
            ])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    let p = Path::new(&s).join(r"Common7\Tools\VsDevCmd.bat");
                    if p.exists() {
                        return Some(p);
                    }
                }
            }
        }
    }
    None
}

pub fn compile_with_msvc(dir: &Path, _base: &str) -> Result<(), String> {
    let build_bat = dir.join("build.bat");
    if build_bat.exists() {
        let out = Command::new("cmd.exe")
            .current_dir(dir)
            .args(["/C", "build.bat"])
            .output()
            .map_err(|e| e.to_string())?;
        if out.status.success() {
            return Ok(());
        }
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!(
            "MSVC build.bat failed:\nSTDOUT:\n{}\nSTDERR:\n{}",
            stdout, stderr
        ));
    }
    Err(String::from("build.bat not found (MSVC)"))
}

pub fn write_build_script_opts(
    dir: &Path,
    base: &str,
    release: bool,
    _gpu: Option<&str>,
    bridge: Option<&str>,
    san: Option<&str>,
    coverage: bool,
) -> Result<(), String> {
    let script = dir.join("build.bat");
    let mut content = String::new();
    let src_all = dir.join("src").join("all.cpp");
    let use_unity_src = src_all.exists();
    if let Some(vsdevcmd) = find_vs_dev_cmd() {
        content.push_str("@echo off\n");
        content.push_str("setlocal EnableDelayedExpansion\n");
        content.push_str(&format!("set BASE={}\n", base));
        content.push_str(&format!("call \"{}\"\n", vsdevcmd.display()));
        let mut opts = String::from("/nologo /std:c++17 /EHsc /W4 /WX /permissive-");
        if bridge.is_some() {
            opts.push_str(" /D UCPP_DLL /D UCPP_BUILD");
        }
        if let Some(s) = san {
            if s == "asan" {
                opts.push_str(" /fsanitize=address");
            }
        }
        if release {
            opts.push_str(" /O2");
        }
        content.push_str("if exist \"src\\pch.cpp\" (\n");
        content.push_str(&format!("  cl.exe /c {} /I include /Yc\"pch.hpp\" /Fp\"build\\obj\\\\pch.pch\" src\\pch.cpp /Fo\"build\\obj\\\\pch.obj\"\n", opts));
        content.push_str(")\n");
        if use_unity_src {
            content.push_str(&format!("cl.exe {} /I include /Yu\"pch.hpp\" /Fp\"build\\obj\\\\pch.pch\" src\\all.cpp build\\obj\\pch.obj /Fo\"build\\obj\\\\\" /Fe\"build\\bin\\%BASE%.exe\"\n", opts));
        } else {
            content.push_str("set SRCS=\n");
            content.push_str("for %%F in (src\\*.cpp) do (\n");
            content.push_str("  if /I not \"%%~nxF\"==\"pch.cpp\" (\n");
            content.push_str("    set SRCS=!SRCS! \"%%F\"\n");
            content.push_str("  )\n");
            content.push_str(")\n");
            content.push_str(&format!("cl.exe {} /I include /Yu\"pch.hpp\" /Fp\"build\\obj\\\\pch.pch\" !SRCS! build\\obj\\pch.obj /Fo\"build\\obj\\\\\" /Fe\"build\\bin\\%BASE%.exe\"\n", opts));
        }
        if bridge.is_some() {
            content.push_str("set SRCS=\n");
            content.push_str("for %%F in (src\\*.cpp) do (\n");
            content.push_str("  if /I not \"%%~nxF\"==\"entry.cpp\" if /I not \"%%~nxF\"==\"all.cpp\" if /I not \"%%~nxF\"==\"pch.cpp\" (\n");
            content.push_str("    set SRCS=!SRCS! \"%%F\"\n");
            content.push_str("  )\n");
            content.push_str(")\n");
            content.push_str(&format!("cl.exe {} /I include /Yu\"pch.hpp\" !SRCS! /link /DLL /OUT:\"build\\bin\\%BASE%.dll\"\n", opts));
        }
        if release {
            content.push_str("if not exist \"build\\release\" mkdir \"build\\release\"\n");
            content.push_str("powershell -Command Compress-Archive -Force -Path include\\*,build\\bin\\*,README.md -DestinationPath \"build\\release\\%BASE%-windows.zip\"\n");
        }
    } else {
        content.push_str("@echo off\n");
        content.push_str("setlocal EnableDelayedExpansion\n");
        content.push_str("call VsDevCmd.bat\n");
        content.push_str(&format!("set BASE={}\n", base));
        let mut opts = String::from("/nologo /std:c++17 /EHsc /W4 /WX /permissive-");
        if bridge.is_some() {
            opts.push_str(" /D UCPP_DLL /D UCPP_BUILD");
        }
        if let Some(s) = san {
            if s == "asan" {
                opts.push_str(" /fsanitize=address");
            }
        }
        if release {
            opts.push_str(" /O2");
        }
        content.push_str("if exist \"src\\pch.cpp\" (\n");
        content.push_str(&format!("  cl.exe /c {} /I include /Yc\"pch.hpp\" /Fp\"build\\obj\\\\pch.pch\" src\\pch.cpp /Fo\"build\\obj\\\\pch.obj\"\n", opts));
        content.push_str(")\n");
        if use_unity_src {
            content.push_str(&format!("cl.exe {} /I include /Yu\"pch.hpp\" /Fp\"build\\obj\\\\pch.pch\" src\\all.cpp build\\obj\\pch.obj /Fo\"build\\obj\\\\\" /Fe\"build\\bin\\%BASE%.exe\"\n", opts));
        } else {
            content.push_str("set SRCS=\n");
            content.push_str("for %%F in (src\\*.cpp) do (\n");
            content.push_str("  if /I not \"%%~nxF\"==\"pch.cpp\" (\n");
            content.push_str("    set SRCS=!SRCS! \"%%F\"\n");
            content.push_str("  )\n");
            content.push_str(")\n");
            content.push_str(&format!("cl.exe {} /I include /Yu\"pch.hpp\" /Fp\"build\\obj\\\\pch.pch\" !SRCS! build\\obj\\pch.obj /Fo\"build\\obj\\\\\" /Fe\"build\\bin\\%BASE%.exe\"\n", opts));
        }
        if bridge.is_some() {
            content.push_str("set SRCS=\n");
            content.push_str("for %%F in (src\\*.cpp) do (\n");
            content.push_str("  if /I not \"%%~nxF\"==\"entry.cpp\" if /I not \"%%~nxF\"==\"all.cpp\" if /I not \"%%~nxF\"==\"pch.cpp\" (\n");
            content.push_str("    set SRCS=!SRCS! \"%%F\"\n");
            content.push_str("  )\n");
            content.push_str(")\n");
            content.push_str(&format!("cl.exe {} /I include /Yu\"pch.hpp\" !SRCS! /link /DLL /OUT:\"build\\bin\\%BASE%.dll\"\n", opts));
        }
        if release {
            content.push_str("if not exist \"build\\release\" mkdir \"build\\release\"\n");
            content.push_str("powershell -Command Compress-Archive -Force -Path include\\*,build\\bin\\*,README.md -DestinationPath \"build\\release\\%BASE%-windows.zip\"\n");
        }
    }
    std::fs::write(script, content).map_err(|e| e.to_string())
        .and_then(|_| {
            let sh = dir.join("build.sh");
            let mut shc = String::new();
            shc.push_str("#!/usr/bin/env bash\n");
            shc.push_str("set -e\n");
            let opt = if release { "-O2 " } else { "" };
            let warn = "-Wall -Wextra -Werror ";
            let sanflag = if let Some(s) = san {
                match s {
                    "asan" => "-fsanitize=address ",
                    "ubsan" => "-fsanitize=undefined ",
                    "tsan" => "-fsanitize=thread ",
                    _ => "",
                }
            } else { "" };
            let covflag = if coverage { "-fprofile-arcs -ftest-coverage " } else { "" };
            shc.push_str("if [ -f \"include/pch.hpp\" ]; then\n");
            shc.push_str("  g++ -std=c++17 -x c++-header include/pch.hpp -o build/obj/pch.hpp.gch || clang++ -std=c++17 -x c++-header include/pch.hpp -o build/obj/pch.hpp.gch\n");
            shc.push_str("fi\n");
            if use_unity_src {
                shc.push_str(&format!("g++ -std=c++17 {}{}{}{}-fvisibility=hidden -I include -include include/pch.hpp src/all.cpp -o build/bin/{}.exe || clang++ -std=c++17 {}{}{}{}-fvisibility=hidden -I include -include include/pch.hpp src/all.cpp -o build/bin/{}.exe\n", opt, warn, sanflag, covflag, base, opt, warn, sanflag, covflag, base));
            } else {
                shc.push_str(&format!("g++ -std=c++17 {}{}{}{}-fvisibility=hidden -I include -include include/pch.hpp src/*.cpp -o build/bin/{}.exe || clang++ -std=c++17 {}{}{}{}-fvisibility=hidden -I include -include include/pch.hpp src/*.cpp -o build/bin/{}.exe\n", opt, warn, sanflag, covflag, base, opt, warn, sanflag, covflag, base));
            }
            if bridge.is_some() {
                shc.push_str("SRCS=$(ls src/*.cpp | grep -v -E \"(entry\\.cpp|all\\.cpp)\")\n");
                shc.push_str(&format!("g++ -std=c++17 {}{}{}-fvisibility=hidden -I include -include include/pch.hpp $SRCS -shared -fPIC -o build/bin/lib{}.so || clang++ -std=c++17 {}{}{}-fvisibility=hidden -I include -include include/pch.hpp $SRCS -shared -fPIC -o build/bin/lib{}.so\n", opt, sanflag, covflag, base, opt, sanflag, covflag, base));
            }
            if release {
                shc.push_str("mkdir -p build/release\n");
                shc.push_str(&format!("tar -czf build/release/{}-linux.tar.gz include build/bin README.md\n", base));
            }
            std::fs::write(sh, shc).map_err(|e| e.to_string())
        })
}
pub fn write_build_script(dir: &Path, base: &str) -> Result<(), String> {
    write_build_script_opts(dir, base, false, None, None, None, false)
}
