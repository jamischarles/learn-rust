// compile  the c file(s)
// Read about build scripts
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// Run this via cargo run (plus TARGET, triple env vars)
// https://doc.rust-lang.org/cargo/reference/environment-variables.html

fn main() {
    println!("compiling the c file");
    cc::Build::new().file("./src/hello.c").compile("foo");
}
