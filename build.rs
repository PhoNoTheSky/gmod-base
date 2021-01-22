fn build_bindings(bindings_dir: &std::path::PathBuf) {
    let bindings = bindgen::Builder::default()
        .header("luajit/src/luajit.h")
        .header("luajit/src/lualib.h")
        .header("luajit/src/lauxlib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_var("(?:LUA|lua).*")
        .whitelist_type("(?:LUA|lua).*")
        .whitelist_function("(?:LUA|lua).*")
        .generate()
        .expect("failed to generate bindings");

    bindings
        .write_to_file(bindings_dir.join("bindings.rs"))
        .expect("failed to write bindings");
}

fn build_def(name: &str, target: &str, dll_path: &std::path::PathBuf, out_dir: &std::path::PathBuf) {
    let mut dump_bin = cc::windows_registry::find(target, "dumpbin.exe").unwrap();
    let output = dump_bin
        .arg("/EXPORTS")
        .arg(dll_path)
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let exports = stdout
        .split("\r\n")
        .map(|line| line.trim())
        .skip_while(|line| line != &"ordinal hint RVA      name")
        .skip(2)
        .take_while(|line| line != &"")
        .map(|line| line.split_whitespace().nth(3).unwrap())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let def_file = format!("{}\\{}.def", out_dir.to_str().unwrap(), name);
    let mut def_file = std::fs::File::create(def_file).unwrap();
    use std::io::Write;
    writeln!(def_file, "EXPORTS").unwrap();
    for export in exports {
        writeln!(def_file, "{}", export).unwrap();
    }
}

fn build_lib(name: &str, platform: &str, target: &str, out_dir: &std::path::PathBuf) {
    let out_dir = out_dir.to_str().unwrap();

    let mut lib = cc::windows_registry::find(target, "lib.exe")
        .expect("failed to find lib.exe");
    lib
        .arg("/NOLOGO")
        .arg(format!("/MACHINE:{}", platform))
        .arg(format!("/DEF:{}\\{}.def", out_dir, name))
        .arg(format!("/OUT:{}\\{}.lib", out_dir, name))
        .status()
        .unwrap()
        .success();
}

fn get_lua_shared(platform: &str) -> std::path::PathBuf {
    use winreg::enums::*;
    use winreg::RegKey;
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Valve\Steam").unwrap();
    let install_path: String = steam.get_value("InstallPath").unwrap();
    let install_path: std::path::PathBuf = install_path.into();
    if platform == "X64" {
        install_path
            .join("steamapps")
            .join("common")
            .join("GarrysMod")
            .join("bin")
            .join("win64")
            .join("lua_shared.dll")
    } else {
        install_path
            .join("steamapps")
            .join("common")
            .join("GarrysMod")
            .join("bin")
            .join("lua_shared.dll")
    }
}

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let platform = if target.contains("x86_64") { "X64" } else { "X86" };
    let dll_path = get_lua_shared(platform);
    let name = dll_path.file_stem().unwrap().to_str().unwrap();
    let out_dir: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let bindings_dir: std::path::PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    let bindings_dir = bindings_dir.join("bindings");

    std::fs::create_dir_all(&bindings_dir).unwrap();
    build_bindings(&bindings_dir);
    build_def(name, &target, &dll_path, &out_dir);
    build_lib(name, platform, &target, &out_dir);

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib={}", name);
}