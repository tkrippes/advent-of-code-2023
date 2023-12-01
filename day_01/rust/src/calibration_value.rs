pub struct CalibrationValue {
    first_digit: u32,
    last_digit: u32,
}

impl CalibrationValue {
    pub fn try_build(input: &str) -> Option<Self> {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;

        for character in input.chars() {
            if character.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(character.to_digit(10).unwrap());
                }
                last_digit = Some(character.to_digit(10).unwrap());
            }
        }

        if first_digit.is_some() && last_digit.is_some() {
            Some(CalibrationValue { first_digit: first_digit.unwrap(), last_digit: last_digit.unwrap() })
        } else {
            None
        }
    }

    pub fn get_value(&self) -> u32 {
        self.first_digit * 10 + self.last_digit
    }
}