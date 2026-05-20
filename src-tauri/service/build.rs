fn main() {
    cc::Build::new().file("src-c/c_lib.c").compile("c_lib");
}
