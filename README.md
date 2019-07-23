# upc-checker

## Overview

### About

`upc-checker` is a small Rust Crate for quickly checking a UPC code compared to a check digit. It currently only supports the popular `UPC-A` format and is a `no_std` crate.

### An Example

Here is a small, working example of `upc-checker` in action:

```rust
extern crate upc_checker;

fn main() {
    let my_upc = upc_checker::UPCStandard::UPCA(
        [0, 3, 6, 0, 0, 0, 2, 4, 1, 4, 5]
    );
    let my_check_code: i8 = 7;

    let my_upc_struct = upc_checker::UPC {
        upc: my_upc,
        check_digit: my_check_code,
    };

    match my_upc_code_struct.check_code() { // `my_upc_code_struct.check_code()` returns `Result<bool, UPCError>`.
        Ok(x) => (), // `x` is a bool
        Err(_) => (), // Deal as you like
    }
}
```

## Documentation

You can find the documentation of this crate on a [handy doc.rs page](https://docs.rs/upc-checker/0.1.1/upc_checker/).
