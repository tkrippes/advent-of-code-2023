use camel_cards::CamelCards;
use std::fs;

mod camel_cards;
mod hand;

pub fn part_1(file_name: &str) -> u64 {
    match try_get_camel_cards(file_name, false) {
        Some(camel_cards) => get_total_winnings(camel_cards),
        None => {
            println!("Failed to get camel cards");
            0
        }
    }
}

pub fn part_2(file_name: &str) -> u64 {
    match try_get_camel_cards(file_name, true) {
        Some(camel_cards) => get_total_winnings(camel_cards),
        None => {
            println!("Failed to get camel cards");
            0
        }
    }
}

fn try_get_camel_cards(file_name: &str, consider_jokers: bool) -> Option<CamelCards> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content
        .split('\n')
        .map(|file_line| file_line.trim())
        .collect();

    CamelCards::try_build(file_lines, consider_jokers)
}

fn get_total_winnings(camel_cards: CamelCards) -> u64 {
    let ranked_bids = camel_cards.get_ranked_bids();

    let mut total_winnings = 0;
    for (rank_index, bid) in ranked_bids.iter().enumerate() {
        let rank = rank_index + 1;
        total_winnings += *bid * rank as u64;
    }

    total_winnings
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
        assert_eq!(result, 5905);
    }
}
