use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=c/shim.h");
    println!("cargo:rerun-if-changed=c/shim.c");

    cc::Build::new()
        .file("c/shim.c")
        .include("c")
        .flag_if_supported("-std=c11")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Werror")
        .compile("shim");

    let bindings = bindgen::Builder::default()
        .header("c/shim.h")
        .clang_arg("-std=c11")
        .allowlist_function("shim_.*")
        .allowlist_type("shim_.*")
        .allowlist_var("SHIM_.*")
        .derive_default(false)
        .derive_debug(true)
        .layout_tests(true)
        .generate()
        .expect("bindgen failed to generate bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out)
        .expect("failed to write bindings");
}
