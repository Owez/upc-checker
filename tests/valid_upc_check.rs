extern crate upc_checker;

#[test]
fn valid_upc_a_check() {
    let my_upc = upc_checker::UPCStandard::UPCA([9,9,9,9,9,9,9,9,9,9,9,9]);
    let my_check_code: i8 = 7;

    let my_upc_struct = upc_checker::UPC {
        upc: my_upc,
        check_digit: my_check_code,
    };

    assert_eq!(Ok(true), my_upc_struct.check_upc());
}
