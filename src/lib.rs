#![no_std]

/// Error enum for the UPCs. This is responsable for the Error return on
/// the `Result<T, E>` E (error) on the `UPC.check_upc()` method and
/// commonly implaments errors when users use standards implamented by the
/// `UPCStandard` wrongly.
///
/// # Error Types
///
/// - UPCOverflow: When the i8 array implamented in the standards defined
/// by the `UPCStandard` enum has been overflown with data that is not 0-9
/// (1 digit)
/// - CheckDigitOverflow: When the i8 `check_digit` value implamented in the
/// `UPC` has been overflown with data that is not 0-9 (1 digit)
pub enum UPCError {
    UPCOverflow,
    CheckDigitOverflow,
}

/// The impamentations on the widely-used UPC code standards as simple i8
/// arrays with a defined length. These arrays should **only** have ints that
/// are 0-9 (1 digit) otherwise `UPC.upc_check()` will throw an error
/// defined as `UPCError::UPCOverflow`.
///
/// # Standards implamented
///
/// - [UPC-A](https://en.wikipedia.org/wiki/Universal_Product_Code#Encoding)
/// - [UPC-E](https://en.wikipedia.org/wiki/Universal_Product_Code#UPC-E)
#[derive(Debug, PartialEq, Clone)]
pub enum UPCStandard {
    UPCA([i8; 12]),
    UPCE([i8; 8]),
}

/// Main UPC structure containing the base UPC code alonside it's
/// check digit. This is the core of the `upc_checker` library
///
/// # Params
///
/// - upc: A `UPCStandard` enum
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
/// let my_code_vector = upc_checker::UPCStandard::UPCE(
///     [0, 1, 2, 3, 4, 5, 6, 7]
/// ); // NOTE digits should be 0-9.
/// let my_check_digit: i8 = 2; // NOTE check digit should be 0-9
///
/// let my_upc_code = upc_checker::UPC {
///     upc: my_code_vector,
///     check_digit: my_check_digit,
/// };
///
/// match my_upc_code.check_upc() {
///     Ok(x) => println!("Is the code valid?: {}", x),
///     Err(upc_checker::UPCError::UPCOverflow) => {
///         println!("UPC code overflow! Please use only 0-9!");
///     },
///     Err(upc_checker::UPCError::CheckDigitOverflow) => {
///         println!("UPC check digit overflow! Please use only 0-9!");
///     },
/// };
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct UPC {
    pub upc: UPCStandard,
    pub check_digit: i8,
}

impl UPC {
    /// The main frontend method for the `UPC` structure. This method uses
    /// data from the super `UPC` struct and returns a Result enum with
    /// either a `bool` (IF the check digit is valid) or an instance of the
    /// `UPCError` enum.
    ///
    /// **NOTE: For more documentation & examples, please view the `UPC`
    /// documentation directly.**
    pub fn check_upc(&self) -> Result<bool, UPCError> {
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

    /// Converts any defined standards given in `UPCStandard` to an i8
    /// slice and returns it.
    ///
    /// **TODO: Make this automatically implament new standard from the
    /// afformentioned enum instead of matching all values.**
    fn get_upc_slice(&self) -> &[i8] {
        match &self.upc {
            UPCStandard::UPCA(x) => &x[..],
            UPCStandard::UPCE(x) => &x[..],
        }
    }

    /// Validates that there has been no overflow of the `UPC` structure
    /// by hooking onto the `is_1_digit` helper function. This is the main
    /// source of the uses of `UPCError`.
    fn validate_upc_overflow(&self) -> Result<(), UPCError> {
        for upc_code in self.get_upc_slice() {
            if !is_1_digit(*upc_code) {
                return Err(UPCError::UPCOverflow);
            }
        }

        match is_1_digit(self.check_digit) {
            true => Ok(()),
            false => Err(UPCError::CheckDigitOverflow),
        }
    }

    /// Splits the UPC codes depending if they are odd or even (defined by a
    /// mod) into one of 2 values in a tuple of `([EVEN] u16, [ODD] u16)`.
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

/// Checks if a given i8 is 1 digit/character (0-9) wide
fn is_1_digit(digit: i8) -> bool {
    digit > 0 && digit < 9
}
