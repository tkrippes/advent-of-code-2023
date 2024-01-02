use std::collections::VecDeque;

pub struct History {
    values: VecDeque<i32>,
}

impl History {
    pub fn try_build(input: &str) -> Option<Self> {
        let mut values = VecDeque::new();

        for value in input.split_whitespace() {
            match value.parse::<i32>() {
                Ok(value) => values.push_back(value),
                Err(value_parsing_error) => {
                    println!("Cannot build history, {}", value_parsing_error);
                    return None;
                }
            }
        }

        Some(History { values })
    }

    pub fn get_prediction_of_next_value(&self) -> i32 {
        let mut sequences = vec![self.values.clone()];

        while Self::is_there_a_non_zero_value_in_last_sequence(&sequences) {
            Self::add_next_sequence(&mut sequences);
        }

        Self::fill_next_placeholders(&mut sequences);

        match sequences.first() {
            Some(first_sequence) => match first_sequence.back() {
                Some(next_value) => *next_value,
                None => 0,
            },
            None => 0,
        }
    }

    pub fn get_prediction_of_previous_value(&self) -> i32 {
        let mut sequences = vec![self.values.clone()];

        while Self::is_there_a_non_zero_value_in_last_sequence(&sequences) {
            Self::add_next_sequence(&mut sequences);
        }

        Self::fill_previous_placeholders(&mut sequences);

        match sequences.first() {
            Some(first_sequence) => match first_sequence.front() {
                Some(previous_value) => *previous_value,
                None => 0,
            },
            None => 0,
        }
    }

    fn is_there_a_non_zero_value_in_last_sequence(sequences: &[VecDeque<i32>]) -> bool {
        match sequences.last() {
            Some(last_sequence) => last_sequence.iter().any(|value| *value != 0),
            None => false,
        }
    }

    fn add_next_sequence(sequences: &mut Vec<VecDeque<i32>>) {
        if let Some(last_sequence) = sequences.last() {
            let mut next_sequence = VecDeque::new();

            for index in 0..last_sequence.len() - 1 {
                next_sequence.push_back(last_sequence[index + 1] - last_sequence[index]);
            }

            sequences.push(next_sequence);
        }
    }

    fn fill_next_placeholders(sequences: &mut Vec<VecDeque<i32>>) {
        if let Some(last_sequence) = sequences.last_mut() {
            last_sequence.push_back(0);
        }

        for index in (1..=sequences.len() - 1).rev() {
            let last_previous_value = *sequences.get(index - 1).unwrap().back().unwrap();
            let last_current_value = *sequences.get(index).unwrap().back().unwrap();

            sequences
                .get_mut(index - 1)
                .unwrap()
                .push_back(last_previous_value + last_current_value)
        }
    }

    fn fill_previous_placeholders(sequences: &mut Vec<VecDeque<i32>>) {
        if let Some(last_sequence) = sequences.last_mut() {
            last_sequence.push_front(0);
        }

        for index in (1..=sequences.len() - 1).rev() {
            let first_previous_value = *sequences.get(index - 1).unwrap().front().unwrap();
            let first_current_value = *sequences.get(index).unwrap().front().unwrap();

            sequences
                .get_mut(index - 1)
                .unwrap()
                .push_front(first_previous_value - first_current_value)
        }
    }
}
