fn main() {
    println!("Hello, world! from rust");
    call();
}

// output --> use this to map to bottom link
// exit status: 0
// cargo:rustc-link-lib=static=foo
// cargo:rustc-link-search=native=TARGET

// do we use C or no? is C for when we allow calling from C to Rust?
// link to c library in target folder (placed there by the build step)
#[link(name = "foo", kind = "static")]
extern "C" {
    // basically just providing input/output types?
    // establish connection to print_from_c function in c file
    fn print_from_c();
}

pub fn call() {
    unsafe {
        print_from_c();
    }
}
