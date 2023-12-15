use command::Commands;
use network::Network;
use std::fs;

mod command;
mod math;
mod network;

pub fn part_1(file_name: &str) -> u64 {
    match try_get_commands_and_network(file_name) {
        Some((commands, network)) => {
            get_number_of_steps_single_start_node(&commands, &network, "AAA", "ZZZ")
        }
        None => {
            println!("Failed to get commands or network");
            0
        }
    }
}

pub fn part_2(file_name: &str) -> u64 {
    match try_get_commands_and_network(file_name) {
        Some((commands, network)) => {
            get_number_of_steps_multiple_start_nodes(&commands, &network, 'A', 'Z')
        }
        None => {
            println!("Failed to get commands or network");
            0
        }
    }
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

fn get_number_of_steps_single_start_node(
    commands: &Commands,
    network: &Network,
    start_node: &str,
    end_node: &str,
) -> u64 {
    match network.get_number_of_steps_single_start_node(commands, start_node, end_node, u64::MAX) {
        Some(number_of_steps) => number_of_steps,
        None => {
            println!(
                "Unable to get number of steps from {} to {}",
                start_node, end_node
            );
            0
        }
    }
}

fn get_number_of_steps_multiple_start_nodes(
    commands: &Commands,
    network: &Network,
    start_nodes_ending_character: char,
    end_nodes_ending_character: char,
) -> u64 {
    match network.get_number_of_steps_multiple_start_nodes(
        commands,
        start_nodes_ending_character,
        end_nodes_ending_character,
    ) {
        Some(number_of_steps) => number_of_steps,
        None => {
            println!(
                "Unable to get number of steps from all nodes ending with {} to all nodes ending with {}",
                start_nodes_ending_character, end_nodes_ending_character
            );
            0
        }
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
    fn test_input_3_part_2() {
        let result = part_2("../input/test_input_3.txt");
        assert_eq!(result, 6);
    }
}
