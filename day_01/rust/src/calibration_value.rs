use std::collections::HashMap;

#[derive(Copy, Clone)]
struct IndexedDigit {
    digit: u32,
    index: usize,
}

impl IndexedDigit {
    fn build(digit: u32, index: usize) -> Self {
        IndexedDigit { digit, index }
    }
}

pub struct CalibrationValue {
    first_digit: u32,
    last_digit: u32,
}

impl CalibrationValue {
    pub fn try_build(input: &str, consider_letters: bool) -> Option<Self> {
        let mut first_digit = Self::find_first_digit(input);

        if consider_letters {
            let first_letter_digit = Self::find_first_letter_digit(input);

            first_digit = match (first_digit, first_letter_digit) {
                (Some(digit), Some(letter_digit)) => {
                    if digit.index < letter_digit.index {
                        Some(digit)
                    } else {
                        Some(letter_digit)
                    }
                }
                (Some(digit), None) => Some(digit),
                (None, Some(letter_digit)) => Some(letter_digit),
                (None, None) => None
            };
        }

        let mut last_digit = Self::find_last_digit(input);

        if consider_letters {
            let last_letter_digit = Self::find_last_letter_digit(input);

            last_digit = match (last_digit, last_letter_digit) {
                (Some(digit), Some(letter_digit)) => {
                    if digit.index > letter_digit.index {
                        Some(digit)
                    } else {
                        Some(letter_digit)
                    }
                }
                (Some(digit), None) => Some(digit),
                (None, Some(letter_digit)) => Some(letter_digit),
                (None, None) => None
            };
        }

        if first_digit.is_some() && last_digit.is_some() {
            Some(CalibrationValue { first_digit: first_digit.unwrap().digit, last_digit: last_digit.unwrap().digit })
        } else {
            None
        }
    }

    fn find_first_digit(input: &str) -> Option<IndexedDigit> {
        for (index, character) in input.chars().enumerate() {
            if character.is_digit(10) {
                return Some(IndexedDigit::build(character.to_digit(10).unwrap(), index));
            }
        }

        None
    }

    fn find_first_letter_digit(input: &str) -> Option<IndexedDigit> {
        let mut first_digit: Option<IndexedDigit> = None;
        for (letter, digit) in &Self::get_digit_map() {
            let position = input.find(letter);
            if position.is_some() {
                if first_digit.is_none() || position.unwrap() < first_digit.unwrap().index {
                    first_digit = Some(IndexedDigit::build(*digit, position.unwrap()));
                }
            }
        }

        first_digit
    }

    fn find_last_digit(input: &str) -> Option<IndexedDigit> {
        let mut last_digit: Option<IndexedDigit> = None;

        for (index, character) in input.chars().enumerate() {
            if character.is_digit(10) {
                last_digit = Some(IndexedDigit::build(character.to_digit(10).unwrap(), index));
            }
        }

        last_digit
    }

    fn find_last_letter_digit(input: &str) -> Option<IndexedDigit> {
        let mut last_digit: Option<IndexedDigit> = None;
        for (letter, digit) in &Self::get_digit_map() {
            let position = input.rfind(letter);
            if position.is_some() {
                if last_digit.is_none() || position.unwrap() > last_digit.unwrap().index {
                    last_digit = Some(IndexedDigit::build(*digit, position.unwrap()));
                }
            }
        }

        last_digit
    }

    fn get_digit_map() -> HashMap<&'static str, u32> {
        HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9)
        ])
    }

    pub fn get_value(&self) -> u32 {
        self.first_digit * 10 + self.last_digit
    }
}