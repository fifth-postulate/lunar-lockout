pub mod command;
pub mod direction;
pub mod robot;
pub mod configuration;

#[cfg(test)]
mod tests {
    use crate::state::{robot::Robot, configuration::Configuration, command::Command, direction::Compass};

    #[test]
    fn a_valid_command_should_move_a_robot_next_to_an_other() {
        let configuration = Configuration::from((5, 5, vec![(Robot::Red, 0, 2), (Robot::Yellow, 3, 2)]));
        let command = Command::from((Robot::Red, Compass::East));

        let actual = configuration.perform(&command).unwrap();

        assert_eq!(actual, Configuration::from((5, 5, vec![(Robot::Red, 2, 2), (Robot::Yellow, 3,2)])));
    }
}