use crate::state::command::Command;

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    commands: Vec<Command>,
}

impl Solution {
    pub fn push(&mut self, command: Command) {
        self.commands.push(command)
    }

    pub fn reverse(&mut self) {
        self.commands.reverse()
    }
}

impl From<Vec<Command>> for Solution {
    fn from(commands: Vec<Command>) -> Self {
        Self { commands }
    }
}
