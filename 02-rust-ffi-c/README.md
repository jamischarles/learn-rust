# FFI Interop part 2 - Intermediate Example. Using a real C library from your Rust code Using a real C library

## Read this
https://doc.rust-lang.org/nomicon/ffi.html
https://github.com/rust-lang/nomicon/pull/440
https://doc.rust-lang.org/cargo/reference/build-scripts.html



## We're doing this exercise from the book
https://doc.rust-lang.org/nomicon/ffi.html
https://github.com/rust-lang/nomicon/pull/440

## Running the Rust app
`cargo run`


## Failure?
You'll see a failure, so you have to install snappy, then build it  and then reference the location in the build script. 

## Success:
Prints to terminal:
```
Response from snappy (C library we called via FFI):
---------------------------------------------------
max compressed length of a 100 byte buffer: 148
```


## Extra credit:
Q: How could we get this to work with the brew-installed version of snappy? 

## More reading:
https://www.reddit.com/r/rust/comments/82he2e/comment/dvb38vr/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

