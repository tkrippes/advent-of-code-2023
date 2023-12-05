mod engine;

use std::fs;

use engine::Engine;

pub fn part_1(file_name: &str) -> u32 {
    let engine = try_get_engine(file_name);

    if let Some(engine) = engine {
        get_sum_of_part_numbers(engine)
    } else {
        println!("Failed to get engine");
        0
    }
}

pub fn part_2(file_name: &str) -> u32 {
    todo!()
}

fn try_get_engine(file_name: &str) -> Option<Engine> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content.split('\n').collect();

    Engine::try_build(file_lines)
}

fn get_sum_of_part_numbers(engine: Engine) -> u32 {
    let part_numbers = engine.get_all_part_numbers();

    part_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 4361);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 0);
    }
}
