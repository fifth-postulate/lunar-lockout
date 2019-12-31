use super::direction::Compass;
use super::robot::Robot;

pub struct Command {
    robot: Robot,
    direction: Compass,
}
