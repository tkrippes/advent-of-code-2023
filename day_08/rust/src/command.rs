#[derive(Clone)]
pub enum Command {
    Left,
    Right,
}

impl TryFrom<char> for Command {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Command::Left),
            'R' => Ok(Command::Right),
            _ => Err(format!("Cannot parse command from {}", value)),
        }
    }
}

pub struct Commands {
    commands: Vec<Command>,
}

impl Commands {
    pub fn try_build(input: &str) -> Option<Self> {
        let mut commands = Vec::new();

        for character in input.trim().chars() {
            match Command::try_from(character) {
                Ok(command) => commands.push(command),
                Err(command_parsing_error) => {
                    println!("Cannot parse commands, {}", command_parsing_error);
                    return None;
                }
            }
        }

        Some(Commands { commands })
    }

    pub fn get_commands(&self) -> Vec<Command> {
        self.commands.clone()
    }
}
