mod card;

use card::Card;
use std::{collections::HashMap, fs};

pub fn part_1(file_name: &str) -> u32 {
    let cards = try_get_cards(file_name);

    get_points(cards)
}

pub fn part_2(file_name: &str) -> u32 {
    let cards = try_get_cards(file_name);

    get_number_of_scratchcards(cards)
}

fn try_get_cards(file_name: &str) -> Vec<Card> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content
        .split('\n')
        .map(|file_line| file_line.trim())
        .collect();

    let mut cards = Vec::new();
    for line in file_lines {
        if let Some(card) = Card::try_build(line) {
            cards.push(card);
        }
    }

    cards
}

fn get_points(cards: Vec<Card>) -> u32 {
    let mut points = 0;

    for card in cards {
        let number_of_own_winning_numbers = card.get_number_of_own_winning_numbers();

        if number_of_own_winning_numbers > 0 {
            points += u32::pow(2, number_of_own_winning_numbers - 1);
        }
    }

    points
}

fn get_number_of_scratchcards(cards: Vec<Card>) -> u32 {
    let max_card_id = cards.last().unwrap().get_id();
    let mut number_of_scratchcards = HashMap::new();

    for card in &cards {
        number_of_scratchcards.insert(card.get_id(), 1);
    }

    for card in &cards {
        let card_id = card.get_id();
        let number_of_own_winning_numbers = card.get_number_of_own_winning_numbers();

        for _ in 0..*number_of_scratchcards.get(&card_id).unwrap() {
            for i in 0..number_of_own_winning_numbers {
                let new_card_id = card_id + i + 1;

                if new_card_id <= max_card_id {
                    *number_of_scratchcards.get_mut(&new_card_id).unwrap() += 1;
                }
            }
        }
    }

    number_of_scratchcards.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 30);
    }
}
