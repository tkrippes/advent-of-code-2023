use std::fs;

pub fn part_1(file_name: &str) -> u32 {
    let file_content = fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content.split("\n").collect();

    let mut calibration_values: Vec<u32> = Vec::new();
    for line in &file_lines {
        let mut first_digit: Option<u32> = None;
        let mut last_digit: Option<u32> = None;
        for character in line.chars() {
            if character.is_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(character.to_digit(10).unwrap());
                }
                last_digit = Some(character.to_digit(10).unwrap());
            }
        }
        if first_digit.is_some() && last_digit.is_some() {
            calibration_values.push(first_digit.unwrap() * 10 + last_digit.unwrap());
        }
    }

    return calibration_values.iter().sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_file_calibration_value() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 142);
    }
}
