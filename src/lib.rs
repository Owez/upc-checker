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
    fn split_upc_even_odd(&self) -> (u8, u8) {
        let mut even_odd: (u8, u8) = (0, 0);

        for upc_code in self.get_upc_slice() {
            if upc_code % 2 == 0 {
                even_odd.0 += *upc_code as u8;
            } else {
                even_odd.1 += *upc_code as u8;
            }
        }

        even_odd
    }

    pub fn check_upc(&self) -> Result<bool, UPCCodeError> {
        match self.validate_upc_overflow() {
            Err(x) => return Err(x),
            Ok(_) => (),
        };

        let total: u16 = 0;
        let even_odd_split = self.split_upc_even_odd();

        // TODO more

        Ok(false)
    }
}

fn is_1_digit(digit: i8) -> bool {
    digit > 0 && digit < 9
}
