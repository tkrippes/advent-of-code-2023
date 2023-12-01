mod calibration_value;

use std::fs;
use calibration_value::CalibrationValue;

pub fn part_1(file_name: &str) -> u32 {
    let file_content = fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content.split("\n").collect();

    let mut calibration_values: Vec<CalibrationValue> = Vec::new();
    for line in &file_lines {
        let calibration_value = CalibrationValue::try_build(line);

        if calibration_value.is_some() {
            calibration_values.push(calibration_value.unwrap());
        }
    }

    return calibration_values.iter().map(|calibration_value| calibration_value.get_value()).sum();
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
