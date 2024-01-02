use history::History;
use std::fs;

mod history;

pub fn part_1(file_name: &str) -> i32 {
    match try_get_histories(file_name) {
        Some(histories) => get_sum_of_extrapolated_values(histories),
        None => {
            println!("Failed to get histories");
            0
        }
    }
}

pub fn part_2(file_name: &str) -> i32 {
    todo!()
}

fn try_get_histories(file_name: &str) -> Option<Vec<History>> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content
        .split('\n')
        .map(|file_line| file_line.trim())
        .collect();

    let mut histories = Vec::new();

    for line in file_lines {
        match History::try_build(line) {
            Some(history) => histories.push(history),
            None => {
                println!("Cannot find history for {}", line);
                return None;
            }
        }
    }

    Some(histories)
}

fn get_sum_of_extrapolated_values(histories: Vec<History>) -> i32 {
    histories
        .iter()
        .map(|history| history.get_predicition_of_next_value())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 114);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 0);
    }
}
