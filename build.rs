extern crate cc;

fn main() {
    cc::Build::new()
        // .cpp(true)
        // .flag("-std=c++11")
        .include("cpp")
        .file("cpp/fib.c")
        // .file("cpp/tree.cpp")
        .compile("libsolution.a");
}
