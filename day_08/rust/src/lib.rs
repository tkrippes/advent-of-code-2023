use command::Commands;
use network::Network;
use std::fs;

mod command;
mod network;

pub fn part_1(file_name: &str) -> u32 {
    match try_get_commands_and_network(file_name) {
        Some((commands, network)) => todo!(),
        None => {
            println!("Failed to get commands or network");
            0
        }
    }
}

pub fn part_2(file_name: &str) -> u32 {
    todo!()
}

fn try_get_commands_and_network(file_name: &str) -> Option<(Commands, Network)> {
    let file_content =
        fs::read_to_string(file_name).expect("input file should be located in input folder");
    let file_lines: Vec<&str> = file_content
        .split('\n')
        .map(|file_line| file_line.trim())
        .collect();

    let commands_input = file_lines[0];
    let network_input = file_lines[2..].to_vec();

    if let (Some(commands), Some(network)) = (
        Commands::try_build(commands_input),
        Network::try_build(network_input),
    ) {
        Some((commands, network))
    } else {
        println!("Cannot get either command or network");
        None
    }
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
