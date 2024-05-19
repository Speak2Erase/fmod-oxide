use std::path::PathBuf;

#[cfg(windows)]
fn find_fmod_directory() -> PathBuf {
    for drive in ["C", "D"] {
        let test_path = PathBuf::from(format!(
            "{drive}:/Program Files (x86)/FMOD SoundSystem/FMOD Studio API Windows"
        ));
        if test_path.exists() {
            return test_path;
        }
    }

    for path in ["./FMOD Studio API Windows", "./FMOD SoundSystem"] {
        let path = PathBuf::from(path)
            .canonicalize()
            .expect("failed to canonicalize fmod path");
        if path.exists() {
            return path;
        }
    }

    std::env::var_os("FMOD_SYS_FMOD_DIRECTORY")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fmod"))
}

#[cfg(not(windows))]
fn find_fmod_directory() -> PathBuf {
    std::env::var_os("FMOD_SYS_FMOD_DIRECTORY")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fmod"))
}

fn main() {
    let fmod_dir = find_fmod_directory();
    let api_dir = fmod_dir.join("api");

    assert!(fmod_dir.exists(), "fmod directory not present");
    assert!(api_dir.exists(), "fmod api dir does not exist");

    let api_dir_display = api_dir.display();
    println!("cargo:rerun-if-changed=\"{api_dir_display}/core/inc\"");
    println!("cargo:rerun-if-changed=\"{api_dir_display}/studio/inc\"");

    let bindgen = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("-I{api_dir_display}/core/inc"))
        .clang_arg(format!("-I{api_dir_display}/studio/inc"))
        .newtype_enum("FMOD_RESULT")
        .must_use_type("FMOD_RESULT")
        .new_type_alias("FMOD_BOOL")
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .derive_hash(true)
        .derive_default(true)
        .prepend_enum_name(false) // fmod already does this
        .header("src/wrapper.h");

    #[cfg(target_arch = "x86")]
    let target_arch = "x86";
    #[cfg(all(target_arch = "x86_64", not(windows)))]
    let target_arch = "x86_64";
    #[cfg(all(target_arch = "x86_64", windows))]
    let target_arch = "x64";

    let include_debug = cfg!(any(debug_assertions, feature = "force-debug"));
    let debug_char = if include_debug { "L" } else { "" };

    println!("cargo:rustc-link-search={api_dir_display}/core/lib/{target_arch}");
    println!("cargo:rustc-link-search={api_dir_display}/studio/lib/{target_arch}");

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=fmod{debug_char}");
        println!("cargo:rustc-link-lib=fmodstudio{debug_char}");
    }
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=fmod{debug_char}_vc");
        println!("cargo:rustc-link-lib=fmodstudio{debug_char}_vc");
    }

    let bindings = bindgen.generate().expect("failed to generate bindings");
    let out_path = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("failed to write bindings");

    println!("cargo:rerun-if-changed=\"src/channel_control.cpp\"");
    println!("cargo:rerun-if-changed=\"src/channel_control.h\"");

    // wrapper does not use the stdlib
    let mut build = cc::Build::new();

    build
        .cpp(true)
        .cpp_link_stdlib(None)
        .cpp_set_stdlib(None)
        .include(format!("{api_dir_display}/core/inc"))
        .file("src/channel_control.cpp");
    #[cfg(target_os = "windows")]
    {
        #[cfg(target_arch = "x86_64")]
        let target = "x86_64-pc-windows-msvc";
        #[cfg(target_arch = "x86")]
        let target = "i686-pc-windows-msvc";
        let tool = cc::windows_registry::find_tool(target, "cl.exe").expect("failed to find cl");
        build.compiler(tool.path());
    }

    build.compile("channel_control_wrapper");
}
