use std::path::PathBuf;

fn main() {
    let fmod_dir = option_env!("FMOD_SYS_FMOD_DIRECTORY")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fmod"));
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
        .header("src/wrapper.h");

    #[cfg(target_arch = "x86")]
    let target_arch = "x86";
    #[cfg(target_arch = "x86_64")]
    let target_arch = "x86_64";

    let include_debug = cfg!(any(debug_assertions, feature = "force-debug"));
    let debug_char = if include_debug { "L" } else { "" };

    println!("cargo:rustc-link-search={api_dir_display}/core/lib/{target_arch}");
    println!("cargo:rustc-link-search={api_dir_display}/studio/lib/{target_arch}");

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=fmod{debug_char}");
        println!("cargo:rustc-link-lib=fmodstudio{debug_char}");
    }

    let bindings = bindgen.generate().expect("failed to generate bindings");
    let out_path = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("failed to write bindings");
}
