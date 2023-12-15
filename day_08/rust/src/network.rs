use std::collections::HashMap;

use super::command;

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
    ) -> Option<u64> {
        if self.connections.contains_key(start_node) && self.connections.contains_key(end_node) {
            let commands = commands.get_commands();
            let mut current_node = start_node;
            let mut number_of_steps = 0;

            while current_node != end_node {
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

            Some(number_of_steps)
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
    ) -> Option<u64> {
        let mut current_nodes = self.get_nodes_ending_in(start_nodes_ending_character);

        if !current_nodes.is_empty() {
            let initial_nodes_size = current_nodes.len();
            let commands = commands.get_commands();
            let mut number_of_steps = 0;

            while !Self::do_all_nodes_end_in(&current_nodes, end_nodes_ending_character) {
                for command in &commands {
                    match command {
                        command::Command::Left => {
                            current_nodes = current_nodes
                                .iter()
                                .filter_map(|node| self.connections.get(*node))
                                .map(|node_connection| node_connection.left_connection.as_str())
                                .collect()
                        }
                        command::Command::Right => {
                            current_nodes = current_nodes
                                .iter()
                                .filter_map(|node| self.connections.get(*node))
                                .map(|node_connection| node_connection.right_connection.as_str())
                                .collect()
                        }
                    };

                    if current_nodes.len() == initial_nodes_size {
                        number_of_steps += 1;
                    } else {
                        println!("Cannot get number of steps, something went wrong!");
                        return None;
                    }
                }
            }

            Some(number_of_steps)
        } else {
            println!(
                "Cannot get number of steps, no nodes end with {}",
                start_nodes_ending_character
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

    fn do_all_nodes_end_in(nodes: &[&str], character: char) -> bool {
        nodes.iter().all(|node| node.ends_with(character))
    }
}
