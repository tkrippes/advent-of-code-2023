pub struct History {
    values: Vec<i32>,
}

impl History {
    pub fn try_build(input: &str) -> Option<Self> {
        let mut values = Vec::new();

        for value in input.split_whitespace() {
            match value.parse::<i32>() {
                Ok(value) => values.push(value),
                Err(value_parsing_error) => {
                    println!("Cannot build history, {}", value_parsing_error);
                    return None;
                }
            }
        }

        Some(History { values })
    }

    pub fn get_predicition_of_next_value(&self) -> i32 {
        let mut sequences = vec![self.values.clone()];

        while Self::is_there_a_non_zero_value_in_last_sequence(&sequences) {
            Self::fill_sequences(&mut sequences);
        }

        Self::fill_placeholders(&mut sequences);

        *sequences.first().unwrap().last().unwrap()
    }

    fn is_there_a_non_zero_value_in_last_sequence(sequences: &[Vec<i32>]) -> bool {
        match sequences.last() {
            Some(last_sequence) => last_sequence.iter().any(|value| *value != 0),
            None => false,
        }
    }

    fn fill_sequences(sequences: &mut Vec<Vec<i32>>) {
        let mut next_sequence = Vec::new();

        for index in 0..sequences.last().unwrap().len() - 1 {
            next_sequence
                .push(sequences.last().unwrap()[index + 1] - sequences.last().unwrap()[index]);
        }

        sequences.push(next_sequence);
    }

    fn fill_placeholders(sequences: &mut Vec<Vec<i32>>) {
        sequences.last_mut().unwrap().push(0);

        for index in (1..=sequences.len() - 1).rev() {
            let last_current_value = *sequences.get(index).unwrap().last().unwrap();
            let last_previous_value = *sequences.get(index - 1).unwrap().last().unwrap();

            sequences
                .get_mut(index - 1)
                .unwrap()
                .push(last_current_value + last_previous_value)
        }
    }
}
