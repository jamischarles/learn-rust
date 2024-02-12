// compile  the c file(s)
//
// Run this via cargo run (plus TARGET, triple env vars)
// https://doc.rust-lang.org/cargo/reference/environment-variables.html

// from https://github.com/rust-lang/nomicon/pull/440
fn main() {
    // println!("cargo:rustc-link-lib=dylib=stdc++");
    //
    // Read about build scripts
    // Is this like setting an env var for cargo to pick up on? Very strange way to do it...
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo:rustc-link-search=/Users/jacharles/dev_freelance/snappy/build");
    // println!("cargo:rustc-link-search=/opt/homebrew/Cellar/snappy/1.1.10/include/");
}
