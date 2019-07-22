# upc-checker

## Overview

### About

`upc-checker` is a small Rust Crate for quickly checking a UPC code compared to a check digit. Some more of the in-depth of this process can be found [here](). This is my first Rust Crate so apologies if it isn't memory efficiant as it could be (though it uses only `i8`, `i8` and `u16` for data ints).

### An Example

Here is a small example of `upc-checker` in action:

```rust
extern crate upc_checker;

fn main() {
    let my_code: Vec<i8> = vec![0, 3, 6, 0, 0, 0, 2, 4, 1, 4, 5]; // This is the main UPC code
    let my_check_digit: i8 = 7; // This is the UPC check digit

    let my_upc_code_struct = UPCCode { // Creating a structure with the UPC components inside
        code: my_code,
        check_digit: my_check_digit,
    };

    match my_upc_code_struct.check_code() { // `my_upc_code_struct.check_code()` returns `Result<bool, std::io::Error>`.
        Ok(x) => (), // `x` is a bool
        Err(_) => (), // Deal as you like
    }
}
```

## Documentation

You can find the documentation of this crate on a [handy doc.rs page](https://docs.rs/upc-checker/0.1.1/upc_checker/).
