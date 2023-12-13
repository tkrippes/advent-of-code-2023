pub fn part_1(file_name: &str) -> u32 {
    todo!()
}

pub fn part_2(file_name: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let result = part_1("../input/test_input.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_input_2_part_1() {
        let result = part_1("../input/test_input_2.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_input_part_2() {
        let result = part_2("../input/test_input.txt");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_input_2_part_2() {
        let result = part_2("../input/test_input_2.txt");
        assert_eq!(result, 0);
    }
}
