mod almanac;

use almanac::Almanac;
use std::fs;

pub fn part_1(file_name: &str) -> u64 {
    let almanac = try_get_almanac(file_name);

    if let Some(almanac) = almanac {
        get_lowest_location_of_initial_seeds(&almanac)
    } else {
        println!("Failed to get almanac");
        0
    }
}

pub fn part_2(file_name: &str) -> u64 {
    todo!()
}

fn try_get_almanac(file_name: &str) -> Option<Almanac> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content
        .split('\n')
        .map(|file_line| file_line.trim())
        .collect();

    Almanac::try_build(file_lines)
}

fn get_lowest_location_of_initial_seeds(almanac: &Almanac) -> u64 {
    let locations = almanac.get_locations_from_seeds();

    if let Some(minimum_location) = locations.iter().min() {
        *minimum_location
    } else {
        println!("Failed to get minimum location");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 35);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 0);
    }
}
