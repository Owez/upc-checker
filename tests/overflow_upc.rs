extern crate upc_checker;

/// Checks that `UPCError` is being properly called when is_1_digit = false for
/// [UPC-A](https://en.wikipedia.org/wiki/Universal_Product_Code#Encoding)
#[test]
fn overflow_upc_a() {
    let my_upc = upc_checker::UPCStandard::UPCA([9, 9, 9, 9, 9, 12, 9, 9, 9, 9, 9]); // `12` should error
    let my_check_code: i8 = 7;

    let my_upc_struct = upc_checker::UPC {
        upc: my_upc,
        check_digit: my_check_code,
    };

    assert_eq!(
        Err(upc_checker::UPCError::CheckDigitOverflow),
        my_upc_struct.check_upc()
    );
}
