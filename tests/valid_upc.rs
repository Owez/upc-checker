use upc_checker::{Standard, Upc};

/// Checks if check_upc is returning the right values for
/// [UPC-A](https://en.wikipedia.org/wiki/Universal_Product_Code#Encoding)
#[test]
fn valid_upc_a() {
    let my_upc = Standard::UpcA([0, 3, 6, 0, 0, 0, 2, 4, 1, 4, 5]);
    let my_check_code: i8 = 7;

    let my_upc_struct = Upc {
        upc: my_upc,
        check_digit: my_check_code,
    };

    assert_eq!(Ok(true), my_upc_struct.check());
}
