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

#[derive(Debug, PartialEq)]
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

        // TODO remove
        println!("Part numbers: {:?}", part_numbers);

        part_numbers
    }

    fn get_part_number(&self, row_index: usize, column_index: usize) -> PartNumber {
        let row = self.schematic.get(row_index).unwrap();
        let mut part_number_value_digits = Vec::new();
        let mut value_digits_index = Vec::new();

        // TODO improve search for first column index
        // get part number value digits from beginning to column index
        let mut first_digit_column_index = column_index;
        while let Some(Part::Digit(n)) = row.get(first_digit_column_index) {
            part_number_value_digits.push(n);
            value_digits_index.push(first_digit_column_index);
            if first_digit_column_index == 0 {
                break;
            }
            first_digit_column_index -= 1;
        }
        // reverse in order to have right sorting of digits
        part_number_value_digits.reverse();
        value_digits_index.reverse();

        // TODO improve search for last column index
        // get part number value digits from column index to end
        let mut last_digit_column_index = column_index + 1;
        while let Some(Part::Digit(n)) = row.get(last_digit_column_index) {
            part_number_value_digits.push(n);
            value_digits_index.push(last_digit_column_index);
            if last_digit_column_index == row.len() - 1 {
                break;
            }
            last_digit_column_index += 1;
        }

        // calculate part number value from digits
        let mut part_number_value = 0;
        for (index, part_number_digit) in part_number_value_digits.into_iter().rev().enumerate() {
            part_number_value += part_number_digit * (u32::pow(10, index as u32));
        }

        PartNumber {
            value: part_number_value,
            row_index: row_index,
            column_indices: value_digits_index,
        }
    }

    fn is_valid_part_number(&self, part_number: &PartNumber) -> bool {
        todo!("If any of the neighbours of any digit is a symbol, the part number is valid")
    }
}
