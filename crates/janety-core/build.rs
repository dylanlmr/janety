use std::env;

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    
    if target.contains("wasm32") {
        return;
    }

    println!("cargo:rerun-if-changed=janet_c/janet.c");
    println!("cargo:rerun-if-changed=janet_c/wrapper.c");

    cc::Build::new()
        .file("janet_c/janet.c")
        .file("janet_c/wrapper.c")
        .include("janet_c")
        .compile("janet_embed");
}
