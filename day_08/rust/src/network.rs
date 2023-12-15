use std::collections::HashMap;

use super::command;
use super::math;

type Node = String;

struct NodeConnections {
    left_connection: Node,
    right_connection: Node,
}

impl NodeConnections {
    fn build(left_connection: &str, right_connection: &str) -> Self {
        NodeConnections {
            left_connection: String::from(left_connection),
            right_connection: String::from(right_connection),
        }
    }
}

pub struct Network {
    connections: HashMap<Node, NodeConnections>,
}

impl Network {
    pub fn try_build(inputs: Vec<&str>) -> Option<Self> {
        let connection_regex = match Self::try_get_connections_regex() {
            Ok(connections_regex) => connections_regex,
            Err(connections_regex_parsing_error) => {
                println!(
                    "Cannot parse network, cannot get valid connection regex, {}",
                    connections_regex_parsing_error
                );
                return None;
            }
        };

        let mut connections = HashMap::new();

        let inputs = Self::tidy_entries(inputs);

        for input in inputs {
            match Self::try_get_connection(input, &connection_regex) {
                Some((source_node, node_connections)) => {
                    connections.insert(source_node, node_connections);
                }
                None => {
                    println!("Cannot parse network");
                    return None;
                }
            }
        }

        Some(Network { connections })
    }

    fn try_get_connections_regex() -> Result<regex::Regex, regex::Error> {
        regex::Regex::new(r"(\w+)+ = \((\w+)+, (\w+)+\)")
    }

    fn tidy_entries(entries: Vec<&str>) -> Vec<&str> {
        entries
            .into_iter()
            .filter(|entry| !entry.is_empty())
            .map(|entry| entry.trim())
            .collect()
    }

    fn try_get_connection(input: &str, regex: &regex::Regex) -> Option<(Node, NodeConnections)> {
        match regex.captures(input) {
            Some(captures) => {
                if let (Some(source_node), Some(left_connection), Some(right_connection)) =
                    (captures.get(1), captures.get(2), captures.get(3))
                {
                    Some((
                        String::from(source_node.as_str()),
                        NodeConnections::build(left_connection.as_str(), right_connection.as_str()),
                    ))
                } else {
                    println!("Cannot get connection, either source node, left or right connection missing in {}", input);
                    None
                }
            }
            None => {
                println!("Cannot get connection, no connection found in {}", input);
                None
            }
        }
    }

    pub fn get_number_of_steps_single_start_node(
        &self,
        commands: &command::Commands,
        start_node: &str,
        end_node: &str,
        max_number_of_iterations: u64,
    ) -> Option<u64> {
        if self.connections.contains_key(start_node) && self.connections.contains_key(end_node) {
            let commands = commands.get_commands();
            let mut current_node = start_node;
            let mut number_of_steps = 0;

            while current_node != end_node && number_of_steps < max_number_of_iterations {
                for command in &commands {
                    match self.connections.get(current_node) {
                        Some(connections) => {
                            number_of_steps += 1;

                            match command {
                                command::Command::Left => {
                                    current_node = &connections.left_connection
                                }
                                command::Command::Right => {
                                    current_node = &connections.right_connection
                                }
                            }
                        }
                        None => {
                            println!(
                                "Cannot get number of steps, node {} not found in connections",
                                current_node
                            );
                            return None;
                        }
                    }
                }
            }

            if number_of_steps < max_number_of_iterations {
                Some(number_of_steps)
            } else {
                println!(
                    "Cannot get number of steps, cannot find connection between {} and {} in under {} iterations",
                    start_node, end_node, max_number_of_iterations
                );
                None
            }
        } else {
            println!("Cannot get number of steps, either start or end note are not in network");
            None
        }
    }

    pub fn get_number_of_steps_multiple_start_nodes(
        &self,
        commands: &command::Commands,
        start_nodes_ending_character: char,
        end_nodes_ending_character: char,
        max_number_of_iterations: u64,
    ) -> Option<u64> {
        let start_nodes = self.get_nodes_ending_in(start_nodes_ending_character);
        let end_nodes = self.get_nodes_ending_in(end_nodes_ending_character);

        if !start_nodes.is_empty() && !end_nodes.is_empty() {
            let mut min_steps_to_end_nodes_list = Vec::new();

            for start_node in &start_nodes {
                let mut min_steps_to_end_nodes: Option<u64> = None;

                for end_node in &end_nodes {
                    if let Some(number_of_steps) = self.get_number_of_steps_single_start_node(
                        commands,
                        start_node,
                        end_node,
                        max_number_of_iterations,
                    ) {
                        if min_steps_to_end_nodes.is_none()
                            || number_of_steps < min_steps_to_end_nodes.unwrap()
                        {
                            min_steps_to_end_nodes = Some(number_of_steps);
                        }
                    }
                }

                match min_steps_to_end_nodes {
                    Some(min_steps_to_end_nodes) => {
                        min_steps_to_end_nodes_list.push(min_steps_to_end_nodes)
                    }
                    None => {
                        println!(
                            "Cannot get number of steps, no path found for starting node {}",
                            start_node
                        );
                        return None;
                    }
                }
            }

            Some(math::least_common_multiple(&min_steps_to_end_nodes_list))
        } else {
            println!(
                "Cannot get number of steps, no nodes end with {} or {}",
                start_nodes_ending_character, end_nodes_ending_character
            );
            None
        }
    }

    fn get_nodes_ending_in(&self, character: char) -> Vec<&str> {
        self.connections
            .keys()
            .filter(|node| node.ends_with(character))
            .map(|node| node.as_str())
            .collect()
    }
}
