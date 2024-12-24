fn main() {
    // 告诉 Cargo 在构建时链接 `ffi.c`
    println!("cargo:rerun-if-changed=ffi.c");
    cc::Build::new()
        .file("ffi.c")
        .compile("libffi.a");
}