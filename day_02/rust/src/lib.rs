use std::fs;
mod game;
use game::{Cube, Game, Subset};

pub fn part_1(file_name: &str) -> u32 {
    let subset_limit = Subset::from([(Cube::Red, 12), (Cube::Green, 13), (Cube::Blue, 14)]);

    let games = get_games(file_name);

    get_sum_of_possible_game_ids(games, subset_limit)
}

pub fn part_2(file_name: &str) -> u32 {
    let games = get_games(file_name);

    get_sum_of_power(games)
}

fn get_games(file_name: &str) -> Vec<Game> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content.split('\n').collect();

    let mut games = Vec::new();
    for line in file_lines {
        if let Some(game) = Game::try_build(line) {
            games.push(game);
        }
    }

    games
}

fn get_sum_of_possible_game_ids(games: Vec<Game>, subset_limit: Subset) -> u32 {
    let mut sum_of_possible_game_ids = 0;

    for game in games {
        if game.is_possible(&subset_limit) {
            sum_of_possible_game_ids += game.get_id();
        }
    }

    sum_of_possible_game_ids
}

fn get_sum_of_power(games: Vec<Game>) -> u32 {
    let mut power = 0;

    for game in games {
        power += game.get_power();
    }

    power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 2286);
    }
}
