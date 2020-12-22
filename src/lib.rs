#![no_std]

/// Possible errors for [Upc]-based checkin
#[derive(Debug, PartialEq, Clone)]
pub enum UpcError {
    /// Given i8 array has overflown with data that is not 0-9 (1 digit)
    UpcOverflow,

    /// Given i8 check digit has overflown with data that is not 0-9 (1 digit)
    CheckDigitOverflow,
}

/// The implementation on the widely-used UPC code standards with simple `i8`
/// arrays of a defined length.
///
/// # Overflowing
///
/// These arrays should **only** have int's that are 0-9 (1 digit) otherwise
/// [Upc.check] will throw an error defined as [UpcError::UpcOverflow].
///
/// # Standards implemented
///
/// - [Upc-A](https://en.wikipedia.org/wiki/Universal_Product_Code#Encoding)
#[derive(Debug, PartialEq, Clone)]
pub enum Standard {
    UpcA([i8; 11]),
}

/// Main Upc structure containing the base Upc code alonside it's
/// check digit. This is the core of the `Upc_checker` library
///
/// # Params
///
/// - Upc: A [Standard] enum
/// - check_digit: An i8 int for the Upc code's check digit
///
/// # Examples
///
/// **NOTE: The below example is a demo and will not work with the given Upc
/// code & check digit in practise.**
///
/// ```rust
/// use upc_checker::{Standard, Upc, UpcError};
///
/// let my_code_vector = Standard::UpcA(
///     [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0]
/// );
/// let my_check_digit: i8 = 2;
///
/// let code = Upc {
///     upc: my_code_vector,
///     check_digit: my_check_digit,
/// };
///
/// match code.check() {
///     Ok(x) => println!("Is the code valid?: {}", x),
///     Err(UpcError::UpcOverflow) => {
///         eprintln!("UPC code overflow! Please use only 0-9!");
///     },
///     Err(UpcError::CheckDigitOverflow) => {
///         eprintln!("UPC check digit overflow! Please use only 0-9!");
///     },
/// };
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Upc {
    /// UPC standard conformed to
    pub upc: Standard,

    /// Check digit for verification
    pub check_digit: i8,
}

impl Upc {
    /// Checks given upc code passed
    pub fn check(&self) -> Result<bool, UpcError> {
        self.validate_upc_overflow()?;

        let (even_nums, odd_nums) = self.split_upc_even_odd();

        let total: u16 = ((odd_nums * 3) + even_nums) % 10;

        if (total == 0 && self.check_digit == 0) || (10 - total == self.check_digit as u16) {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    /// Converts any defined standards given in [Standard] to an i8
    /// slice and returns it.
    fn get_upc_slice(&self) -> &[i8] {
        match &self.upc {
            Standard::UpcA(x) => &x[..],
        }
    }

    /// Validates that there has been no overflow of the [Upc] structure
    /// by hooking onto the `is_1_digit` helper function. This is the main
    /// source of the uses of [UpcError].
    fn validate_upc_overflow(&self) -> Result<(), UpcError> {
        for code in self.get_upc_slice() {
            is_1_digit(*code)?;
        }

        is_1_digit(self.check_digit)
    }

    /// Splits the UPC codes depending if they are odd or even (defined by a
    /// mod) into one of 2 values in a tuple of `([EVEN] u16, [ODD] u16)`.
    fn split_upc_even_odd(&self) -> (u16, u16) {
        let mut even_odd: (u16, u16) = (0, 0);

        for code in self.get_upc_slice() {
            if code % 2 == 0 {
                even_odd.0 += *code as u16;
            } else {
                even_odd.1 += *code as u16;
            }
        }

        even_odd
    }
}

/// Checks if a given i8 is 1 digit/character (0-9) wide
fn is_1_digit(digit: i8) -> Result<(), UpcError> {
    if digit < 0 || digit > 9 {
        Err(UpcError::CheckDigitOverflow)
    } else {
        Ok(())
    }
}
