pub enum UPCCodeError {
    UPCCodeOverflow,
    CheckDigitOverflow,
}

pub enum UPCCodeStandard {
    UPCA([i8; 12]),
    UPCE([i8; 8]),
}

pub struct UPCCode {
    upc: UPCCodeStandard,
    check_digit: i8,
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
