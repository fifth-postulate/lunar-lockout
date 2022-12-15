use std::fmt::{Display, Formatter, Result};

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

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut current = None;
        for command in &self.commands {
            if let Some(current_robot) = current {
                if current_robot != command.robot {
                    current = Some(command.robot);
                    write!(f, " {}{}", command.robot, command.direction)?
                } else {
                    // current_robot == command_robot
                    write!(f, "{}", command.direction)?
                }
            } else {
                current = Some(command.robot);
                write!(f, "{}{}", command.robot, command.direction)?
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{direction::Compass, robot::Robot};
    use std::fmt::{Result, Write};

    #[test]
    fn a_solution_can_be_displayed() -> Result {
        let solution = Solution::from(vec![
            Command::from((Robot::Red, Compass::North)),
            Command::from((Robot::Red, Compass::East)),
            Command::from((Robot::Yellow, Compass::West)),
            Command::from((Robot::Purple, Compass::South)),
        ]);

        let mut actual = String::new();
        write!(actual, "{}", solution)?;

        assert_eq!(actual, "R↑→ Y← P↓");

        Ok(())
    }
}
