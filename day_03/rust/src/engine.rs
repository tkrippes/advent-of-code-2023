enum Part {
    Empty,
    Digit(u32),
    Symbol(char),
    Gear,
}

impl TryFrom<char> for Part {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Part::Empty),
            n @ '0'..='9' => Ok(Part::Digit(n.to_digit(10).unwrap())),
            '*' => Ok(Part::Gear),
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
        let part_numbers = self.get_part_numbers();

        part_numbers
            .into_iter()
            .filter(|part_number| self.is_valid_part_number(part_number))
            .map(|part_number| part_number.value)
            .collect()
    }

    fn get_part_numbers(&self) -> Vec<PartNumber> {
        let mut part_numbers = Vec::new();

        for (row_index, line) in self.schematic.iter().enumerate() {
            for (column_index, part) in line.iter().enumerate() {
                if let Part::Digit(_) = part {
                    let part_number = self.get_part_number(row_index, column_index);

                    if !part_numbers.contains(&part_number) {
                        part_numbers.push(part_number);
                    }
                }
            }
        }

        part_numbers
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

        // TODO improve search
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

    fn is_valid_part_number(&self, part_number: &PartNumber) -> bool {
        self.has_adjacent_symbol(part_number)
    }

    fn has_adjacent_symbol(&self, part_number: &PartNumber) -> bool {
        let neighbour_positions = self.get_neighbour_positions(part_number);

        for (neighbour_row, neighbour_column) in neighbour_positions {
            match self
                .schematic
                .get(neighbour_row)
                .unwrap()
                .get(neighbour_column)
                .unwrap()
            {
                Part::Symbol(_) | Part::Gear => return true,
                _ => continue,
            }
        }

        false
    }

    fn get_neighbour_positions(&self, part_number: &PartNumber) -> Vec<(usize, usize)> {
        let all_possible_neighbour_positions =
            self.get_all_possible_neighbour_positions(part_number);

        let neighbour_positions =
            self.get_valid_neighbour_positions(part_number, all_possible_neighbour_positions);

        neighbour_positions
    }

    fn get_all_possible_neighbour_positions(&self, part_number: &PartNumber) -> Vec<(i32, i32)> {
        let possible_row_indices = vec![
            part_number.row_index as i32 - 1,
            part_number.row_index as i32,
            part_number.row_index as i32 + 1,
        ];

        let mut possible_column_indices =
            vec![*part_number.column_indices.first().unwrap() as i32 - 1];
        possible_column_indices = vec![
            possible_column_indices,
            part_number
                .column_indices
                .iter()
                .map(|column_index| *column_index as i32)
                .collect::<Vec<i32>>(),
        ]
        .concat();
        possible_column_indices.push(*part_number.column_indices.last().unwrap() as i32 + 1);

        let mut all_possible_neighbour_positions = Vec::new();

        for row_index in &possible_row_indices {
            for column_index in &possible_column_indices {
                all_possible_neighbour_positions.push((*row_index, *column_index))
            }
        }

        all_possible_neighbour_positions
    }

    fn get_valid_neighbour_positions(
        &self,
        part_number: &PartNumber,
        all_possible_neighbour_positions: Vec<(i32, i32)>,
    ) -> Vec<(usize, usize)> {
        let mut valid_neighbour_positions = Vec::new();

        for possible_neighbour_position in all_possible_neighbour_positions {
            if self.is_inside_schematic(possible_neighbour_position)
                && !self.is_in_part_number(part_number, possible_neighbour_position)
            {
                valid_neighbour_positions.push((
                    possible_neighbour_position.0 as usize,
                    possible_neighbour_position.1 as usize,
                ))
            }
        }

        valid_neighbour_positions
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

    fn is_in_part_number(&self, part_number: &PartNumber, position: (i32, i32)) -> bool {
        let mut part_number_positions = Vec::new();

        for column_index in &part_number.column_indices {
            part_number_positions.push((part_number.row_index as i32, *column_index as i32));
        }

        part_number_positions.contains(&position)
    }

    pub fn get_gear_ratios(&self) -> Vec<u32> {
        let part_numbers = self.get_part_numbers();

        part_numbers
            .into_iter()
            .filter(|part_number| self.is_valid_gear_part_number(part_number))
            .map(|part_number| part_number.value)
            .collect()
    }

    fn is_valid_gear_part_number(&self, part_number: &PartNumber) -> bool {
        self.has_exactly_two_adjacent_gears(part_number)
    }

    fn has_exactly_two_adjacent_gears(&self, part_number: &PartNumber) -> bool {
        let neighbour_positions = self.get_neighbour_positions(part_number);

        let mut number_of_adjacent_gears = 0;

        for (neighbour_row, neighbour_column) in neighbour_positions {
            if let Part::Gear = self
                .schematic
                .get(neighbour_row)
                .unwrap()
                .get(neighbour_column)
                .unwrap()
            {
                number_of_adjacent_gears += 1;
            }
        }

        number_of_adjacent_gears == 2
    }
}
