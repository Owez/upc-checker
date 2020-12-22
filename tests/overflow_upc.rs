use upc_checker::{Standard, Upc, UpcError};

/// Checks that `UpcError` is being properly called when is_1_digit = false for
/// [UPC-A](https://en.wikipedia.org/wiki/Universal_Product_Code#Encoding)
#[test]
fn overflow_upc_a() {
    let my_upc = Standard::UpcA([9, 9, 9, 9, 9, 12, 9, 9, 9, 9, 9]); // `12` should error
    let my_check_code: i8 = 7;

    let my_upc_struct = Upc {
        upc: my_upc,
        check_digit: my_check_code,
    };

    assert_eq!(Err(UpcError::CheckDigitOverflow), my_upc_struct.check());
}
