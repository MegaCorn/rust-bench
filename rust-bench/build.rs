fn main() {
    println!("cargo:rerun-if-changed=ffi.c");
    cc::Build::new()
        .file("ffi.c")
        .compile("libffi.a");
}