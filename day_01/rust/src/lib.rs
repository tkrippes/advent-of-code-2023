mod calibration_value;

use std::fs;
use calibration_value::CalibrationValue;

pub fn part_1(file_name: &str) -> u32 {
    let calibration_values = get_calibration_values(file_name, false);

    return calibration_values.iter().map(|calibration_value| calibration_value.get_value()).sum();
}

pub fn part_2(file_name: &str) -> u32 {
    let calibration_values = get_calibration_values(file_name, true);

    return calibration_values.iter().map(|calibration_value| calibration_value.get_value()).sum();
}

fn get_calibration_values(file_name: &str, consider_letters: bool) -> Vec<CalibrationValue> {
    let file_content = fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content.split("\n").collect();

    let mut calibration_values: Vec<CalibrationValue> = Vec::new();
    for line in &file_lines {
        let calibration_value = CalibrationValue::try_build(line, consider_letters);

        if calibration_value.is_some() {
            calibration_values.push(calibration_value.unwrap());
        }
    }

    calibration_values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_file_calibration_value() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn test_input_file_2_calibration_value() {
        let result = part_2("../input/test_input_2.txt");
        assert_eq!(result, 281);
    }
}
