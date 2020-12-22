# upc-checker

## Overview

### About

`upc-checker` is a small Rust Crate for quickly checking a UPC code compared to a check digit. It currently only supports the popular `UPC-A` format and is a `no_std` crate.

### An Example

Here is a small, working example of `upc-checker` in action:

```rust
use upc_checker::{Standard, Upc, UpcError};

let my_code_vector = Standard::UpcA(
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
);
let my_check_digit: i8 = 2;

let code = Upc {
    upc: my_code_vector,
    check_digit: my_check_digit,
};

match code.check() {
    Ok(x) => println!("Is the code valid?: {}", x),
    Err(UpcError::UpcOverflow) => {
        eprintln!("UPC code overflow! Please use only 0-9!");
    },
    Err(UpcError::CheckDigitOverflow) => {
        eprintln!("UPC check digit overflow! Please use only 0-9!");
    },
};
```

## Documentation

You can find the documentation of this crate on a [handy doc.rs page](https://docs.rs/upc-checker/).
