use crate::state::{direction::Compass, robot::Robot};

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Command {
    pub robot: Robot,
    pub direction: Compass,
}

impl From<(Robot, Compass)> for Command {
    fn from((robot, direction): (Robot, Compass)) -> Self {
        Self { robot, direction }
    }
}
