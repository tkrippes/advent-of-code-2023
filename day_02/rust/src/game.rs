use regex::Regex;
use std::{collections::HashMap, hash::Hash};

#[derive(PartialEq, Eq, Hash)]
pub enum Cube {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Cube {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "red" => Ok(Cube::Red),
            "green" => Ok(Cube::Green),
            "blue" => Ok(Cube::Blue),
            _ => Err(stringify!("Cannot convert Cube from {}", value)),
        }
    }
}

pub type Subset = HashMap<Cube, u32>;

pub struct Game {
    id: u32,
    subsets: Vec<Subset>,
}

impl Game {
    pub fn try_build(input: &str) -> Option<Self> {
        let id = Self::try_parse_id(input);
        let subsets = Self::try_parse_subsets(input);

        if let (Some(id), Some(subsets)) = (id, subsets) {
            Some(Game { id, subsets })
        } else {
            None
        }
    }

    fn try_parse_id(input: &str) -> Option<u32> {
        let game_regex = Regex::new(r"Game (\d+)+:");

        match game_regex {
            Ok(game_regex) => match game_regex.captures(input) {
                Some(captures) => {
                    let game_id = &captures[1];
                    if let Ok(game_id) = game_id.parse::<u32>() {
                        Some(game_id)
                    } else {
                        println!("Cannot convert game id to u32 ({})", game_id);
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

    fn try_parse_subsets(input: &str) -> Option<Vec<Subset>> {
        let splitted_input: Vec<&str> = input.split(':').collect();
        if splitted_input.len() != 2 {
            println!("Cannot find subset input");
            return None;
        }

        let subsets_input = splitted_input[1];

        let subset_entry_regex = match Regex::new(&format!(r"(\d+)+ {}", Self::get_color_regex())) {
            Ok(subset_entry_regex) => subset_entry_regex,
            Err(subset_entry_regex_error) => {
                println!(
                    "Cannot get valid subset entry regex, {}",
                    subset_entry_regex_error
                );
                return None;
            }
        };

        let mut subsets = Vec::new();

        for subset_input in subsets_input.split(';') {
            let mut subset = Subset::new();

            for subset_entry_input in subset_input.split(',') {
                match subset_entry_regex.captures(subset_entry_input) {
                    Some(captures) => {
                        let amount = &captures[1].parse::<u32>();
                        let cube = Cube::try_from(&captures[2]);

                        if let (Ok(amount), Ok(cube)) = (amount, cube) {
                            subset.insert(cube, *amount);
                        } else {
                            println!("Subset entry is invalid, \"{}\"", subset_entry_input);
                        }
                    }
                    None => {
                        println!("No subset entry found in \"{}\"", subset_entry_input);
                    }
                }
            }

            subsets.push(subset);
        }

        if subsets.is_empty() {
            None
        } else {
            Some(subsets)
        }
    }

    fn get_color_regex() -> &'static str {
        "(red|green|blue)"
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn is_possible(&self, subset_limit: &Subset) -> bool {
        for subset in &self.subsets {
            for (cube, limit_amount) in subset_limit {
                if let Some(amount) = subset.get(cube) {
                    if amount > limit_amount {
                        return false;
                    }
                }
            }
        }

        true
    }
}
