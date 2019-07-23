extern crate upc_checker;

/// Checks that the proper `Err(UPCError::CheckDigitOverflow)` is being
/// returned for overflowing check digits
#[test]
fn overflow_check_digit() {
    let my_upc = upc_checker::UPCStandard::UPCA([9,9,9,9,9,9,9,9,9,9,9,9]);
    let my_check_code: i8 = 70; // `70` should error

    let my_upc_struct = upc_checker::UPC {
        upc: my_upc,
        check_digit: my_check_code,
    };

    assert_eq!(Err(upc_checker::UPCError::CheckDigitOverflow), my_upc_struct.check_upc());
}
