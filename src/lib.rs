#![no_std]

/// Error enum for the UPCCodes. This is responsable for the Error return on
/// the `Result<T, E>` E (error) on the `UPCCode.check_upc()` method and
/// commonly implaments errors when users use standards implamented by the
/// `UPCCodeStandard` wrongly.
///
/// # Error Types
///
/// - UPCCodeOverflow: When the i8 array implamented in the standards defined
/// by the `UPCCodeStandard` enum has been overflown with data that is not 0-9
/// (1 digit)
/// - CheckDigitOverflow: When the i8 `check_digit` value implamented in the
/// `UPCCode` has been overflown with data that is not 0-9 (1 digit)
pub enum UPCCodeError {
    UPCCodeOverflow,
    CheckDigitOverflow,
}

/// The impamentations on the widely-used UPC code standards as simple i8
/// arrays with a defined length. These arrays should **only** have ints that
/// are 0-9 (1 digit) otherwise `UPCCode.upc_check()` will throw an error
/// defined as `UPCCodeError::UPCCodeOverflow`.
///
/// # Standards implamented
///
/// - [UPC-A](https://en.wikipedia.org/wiki/Universal_Product_Code#Encoding)
/// - [UPC-E](https://en.wikipedia.org/wiki/Universal_Product_Code#UPC-E)
pub enum UPCCodeStandard {
    UPCA([i8; 12]),
    UPCE([i8; 8]),
}

/// Main UPCCode structure containing the base UPC code alonside it's
/// check digit. This is the core of the `upc_checker` library
///
/// # Params
///
/// - upc: A `UPCCodeStandard` enum
/// - check_digit: An i8 int for the UPC code's check digit
///
/// # Examples
///
/// **NOTE: The below example is a demo and will not work with the given upc
/// code & check digit in practise.**
///
/// ```rust
/// extern crate upc_checker;
///
/// let my_code_vector = upc_checker::UPCCodeStandard::UPCE(
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// ); // NOTE digits should be 0-9.
/// let my_check_digit: i8 = 2; // NOTE check digit should be 0-9
///
/// let my_upc_code = upc_checker::UPCCode {
///     upc: my_code_vector,
///     check_digit: my_check_digit,
/// };
///
/// match my_upc_code.check_upc() {
///     Ok(x) => println!("Is the code valid?: {}", x),
///     Err(upc_checker::UPCCodeError::UPCCodeOverflow) => {
///         println!("UPC code overflow! Please use only 0-9!");
///     },
///     Err(upc_checker::UPCCodeError::CheckDigitOverflow) => {
///         println!("UPC check digit overflow! Please use only 0-9!");
///     },
/// };
/// ```
pub struct UPCCode {
    pub upc: UPCCodeStandard,
    pub check_digit: i8,
}

impl UPCCode {
    pub fn check_upc(&self) -> Result<bool, UPCCodeError> {
        match self.validate_upc_overflow() {
            Err(x) => return Err(x),
            Ok(_) => (),
        };

        let (even_nums, odd_nums) = self.split_upc_even_odd();
        let total: u16 = ((odd_nums * 3) + even_nums) % 10;

        if (total == 0 && self.check_digit == 0) || (10 - total == self.check_digit as u16) {
            return Ok(true);
        }

        Ok(false)
    }

    fn get_upc_slice(&self) -> &[i8] {
        match &self.upc {
            UPCCodeStandard::UPCA(x) => &x[..],
            UPCCodeStandard::UPCE(x) => &x[..],
        }
    }

    fn validate_upc_overflow(&self) -> Result<(), UPCCodeError> {
        for upc_code in self.get_upc_slice() {
            if !is_1_digit(*upc_code) {
                return Err(UPCCodeError::UPCCodeOverflow);
            }
        }

        match is_1_digit(self.check_digit) {
            true => Ok(()),
            false => Err(UPCCodeError::CheckDigitOverflow),
        }
    }
    fn split_upc_even_odd(&self) -> (u16, u16) {
        let mut even_odd: (u16, u16) = (0, 0);

        for upc_code in self.get_upc_slice() {
            if upc_code % 2 == 0 {
                even_odd.0 += *upc_code as u16;
            } else {
                even_odd.1 += *upc_code as u16;
            }
        }

        even_odd
    }
}

fn is_1_digit(digit: i8) -> bool {
    digit > 0 && digit < 9
}
