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
}

const NUMBER_OF_CARDS: usize = 5;

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
}
