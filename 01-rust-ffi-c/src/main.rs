fn main() {
    println!("Hello, world! from rust");

    call();
}

// output --> use this to map to bottom link
// exit status: 0
// cargo:rustc-link-lib=static=foo
// cargo:rustc-link-search=native=TARGET

// do we use C or no? is C for when we allow calling from C to Rust?
// link to c library in target folder
#[link(name = "foo", kind = "static")]
extern "C" {
    // basically just providing input/output types?
    // fn foo_function();
    // fn bar_function(x: i32) -> i32;
    fn print_from_c();
    // fn bar_function(x: i32) -> i32;
}

pub fn call() {
    unsafe {
        print_from_c();
        // bar_function(42);
    }
}
