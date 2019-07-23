#![no_std]

/// The error enum for the upc-checker crate.
/// 
/// # Error Types
/// 
/// - `IntOverflow`: When a given int inside the `UPCCode` struct is
/// above/below 0-9 (larger than 1 chat)
#[derive(Debug)]
pub enum UPCCodeError {
    IntOverflow,
}

/// The type enum for UPCCode. You can currently use the following standards
/// listed below:
/// 
/// # UPC Code Types
/// 
/// - [UPC-A](https://en.wikipedia.org/wiki/Universal_Product_Code#Encoding)
/// - [UPC-E](https://en.wikipedia.org/wiki/Universal_Product_Code#UPC-E)
#[derive(Debug, PartialEq, Clone)]
pub enum UPCCodeType {
    UPCA([i8; 12]),
    UPCE([i8; 8]),
}

/// UPCCode is the frontend struct for the upc-checker module, allowing easy
/// access with an i8 vector (known as `code`) and one straggler i8 check
/// code (known as `check_code`).
///
/// **NOTE: An integer overflow will occur if the vector is too long. It is
/// advisible to follow the standardized UPC-A, UPC-E or similar.**
///
/// # Examples
///
/// ```rust
/// extern crate upc_checker;
///
/// let my_code_vector: Vec<i8> = vec![3, 5, 7, 4]; // NOTE digits should be 0-9
/// let my_check_digit: i8 = 2; // NOTE check digit should be 0-9
///
/// let my_upc_code = upc_checker::UPCCode {
///     code: my_code_vector,
///     check_digit: my_check_digit,
/// };
///
/// match my_upc_code.check_code() {
///     Ok(x) => println!("Is the code valid?: {}", x),
///     Err(upc_checker::UPCCodeError::IntOverflow) => {
///         println!("Integer overflow! Please use only 0-9!");
///     }
/// };
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct UPCCode {
    pub code: UPCCodeType,
    pub check_digit: i8,
}

impl UPCCode {
    /// Validates that all data in the `UPCCode` is 1-char in len (0-9)
    /// instead of the -255 to 255 of an i8
    fn validate_nums(&self) -> bool {
        match self.code {
            UPCCodeType::UPCA(x) => {
                if !self.check_slice_validity(&x[..]) {
                    return false;
                }
            },
            UPCCodeType::UPCE(x) => {
                if !self.check_slice_validity(&x[..]) {
                    return false;
                }
            },
        }

        is_1_digit(self.check_digit)
    }

    /// Checks an individual slice's validity using the `is_1_digit()` function
    /// and returns a bool if it is valid or not.
    fn check_slice_validity(&self, given_slice: &[i8]) -> bool {
        for code in given_slice {
            if !is_1_digit(*code) {
                return false;
            }
        }

        true
    }

    /// Adds odd and even numbers (using %) to a 2-len
    /// u8 tuple `(even, odd)` respectivly
    fn mod_check_slice(&self, codes: &[i8]) -> (u8, u8) {
        let mut result: (u8, u8) = (0, 0);
        
        for code in codes {
            if code % 2 == 0 {
                result.0 += *code as u8;
            } else {
                result.1 += *code as u8;
            }
        }

        result
    }

    /// Frontend function for `self.mod_check_slice()`. Matches enums and
    /// preforms calculations using said function.UPCCodeType
    /// 
    /// Returns a `(even: u8, odd: u8)` tuple.
    fn add_even_odd_total(&self) -> (u8, u8) {
        match self.code {
            UPCCodeType::UPCA(x) => return self.mod_check_slice(&x[..]),
            UPCCodeType::UPCE(x) => return self.mod_check_slice(&x[..]),
        }
    }

    /// Frontend function for checking with given `check_code` for the
    /// `UPCCode` structure.
    ///
    /// Check wikipedia for the process of using a check digit for a UPC code
    /// [here](https://en.wikipedia.org/wiki/Check_digit#UPC).
    ///
    /// # Examples
    ///
    /// ```rust
    /// extern crate upc_checker;
    ///
    /// let my_struct = upc_checker::UPCCode {
    ///     code: vec![3, 5, 7, 4],
    ///     check_digit: 3,
    /// };
    ///
    /// println!("Result: {}", my_struct.check_code().unwrap());
    /// ```
    pub fn check_code(&self) -> Result<bool, UPCCodeError> {
        if !self.validate_nums() {
            return Err(UPCCodeError::IntOverflow);
        }

        let mut total: u16 = 0;
        let (even_nums, odd_nums) = self.add_even_odd_total();

        total += ((odd_nums as u16 * 3) + even_nums as u16) % 10;

        if (total == 0 && self.check_digit == 0) || (10 - total == self.check_digit as u16) {
            return Ok(true);
        }

        Ok(false)
    }
}

/// Checks if a given i8 is 1 character wide (0-9)
fn is_1_digit(num: i8) -> bool {
    !(num < 0 || num > 9)
}

/// Testing module for the main UPCCode struct along with its helper funcs.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the helper function is_1_digit returns a bool of true if it
    /// is 1-char long (0-9) and a false if it is any other number in the
    /// allowed i8 memory space.
    #[test]
    fn is_1_digit_test() {
        assert_eq!(true, is_1_digit(0));
        assert_eq!(true, is_1_digit(9));
        assert_eq!(false, is_1_digit(10));
        assert_eq!(false, is_1_digit(-2));
    }

    /// Checks if you can properly make the UPCCode structure with the
    /// **UPC-E** type.
    ///
    /// **NOTE: This is an invalid check code and is only running to see if you
    /// can properly create the structure with the given datatypes.**
    #[test]
    fn upc_code_struct_upce_make() {
        let my_code = UPCCodeType::UPCE([0, 3, 6, 7, 4, 3, 3, 4]);
        let my_check_digit: i8 = 9;

        UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };
    }

    /// Checks if you can properly make the UPCCode structure with the
    /// **UPC-A** type.
    ///
    /// **NOTE: This is an invalid check code and is only running to see if you
    /// can properly create the structure with the given datatypes.**
    #[test]
    fn upc_code_struct_upca_make() {
        let my_code = UPCCodeType::UPCA([0, 3, 6, 7, 4, 3, 3, 4, 4, 3, 3, 4]);
        let my_check_digit: i8 = 9;

        UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };
    }

    /// Checks if an invalid code returns the right value when using the
    /// UPCCode.check_code() method.
    /// 
    /// **NOTE: Will be using an invalid UPC-E code for this test.**
    #[test]
    fn upc_code_invalidcheck() {
        let my_code = UPCCodeType::UPCE([0, 3, 6, 7, 4, 9, 9, 4]);
        let my_check_digit: i8 = 9;

        let my_upc_code_struct = UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };

        assert_eq!(false, my_upc_code_struct.check_code().unwrap());
    }

    /// Checks if a valid code is returning the right value when using the
    /// UPCCode.check_code() method.
    /// 
    /// **NOTE: This test is essentially an extention of the UPC-E test**
    #[test]
    fn upc_code_validcheck() {
        let my_code = UPCCodeType::UPCE([0, 3, 6, 7, 4, 3, 3, 4]);
        let my_check_digit: i8 = 9;

        let my_upc_code_struct = UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };

        assert_eq!(true, my_upc_code_struct.check_code().unwrap());
    }

    /// Checks that the UPCCode.check_code() method returns a Err() when given
    /// an overflowing value (in this case any other number then 0-9).
    #[test]
    fn upc_code_overflow_error() -> Result<(), ()> {
        let my_code = UPCCodeType::UPCE([0, 3, 6, 7, 4, 30, 3, 4]); // 30 should error
        let my_check_digit: i8 = 7; // Just random, it will fail anyway

        let my_upc_code_struct = UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };

        match my_upc_code_struct.check_code() {
            Err(_) => return Ok(()),
            Ok(_) => return Err(()),
        };
    }
}
