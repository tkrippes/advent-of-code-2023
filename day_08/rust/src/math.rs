pub fn least_common_multiple(numbers: &[u64]) -> u64 {
    match numbers.len() {
        0 => 0,
        1 => *numbers.first().unwrap(),
        2 => {
            let first_number = *numbers.first().unwrap();
            let second_number = *numbers.last().unwrap();
            (first_number * second_number) / greatest_common_divisor(first_number, second_number)
        }
        _ => least_common_multiple(&[
            least_common_multiple(&numbers[..numbers.len() / 2]),
            least_common_multiple(&numbers[numbers.len() / 2..]),
        ]),
    }
}

pub fn greatest_common_divisor(first_number: u64, second_number: u64) -> u64 {
    let mut first_number = first_number;
    let mut second_number = second_number;
    let mut temp_number;

    while second_number != 0 {
        temp_number = second_number;
        second_number = first_number % second_number;
        first_number = temp_number;
    }

    first_number
}
