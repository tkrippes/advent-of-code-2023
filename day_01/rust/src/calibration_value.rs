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
    pub fn try_build(input: &str, consider_letter_digits: bool) -> Option<Self> {
        let first_indexed_digit = Self::find_first_indexed_digit(input, consider_letter_digits);
        let last_indexed_digit = Self::find_last_indexed_digit(input, consider_letter_digits);

        if let (Some(first_indexed_digit), Some(last_indexed_digit)) =
            (first_indexed_digit, last_indexed_digit)
        {
            Some(CalibrationValue {
                first_digit: first_indexed_digit.digit,
                last_digit: last_indexed_digit.digit,
            })
        } else {
            None
        }
    }

    fn find_first_indexed_digit(input: &str, consider_letter_digits: bool) -> Option<IndexedDigit> {
        let mut first_indexed_digit: Option<IndexedDigit> = None;

        for (index, character) in input.chars().enumerate() {
            if character.is_ascii_digit() {
                first_indexed_digit =
                    Some(IndexedDigit::build(character.to_digit(10).unwrap(), index));
                break;
            }
        }

        if consider_letter_digits {
            let first_indexed_letter_digit = Self::find_first_indexed_letter_digit(input);

            first_indexed_digit = match (first_indexed_digit, first_indexed_letter_digit) {
                (Some(indexed_digit), Some(indexed_letter_digit)) => {
                    if indexed_digit.index < indexed_letter_digit.index {
                        Some(indexed_digit)
                    } else {
                        Some(indexed_letter_digit)
                    }
                }
                (Some(indexed_digit), None) => Some(indexed_digit),
                (None, Some(indexed_letter_digit)) => Some(indexed_letter_digit),
                (None, None) => None,
            };
        }

        first_indexed_digit
    }

    fn find_first_indexed_letter_digit(input: &str) -> Option<IndexedDigit> {
        let mut first_indexed_digit: Option<IndexedDigit> = None;

        for (letter_digit, digit) in &Self::get_digit_map() {
            let position = input.find(letter_digit);

            match (position, first_indexed_digit) {
                (Some(position), Some(indexed_digit)) if position < indexed_digit.index => {
                    first_indexed_digit = Some(IndexedDigit::build(*digit, position))
                }
                (Some(position), None) => {
                    first_indexed_digit = Some(IndexedDigit::build(*digit, position))
                }
                (_, _) => (),
            }
        }

        first_indexed_digit
    }

    fn find_last_indexed_digit(input: &str, consider_letter_digits: bool) -> Option<IndexedDigit> {
        let mut last_indexed_digit: Option<IndexedDigit> = None;

        for (index, character) in input.chars().enumerate() {
            if character.is_ascii_digit() {
                last_indexed_digit =
                    Some(IndexedDigit::build(character.to_digit(10).unwrap(), index));
            }
        }

        if consider_letter_digits {
            let last_indexed_letter_digit = Self::find_last_indexed_letter_digit(input);

            last_indexed_digit = match (last_indexed_digit, last_indexed_letter_digit) {
                (Some(indexed_digit), Some(letter_indexed_digit)) => {
                    if indexed_digit.index > letter_indexed_digit.index {
                        Some(indexed_digit)
                    } else {
                        Some(letter_indexed_digit)
                    }
                }
                (Some(indexed_digit), None) => Some(indexed_digit),
                (None, Some(letter_indexed_digit)) => Some(letter_indexed_digit),
                (None, None) => None,
            };
        }

        last_indexed_digit
    }

    fn find_last_indexed_letter_digit(input: &str) -> Option<IndexedDigit> {
        let mut last_indexed_digit: Option<IndexedDigit> = None;

        for (letter_digit, digit) in &Self::get_digit_map() {
            let position = input.rfind(letter_digit);

            match (position, last_indexed_digit) {
                (Some(position), Some(indexed_digit)) if position > indexed_digit.index => {
                    last_indexed_digit = Some(IndexedDigit::build(*digit, position))
                }
                (Some(position), None) => {
                    last_indexed_digit = Some(IndexedDigit::build(*digit, position))
                }
                (_, _) => (),
            }
        }

        last_indexed_digit
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
            ("nine", 9),
        ])
    }

    pub fn get_value(&self) -> u32 {
        self.first_digit * 10 + self.last_digit
    }
}
