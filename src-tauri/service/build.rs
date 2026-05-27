fn main() {
    println!("cargo:rerun-if-changed=src-c/c_lib.c");
    cc::Build::new().file("src-c/c_lib.c").compile("c_lib");
}
