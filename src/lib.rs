use std::io;

#[allow(dead_code)]
pub struct UPCCode {
    pub code: Vec<i8>,
    pub check_digit: i8,
}

impl UPCCode {
    #[allow(dead_code)]
    fn validate_nums(&self) -> bool {
        for code in &self.code {
            if !is_1_digit(*code) {
                return false;
            }
        }

        is_1_digit(self.check_digit)
    }

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

fn is_1_digit(num: i8) -> bool {
    !(num < 0 || num > 9)
}
