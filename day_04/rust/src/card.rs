#[derive(Debug)]
pub struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    own_numbers: Vec<u32>,
}

impl Card {
    pub fn try_build(input: &str) -> Option<Self> {
        let id = Card::try_parse_id(input);
        let winning_numbers = Card::try_parse_winning_numbers(input);
        let own_numbers = Card::try_parse_own_numbers(input);

        if let (Some(id), Some(winning_numbers), Some(own_numbers)) =
            (id, winning_numbers, own_numbers)
        {
            Some(Card {
                id,
                winning_numbers,
                own_numbers,
            })
        } else {
            None
        }
    }

    fn try_parse_id(input: &str) -> Option<u32> {
        let splitted_input: Vec<&str> = input.split(':').collect();

        if let Some(card_input) = splitted_input.first() {
            let splitted_card_input: Vec<&str> = card_input.split_whitespace().collect();
            if let Some(card_number) = splitted_card_input.last() {
                match card_number.parse::<u32>() {
                    Ok(card_number) => Some(card_number),
                    Err(card_number_parsing_error) => {
                        println!("Could not parse card number, {}", card_number_parsing_error);
                        None
                    }
                }
            } else {
                println!("No card number found");
                None
            }
        } else {
            println!("No card input found");
            None
        }
    }

    fn try_parse_winning_numbers(input: &str) -> Option<Vec<u32>> {
        let splitted_input: Vec<&str> = input.split(':').collect();

        if let Some(numbers_input) = splitted_input.last() {
            let splitted_numbers_input: Vec<&str> = numbers_input.split('|').collect();
            if let Some(winning_numbers_input) = splitted_numbers_input.first() {
                let winning_numbers_input: Vec<&str> =
                    winning_numbers_input.split_whitespace().collect();

                let mut winning_numbers = Vec::new();
                for winning_number in winning_numbers_input {
                    match winning_number.parse::<u32>() {
                        Ok(winning_number) => winning_numbers.push(winning_number),
                        Err(winning_number_parsing_error) => {
                            println!(
                                "Could not parse winning number, {}",
                                winning_number_parsing_error
                            );
                            return None;
                        }
                    }
                }

                Some(winning_numbers)
            } else {
                println!("No winning numbers input found");
                None
            }
        } else {
            println!("No numbers input found");
            None
        }
    }

    fn try_parse_own_numbers(input: &str) -> Option<Vec<u32>> {
        let splitted_input: Vec<&str> = input.split(':').collect();

        if let Some(numbers_input) = splitted_input.last() {
            let splitted_numbers_input: Vec<&str> = numbers_input.split('|').collect();
            if let Some(own_numbers_input) = splitted_numbers_input.last() {
                let own_numbers_input: Vec<&str> = own_numbers_input.split_whitespace().collect();

                let mut own_numbers = Vec::new();
                for own_number in own_numbers_input {
                    match own_number.parse::<u32>() {
                        Ok(own_number) => own_numbers.push(own_number),
                        Err(own_number_parsing_error) => {
                            println!("Could not parse own number, {}", own_number_parsing_error);
                            return None;
                        }
                    }
                }

                Some(own_numbers)
            } else {
                println!("No won numbers input found");
                None
            }
        } else {
            println!("No numbers input found");
            None
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_number_of_own_winning_numbers(&self) -> u32 {
        self.own_numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as u32
    }
}
