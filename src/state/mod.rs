pub mod command;
pub mod configuration;
pub mod direction;
pub mod position;
pub mod robot;

#[cfg(test)]
mod tests {
    use crate::state::{
        command::Command,
        configuration::{Configuration, Error},
        direction::Compass,
        position::Position,
        robot::Robot,
    };

    #[test]
    fn a_valid_command_should_move_a_robot_next_to_an_other() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((0, 2))),
            (Robot::Yellow, Position::from((3, 2))),
        ]);
        let command = Command::from((Robot::Red, Compass::East));

        let actual = configuration.perform(&command).expect("a configuration");

        assert_eq!(
            actual,
            Configuration::from(vec![
                (Robot::Red, Position::from((2, 2))),
                (Robot::Yellow, Position::from((3, 2))),
            ])
        );
    }

    #[test]
    fn a_command_is_invalid_if_the_robot_is_not_in_the_configuration() {
        let configuration = Configuration::from(vec![(Robot::Yellow, Position::from((3, 2)))]);
        let command = Command::from((Robot::Red, Compass::East));

        let actual = configuration.perform(&command);

        assert_eq!(actual, Err(Error::RobotNotInConfiguration));
    }

    #[test]
    fn the_nearest_robot_stops_the_command() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((5, 2))),
            (Robot::Yellow, Position::from((3, 2))),
            (Robot::Orange, Position::from((0, 2))),
        ]);
        let command = Command::from((Robot::Red, Compass::West));

        let actual = configuration.perform(&command).expect("a configuration");

        assert_eq!(
            actual,
            Configuration::from(vec![
                (Robot::Red, Position::from((4, 2))),
                (Robot::Yellow, Position::from((3, 2))),
                (Robot::Orange, Position::from((0, 2)))
            ])
        );
    }

    #[test]
    fn a_command_is_invalid_if_the_robot_is_not_stopped_by_an_other() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((0, 2))),
            (Robot::Yellow, Position::from((3, 2))),
        ]);
        let command = Command::from((Robot::Red, Compass::North));

        let actual = configuration.perform(&command);

        assert_eq!(actual, Err(Error::IllegalMove));
    }
}
