use std::fs;
mod game;
use game::Game;

pub fn part_1(file_name: &str) -> u32 {
    let games = get_games(file_name);

    todo!()
}

pub fn part_2(file_name: &str) -> u32 {
    todo!()
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
        assert_eq!(result, 0);
    }
}
