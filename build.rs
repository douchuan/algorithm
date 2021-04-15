extern crate cc;

fn main() {
    cc::Build::new()
        .include("c")
        .file("c/fib.c")
        .compile("libsolution.a");
}
