use regex::Regex;
use std::collections::HashMap;

enum Cube {
    Red,
    Green,
    Blue,
}

type Subset = HashMap<u32, Cube>;

pub struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    pub fn try_build(input: &str) -> Option<Self> {
        let id = Self::get_id(input);
        let subsets = Self::get_subsets(input);

        if let (Some(id), Some(subsets)) = (id, subsets) {
            Some(Game { id, subsets })
        } else {
            None
        }
    }

    fn get_id(input: &str) -> Option<u32> {
        let game_regex = Regex::new(r"Game (\d+)+:");

        match game_regex {
            Ok(game_regex) => match game_regex.captures(input) {
                Some(captures) => {
                    let game_id_string = captures.extract::<1>().1[0];
                    if let Ok(game_id) = game_id_string.parse::<u32>() {
                        Some(game_id)
                    } else {
                        println!("Cannot convert game id to u32 ({})", game_id_string);
                        None
                    }
                }
                None => {
                    println!("No game id found in \"{}\"", input);
                    None
                }
            },
            Err(game_regex_error) => {
                println!("Cannot get valid game regex, {}", game_regex_error);
                None
            }
        }
    }

    fn get_subsets(input: &str) -> Option<Vec<Subset>> {
        // TODO omit part before ":"
        // TODO split by ";" and get subsets
        let subset_regex = Regex::new(&format!(
            r"(\d+)+ {0}[, (\d+)+ {0}]?[, (\d+)+ {0}]?",
            Self::get_color_regex()
        ));

        if let Err(subset_regex_error) = subset_regex {
            println!("Cannot get valid subset regex, {}", subset_regex_error);
            return None;
        }

        None
    }

    fn get_color_regex() -> &'static str {
        "red|green|blue"
    }
}
