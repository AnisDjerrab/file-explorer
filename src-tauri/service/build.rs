fn main() {
    cc::Build::new().file("src/c_lib.c").compile("c_lib");
}
