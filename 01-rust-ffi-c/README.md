# FF Interop. Calling C code from your Rust code

## Built the C piece
`$ OUT_DIR=TARGET HOST=x86_64-apple-darwin TARGET=x86_64-apple-darwin OPT_LEVEL=0 cargo run --example build`

## Running it
`cargo run`

## Compile the c code (standalone)
`$ clang hello.c`

Run the c code
`$ ./a.out`
`-> Hello, World! from C!!!`


## Notes & Context 

## Using rust with other languages (like C)

## What is FFI?
https://spin.atomicobject.com/ffi-foreign-function-interfaces/

## In depth example for C ffi from Rusto
https://doc.rust-lang.org/nomicon/ffi.html

## Build C files you can call from your rust code
Using this crate: https://crates.io/crates/cc

```rust
// build.rs  - build step to compile the c files

fn main() {
    cc::Build::new()
        .file("foo.c")
        .file("bar.c")
        .compile("foo");
}
```


```rust
// call the c code from your rust app like this
extern "C" {
// basically just providing input/output types?
    fn foo_function();
    fn bar_function(x: i32) -> i32;
}

pub fn call() {
    unsafe {
        foo_function();
        bar_function(42);
    }
}
```



## Debugging ffi
https://github.com/rust-lang/miri
https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust

## More reading:
https://doc.rust-lang.org/nomicon/ffi.html

https://www.reddit.com/r/rust/comments/511c1h/linking_objectivec_to_rust/
https://doc.rust-lang.org/std/keyword.extern.html

https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code


## Required reading
https://doc.rust-lang.org/cargo/reference/build-scripts.html
