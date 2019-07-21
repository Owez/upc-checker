use std::io;

/// UPCCode is the frontend struct for the upc-checker module, allowing easy
/// access with an i8 vector (known as `code`) and one straggler i8 check
/// code (known as `check_code`)
///
/// ## Examples
/// 
/// ```rust
/// my_code_vector: Vec<i8> = vec![3, 5, 7, 4]; // NOTE digits should be 0-9
/// my_check_digit: i8 = 2; // NOTE check digit should be 0-9
///
/// let my_upc_code = UPCCode { my_code_vector, my_check_digit };
/// println!("Is `code` valid?: {}", my_upc_code.check_code().unwrap());
/// ```
#[allow(dead_code)]
pub struct UPCCode {
    pub code: Vec<i8>,
    pub check_digit: i8,
}

impl UPCCode {
    /// Validates that all data in the `UPCCode` is 1-char in len (0-9)
    /// instead of the -255 to 255 of an i8
    ///
    /// ## Examples
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
    #[allow(dead_code)]
    fn validate_nums(&self) -> bool {
        for code in &self.code {
            if !is_1_digit(*code) {
                return false;
            }
        }

        is_1_digit(self.check_digit)
    }

    /// Adds odd and even numbers (using %) to a 2-len
    /// i8 tuple `(even, odd)` respectivly
    ///
    /// ## Examples
    /// 
    /// ```rust
    /// let (even_nums, odd_nums) = &self.add_even_odd_total();
    /// println!("Even: {0}, Odd: {1}", even_nums, odd_nums);
    /// ```
    #[allow(dead_code)]
    fn add_even_odd_total(&self) -> (i8, i8) {
        let mut result: (i8, i8) = (0, 0);

        for code in &self.code {
            if code % 2 == 0 {
                result.0 += code;
            } else {
                result.1 += code;
            }
        }

        result
    }

    /// Frontend function for checking with given `check_code` for the
    /// `UPCCode` structure.
    ///
    /// ## Examples
    /// 
    /// ```rust
    /// let my_struct = UPCCode {code: vec![3, 5, 7, 4], check_digit: 3};
    /// println!("Result: {}", my_struct.check_code().unwrap());
    /// ```
    #[allow(dead_code)]
    pub fn check_code(&self) -> Result<bool, io::Error> {
        if !&self.validate_nums() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "A number used isn\'t 1 digit!",
            ));
        }

        let mut total: i8 = 0;
        let (even_nums, odd_nums) = &self.add_even_odd_total();

        total += ((odd_nums * 3) + even_nums) % 10;

        if (total == 0 && self.check_digit == 0) || (10 - total == self.check_digit) {
            return Ok(true);
        }

        Ok(false)
    }
}

/// Checks if a given i8 is 1 character wide (0-9)
///
/// ## Examples
/// 
/// ```rust
/// let digit_to_check: i8 = 9;
/// let should_be_invalid: i8 = 102;
/// println!(
///     "Is `digit_to_check` valid?: {}. Is `should_be_invalid` valid?: {}",
///     is_1_digit(digit_to_check),
///     is_1_digit(should_be_invalid)
/// )
/// ```
fn is_1_digit(num: i8) -> bool {
    !(num < 0 || num > 9)
}
