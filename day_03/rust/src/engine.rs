enum Part {
    Empty,
    Digit(u32),
    Symbol(char),
}

impl TryFrom<char> for Part {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Part::Empty),
            n @ '0'..='9' => Ok(Part::Digit(n.to_digit(10).unwrap())),
            c if c.is_ascii_punctuation() => Ok(Part::Symbol(c)),
            _ => Err(format!("Cannot parse Part from {}", value)),
        }
    }
}

#[derive(PartialEq)]
struct PartNumber {
    value: u32,
    row_index: usize,
    column_indices: Vec<usize>,
}

pub struct Engine {
    schematic: Vec<Vec<Part>>,
}

impl Engine {
    pub fn try_build(input: Vec<&str>) -> Option<Self> {
        let mut schematic = Vec::new();

        for input_line in &input {
            schematic.push(Vec::new());
            let last_schematic_input = schematic.last_mut().unwrap();
            for character in input_line.chars() {
                match Part::try_from(character) {
                    Ok(part) => last_schematic_input.push(part),
                    Err(part_error) => {
                        println!("Cannot build engine schematic, {}", part_error);
                        return None;
                    }
                }
            }
        }

        Some(Engine { schematic })
    }

    pub fn get_valid_part_number_values(&self) -> Vec<u32> {
        let symbol_positions = self.get_symbol_positions();

        let mut adjacent_part_numbers = Vec::new();

        for symbol_position in symbol_positions {
            let current_adjacent_part_numbers = self.get_adjacent_part_numbers(symbol_position);

            for part_number in current_adjacent_part_numbers {
                if !adjacent_part_numbers.contains(&part_number) {
                    adjacent_part_numbers.push(part_number);
                }
            }
        }

        adjacent_part_numbers
            .into_iter()
            .map(|part_number| part_number.value)
            .collect()
    }

    fn get_symbol_positions(&self) -> Vec<(usize, usize)> {
        let mut symbol_positions = Vec::new();

        for (row_index, row) in self.schematic.iter().enumerate() {
            for (column_index, part) in row.iter().enumerate() {
                if let Part::Symbol(_) = part {
                    symbol_positions.push((row_index, column_index));
                }
            }
        }

        symbol_positions
    }

    fn get_adjacent_part_numbers(&self, position: (usize, usize)) -> Vec<PartNumber> {
        let neighbour_positions = self.get_neighbour_positions(position);

        let mut adjacent_part_numbers = Vec::new();

        for (row_index, column_index) in neighbour_positions {
            if let Some(Part::Digit(_)) = self.schematic.get(row_index).unwrap().get(column_index) {
                let part_number = self.get_part_number(row_index, column_index);

                if !adjacent_part_numbers.contains(&part_number) {
                    adjacent_part_numbers.push(part_number);
                }
            }
        }

        adjacent_part_numbers
    }

    fn get_neighbour_positions(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let possible_neighbour_positions = self.get_all_possible_neighbour_positions(position);

        let mut neighbour_positions = Vec::new();

        for neighbour_position in possible_neighbour_positions {
            if self.is_inside_schematic(neighbour_position) {
                neighbour_positions
                    .push((neighbour_position.0 as usize, neighbour_position.1 as usize));
            }
        }

        neighbour_positions
    }

    fn get_all_possible_neighbour_positions(&self, position: (usize, usize)) -> Vec<(i32, i32)> {
        let mut possible_neighbour_positions = Vec::new();

        for row_index in [
            position.0 as i32 - 1,
            position.0 as i32,
            position.0 as i32 + 1,
        ] {
            for column_index in [
                position.1 as i32 - 1,
                position.1 as i32,
                position.1 as i32 + 1,
            ] {
                if !self.is_same_position(position, (row_index, column_index)) {
                    possible_neighbour_positions.push((row_index, column_index));
                }
            }
        }

        possible_neighbour_positions
    }

    fn is_inside_schematic(&self, position: (i32, i32)) -> bool {
        let max_row_index = self.schematic.len() - 1;
        let max_column_index = self.schematic.first().unwrap().len() - 1;

        if (position.0 >= 0 && position.0 <= max_column_index as i32)
            && (position.1 >= 0 && position.1 <= max_row_index as i32)
        {
            true
        } else {
            false
        }
    }

    fn is_same_position(&self, position_usize: (usize, usize), position_i32: (i32, i32)) -> bool {
        position_usize.0 as i32 == position_i32.0 && position_usize.1 as i32 == position_i32.1
    }

    fn get_part_number(&self, row_index: usize, start_column_index: usize) -> PartNumber {
        let (part_number_value, part_number_value_digit_indices) =
            self.get_part_number_value_and_digit_indices(row_index, start_column_index);

        PartNumber {
            value: part_number_value,
            row_index: row_index,
            column_indices: part_number_value_digit_indices,
        }
    }

    fn get_part_number_value_and_digit_indices(
        &self,
        row_index: usize,
        start_column_index: usize,
    ) -> (u32, Vec<usize>) {
        let (first_part_number_value_digits, first_part_number_value_digit_indices) =
            self.get_first_part_number_digits_and_indices(row_index, start_column_index);
        let (last_part_number_value_digits, last_part_number_value_digit_indices) =
            self.get_last_part_number_digits_and_indices(row_index, start_column_index);

        let part_number_value_digits = vec![
            first_part_number_value_digits,
            last_part_number_value_digits,
        ]
        .concat();
        let part_number_value_digit_indices = vec![
            first_part_number_value_digit_indices,
            last_part_number_value_digit_indices,
        ]
        .concat();

        let part_number_value = self.get_part_number_value(part_number_value_digits);

        (part_number_value, part_number_value_digit_indices)
    }

    fn get_first_part_number_digits_and_indices(
        &self,
        row_index: usize,
        start_column_index: usize,
    ) -> (Vec<u32>, Vec<usize>) {
        let row = self.schematic.get(row_index).unwrap();
        let mut first_part_number_value_digits = Vec::new();
        let mut first_part_number_value_digit_indices = Vec::new();

        let mut first_digit_column_index = start_column_index;
        while let Some(Part::Digit(n)) = row.get(first_digit_column_index) {
            first_part_number_value_digits.push(*n);
            first_part_number_value_digit_indices.push(first_digit_column_index);

            if first_digit_column_index == 0 {
                break;
            }

            first_digit_column_index -= 1;
        }

        // reverse in order to have right sorting of digits
        first_part_number_value_digits.reverse();
        first_part_number_value_digit_indices.reverse();

        (
            first_part_number_value_digits,
            first_part_number_value_digit_indices,
        )
    }

    fn get_last_part_number_digits_and_indices(
        &self,
        row_index: usize,
        start_column_index: usize,
    ) -> (Vec<u32>, Vec<usize>) {
        let row = self.schematic.get(row_index).unwrap();
        let mut last_part_number_value_digits = Vec::new();
        let mut last_part_number_value_digit_indices = Vec::new();

        let mut last_digit_column_index = start_column_index + 1;
        while let Some(Part::Digit(n)) = row.get(last_digit_column_index) {
            last_part_number_value_digits.push(*n);
            last_part_number_value_digit_indices.push(last_digit_column_index);

            if last_digit_column_index == row.len() - 1 {
                break;
            }

            last_digit_column_index += 1;
        }

        (
            last_part_number_value_digits,
            last_part_number_value_digit_indices,
        )
    }

    fn get_part_number_value(&self, part_number_value_digits: Vec<u32>) -> u32 {
        // calculate part number value from digits
        let mut part_number_value = 0;
        for (index, part_number_digit) in part_number_value_digits.into_iter().rev().enumerate() {
            part_number_value += part_number_digit * (u32::pow(10, index as u32));
        }

        part_number_value
    }

    pub fn get_gear_ratios(&self) -> Vec<u32> {
        let gear_positions = self.get_gear_positions();

        let mut gear_ratios: Vec<u32> = Vec::new();

        for gear_position in gear_positions {
            let adjacent_part_numbers = self.get_adjacent_part_numbers(gear_position);

            if adjacent_part_numbers.len() == 2 {
                gear_ratios.push(
                    adjacent_part_numbers.first().unwrap().value
                        * adjacent_part_numbers.last().unwrap().value,
                );
            }
        }

        gear_ratios
    }

    fn get_gear_positions(&self) -> Vec<(usize, usize)> {
        let mut gear_positions = Vec::new();

        for (row_index, row) in self.schematic.iter().enumerate() {
            for (column_index, part) in row.iter().enumerate() {
                if let Part::Symbol('*') = part {
                    gear_positions.push((row_index, column_index));
                }
            }
        }

        gear_positions
    }
}
