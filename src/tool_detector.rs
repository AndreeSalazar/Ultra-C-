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
    let vswhere = if vswhere_pf.exists() { vswhere_pf } else { vswhere_x86 };
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

pub fn compile_with_msvc(dir: &Path, base: &str) -> Result<(), String> {
    if let Some(vsdevcmd) = find_vs_dev_cmd() {
        let cpp = format!("{}.cpp", base);
        let exe = format!("{}.exe", base);
        let cmdline = format!(
            "call \"{}\" && cl.exe /nologo /std:c++17 /EHsc {} main.cpp /Fe:{}",
            vsdevcmd.display(),
            cpp,
            exe
        );
        let out = Command::new("cmd.exe")
            .current_dir(dir)
            .args(["/C", &cmdline])
            .output()
            .map_err(|e| e.to_string())?;
        if out.status.success() {
            return Ok(());
        }
        return Err(String::from("MSVC cl.exe failed"));
    }
    Err(String::from("VsDevCmd.bat not found"))
}

pub fn write_build_script(dir: &Path, base: &str) -> Result<(), String> {
    let script = dir.join("build.bat");
    let mut content = String::new();
    if let Some(vsdevcmd) = find_vs_dev_cmd() {
        content.push_str("@echo off\n");
        content.push_str("setlocal\n");
        content.push_str(&format!("call \"{}\"\n", vsdevcmd.display()));
        content.push_str(&format!("cl.exe /nologo /std:c++17 /EHsc {}.cpp main.cpp /Fe:{}.exe\n", base, base));
    } else {
        content.push_str("@echo off\n");
        content.push_str("setlocal\n");
        content.push_str("call VsDevCmd.bat\n");
        content.push_str(&format!("cl.exe /nologo /std:c++17 /EHsc {}.cpp main.cpp /Fe:{}.exe\n", base, base));
    }
    std::fs::write(script, content).map_err(|e| e.to_string())
}
