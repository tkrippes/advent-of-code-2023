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
}

impl Card {
    fn try_build(character: char) -> Option<Self> {
        match character {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
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
        }
    }
}

const NUMBER_OF_CARDS: usize = 5;

#[derive(Clone, PartialEq, Eq, Ord)]
pub struct Hand {
    cards: [Card; NUMBER_OF_CARDS],
}

impl Hand {
    pub fn try_build(characters_input: &str) -> Option<Self> {
        let characters = characters_input.trim();

        if characters.len() == NUMBER_OF_CARDS {
            let mut cards: [Card; NUMBER_OF_CARDS] = [Card::Two; NUMBER_OF_CARDS];

            for (index, character) in characters.chars().enumerate() {
                match Card::try_build(character) {
                    Some(card) => cards[index] = card,
                    None => {
                        println!("Cannot parse hand");
                        return None;
                    }
                }
            }

            Some(Hand { cards })
        } else {
            println!(
                "Cannot parse hand, had {} instead of 5 cards",
                characters.len()
            );
            None
        }
    }

    fn get_type(&self) -> HandType {
        let sorted_card_count = self.get_sorted_card_count();

        match sorted_card_count.first() {
            Some(5) => HandType::FiveOfAKind,
            Some(4) => HandType::FourOfAKind,
            Some(3) => match sorted_card_count.get(1) {
                Some(2) => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            Some(2) => match sorted_card_count.get(1) {
                Some(2) => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        }
    }

    fn get_sorted_card_count(&self) -> Vec<u64> {
        let mut card_count = HashMap::new();

        for card in self.cards {
            *card_count.entry(card.get_char()).or_insert(0u64) += 1;
        }

        let mut card_count = card_count
            .values()
            .into_iter()
            .map(|value| *value)
            .collect::<Vec<u64>>();

        card_count.sort();
        card_count.reverse();

        card_count
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.get_type().cmp(&other.get_type()) {
            cmp::Ordering::Less => Some(cmp::Ordering::Less),
            cmp::Ordering::Equal => match self.cards.cmp(&other.cards) {
                cmp::Ordering::Less => Some(cmp::Ordering::Less),
                cmp::Ordering::Equal => Some(cmp::Ordering::Equal),
                cmp::Ordering::Greater => Some(cmp::Ordering::Greater),
            },
            cmp::Ordering::Greater => Some(cmp::Ordering::Greater),
        }
    }
}
