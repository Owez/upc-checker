use std::io::{Error, ErrorKind};

/// Testing module for the main UPCCode struct along with its helper funcs.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the helper function is_1_digit returns a bool of true if it
    /// is 1-char long (0-9) and a false if it is any other number in the
    /// allowed i8 memory space.
    #[test]
    fn check_is_1_digit() {
        assert_eq!(true, is_1_digit(0));
        assert_eq!(true, is_1_digit(9));
        assert_eq!(false, is_1_digit(10));
        assert_eq!(false, is_1_digit(-2));
    }

    /// Checks if you can properly make the UPCCode structure.
    ///
    /// **NOTE: This is an invalid check code and is only running to see if you
    /// can properly create the structure with the given datatypes.**
    #[test]
    fn check_upc_code_struct_make() {
        let my_code: Vec<i8> = vec![0, 3, 6, 7, 4, 3, 3, 4];
        let my_check_digit: i8 = 9;

        UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };
    }

    /// Checks if an invalid code returns the right value when using the
    /// UPCCode.check_code() method.
    #[test]
    fn check_upc_code_invalidcheck() {
        let my_code: Vec<i8> = vec![0, 3, 6, 7, 4, 3, 3, 4];
        let my_check_digit: i8 = 9;

        let my_upc_code_struct = UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };

        assert_eq!(false, my_upc_code_struct.check_code().unwrap());
    }

    /// Checks if a valid code is returning the right value when using the
    /// UPCCode.check_code() method.
    #[test]
    fn check_upc_code_validcheck() {
        let my_code: Vec<i8> = vec![0, 3, 6, 0, 0, 0, 2, 4, 1, 4, 5];
        let my_check_digit: i8 = 7;

        let my_upc_code_struct = UPCCode {
            code: my_code,
            check_digit: my_check_digit,
        };

        assert_eq!(true, my_upc_code_struct.check_code().unwrap());
    }
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
/// my_code_vector: Vec<i8> = vec![3, 5, 7, 4]; // NOTE digits should be 0-9
/// my_check_digit: i8 = 2; // NOTE check digit should be 0-9
///
/// let my_upc_code = UPCCode {
///     code: my_code_vector,
///     check_digit: my_check_digit
/// };
/// 
/// println!("Is `code` valid?: {}", my_upc_code.check_code().unwrap());
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct UPCCode {
    pub code: Vec<i8>,
    pub check_digit: i8,
}

impl UPCCode {
    /// Validates that all data in the `UPCCode` is 1-char in len (0-9)
    /// instead of the -255 to 255 of an i8
    ///
    /// # Examples
    ///
    /// ```rust
    /// let is_valid_nums = &self.validate_nums();
    ///
    /// if !&self.validate_nums() {
    ///     return Err(io::Error::new(
    ///         io::ErrorKind::Other,
    ///         "A number used isn\'t 1 digit!",
    ///     ));
    /// }
    ///
    /// // Numbers are valid
    /// ```
    fn validate_nums(&self) -> bool {
        for code in self.code.iter() {
            if !is_1_digit(*code) {
                return false;
            }
        }

        is_1_digit(self.check_digit)
    }

    /// Adds odd and even numbers (using %) to a 2-len
    /// u8 tuple `(even, odd)` respectivly
    ///
    /// # Examples
    ///
    /// ```rust
    /// let (even_nums, odd_nums) = &self.add_even_odd_total();
    /// println!("Even: {0}, Odd: {1}", even_nums, odd_nums);
    /// ```
    fn add_even_odd_total(&self) -> (u8, u8) {
        let mut result: (u8, u8) = (0, 0);

        for code in self.code.iter() {
            if code % 2 == 0 {
                result.0 += *code as u8;
            } else {
                result.1 += *code as u8;
            }
        }

        result
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
    /// let my_struct = UPCCode {code: vec![3, 5, 7, 4], check_digit: 3};
    /// println!("Result: {}", my_struct.check_code().unwrap());
    /// ```
    pub fn check_code(&self) -> Result<bool, Error> {
        if !self.validate_nums() {
            return Err(Error::new(
                ErrorKind::Other,
                "A number used isn\'t 1 digit!",
            ));
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
///
/// # Examples
///
/// ```rust
/// let digit_to_check: i8 = 9;
/// let should_be_invalid: i8 = 102;
///
/// println!(
///     "Is `digit_to_check` valid?: {}. Is `should_be_invalid` valid?: {}",
///     is_1_digit(digit_to_check),
///     is_1_digit(should_be_invalid)
/// )
/// ```
fn is_1_digit(num: i8) -> bool {
    !(num < 0 || num > 9)
}
