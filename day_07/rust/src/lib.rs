use camel_cards::CamelCards;
use std::fs;

mod camel_cards;
mod hand;

pub fn part_1(file_name: &str) -> u64 {
    match try_get_camel_cards(file_name) {
        Some(camel_cards) => get_total_winnings(&camel_cards),
        None => {
            println!("Failed to get camel cards");
            0
        }
    }
}

pub fn part_2(file_name: &str) -> u64 {
    todo!()
}

fn try_get_camel_cards(file_name: &str) -> Option<CamelCards> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content
        .split('\n')
        .map(|file_line| file_line.trim())
        .collect();

    CamelCards::try_build(file_lines)
}

fn get_total_winnings(camel_cards: &CamelCards) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 0);
    }
}
