use upc_checker::{Upc, Standard, UpcError};

/// Checks that the proper `Err(UPCError::CheckDigitOverflow)` is being
/// returned for overflowing check digits
#[test]
fn overflow_check_digit() {
    let my_upc = Standard::UpcA([9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    let my_check_code: i8 = 70; // `70` should error

    let my_upc_struct = Upc {
        upc: my_upc,
        check_digit: my_check_code,
    };

    assert_eq!(
        Err(UpcError::CheckDigitOverflow),
        my_upc_struct.check()
    );
}
