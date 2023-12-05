enum Part {
    Empty,
    Digit(u32),
    Symbol(char),
}

impl TryFrom<char> for Part {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Part::Empty),
            n @ '0'..='9' => Ok(Part::Digit(n.to_digit(10).unwrap())),
            c if c.is_ascii_punctuation() => Ok(Part::Symbol(c)),
            _ => Err(stringify!("Cannot parse Part from {}", value)),
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

    fn get_part_number(&self, row_index: usize, column_index: usize) -> PartNumber {
        todo!("Search for first and last index, get all digits, then create PartNumber from that information")
    }

    fn is_valid_part_number(&self, part_number: &PartNumber) -> bool {
        todo!("If any of the neighbours of any digit is a symbol, the part number is valid")
    }
}
