use std::cmp;
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn try_build(character: char, consider_jokers: bool) -> Option<Self> {
        match character {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' if !consider_jokers => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            'J' if consider_jokers => Some(Card::Joker),
            _ => {
                println!("Cannot parse character from {}", character);
                None
            }
        }
    }

    fn get_char(&self) -> char {
        match self {
            Card::Ace => 'A',
            Card::King => 'K',
            Card::Queen => 'Q',
            Card::Jack => 'J',
            Card::Ten => 'T',
            Card::Nine => '9',
            Card::Eight => '8',
            Card::Seven => '7',
            Card::Six => '6',
            Card::Five => '5',
            Card::Four => '4',
            Card::Three => '3',
            Card::Two => '2',
            Card::Joker => 'J',
        }
    }
}

const NUMBER_OF_CARDS: usize = 5;

#[derive(Clone, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; NUMBER_OF_CARDS],
    consider_jokers: bool,
}

impl Hand {
    pub fn try_build(characters_input: &str, consider_jokers: bool) -> Option<Self> {
        let characters = characters_input.trim();

        if characters.len() == NUMBER_OF_CARDS {
            let mut cards: [Card; NUMBER_OF_CARDS] = [Card::Two; NUMBER_OF_CARDS];

            for (index, character) in characters.chars().enumerate() {
                match Card::try_build(character, consider_jokers) {
                    Some(card) => cards[index] = card,
                    None => {
                        println!("Cannot parse hand");
                        return None;
                    }
                }
            }

            Some(Hand {
                cards,
                consider_jokers,
            })
        } else {
            println!(
                "Cannot parse hand, got {} instead of 5 cards",
                characters.len()
            );
            None
        }
    }

    fn get_type(&self) -> HandType {
        let descending_card_counts = self.get_descending_card_counts();

        match descending_card_counts.first() {
            Some(5) => HandType::FiveOfAKind,
            Some(4) => HandType::FourOfAKind,
            Some(3) => match descending_card_counts.get(1) {
                Some(2) => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            Some(2) => match descending_card_counts.get(1) {
                Some(2) => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        }
    }

    fn get_descending_card_counts(&self) -> Vec<u64> {
        let mut card_counts_map = self.get_card_counts_map();

        let number_of_jokers = self.get_and_remove_number_of_jokers(&mut card_counts_map);

        let mut descending_card_counts = Self::get_descending_map_values(card_counts_map);

        self.add_number_of_jokers_to_highest_card_count(
            &mut descending_card_counts,
            number_of_jokers,
        );

        descending_card_counts
    }

    fn get_card_counts_map(&self) -> HashMap<char, u64> {
        let mut card_count_map = HashMap::new();

        for card in self.cards {
            *card_count_map.entry(card.get_char()).or_insert(0u64) += 1;
        }

        card_count_map
    }

    fn get_and_remove_number_of_jokers(&self, card_counts_map: &mut HashMap<char, u64>) -> u64 {
        if self.consider_jokers {
            card_counts_map.remove(&'J').unwrap_or_default()
        } else {
            0
        }
    }

    fn get_descending_map_values(map: HashMap<char, u64>) -> Vec<u64> {
        let mut map_values = map.values().copied().collect::<Vec<u64>>();

        map_values.sort();
        map_values.reverse();

        map_values
    }

    fn add_number_of_jokers_to_highest_card_count(
        &self,
        descending_card_counts: &mut Vec<u64>,
        number_of_jokers: u64,
    ) {
        if self.consider_jokers {
            match descending_card_counts.first_mut() {
                Some(highest_card_count) => *highest_card_count += number_of_jokers,
                None => descending_card_counts.push(number_of_jokers),
            };
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.get_type().cmp(&other.get_type()) {
            cmp::Ordering::Less => cmp::Ordering::Less,
            cmp::Ordering::Equal => match self.cards.cmp(&other.cards) {
                cmp::Ordering::Less => cmp::Ordering::Less,
                cmp::Ordering::Equal => cmp::Ordering::Equal,
                cmp::Ordering::Greater => cmp::Ordering::Greater,
            },
            cmp::Ordering::Greater => cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
