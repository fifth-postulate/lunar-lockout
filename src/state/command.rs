use super::robot::Robot;
use super::direction::Compass;

pub struct Command {
	robot: Robot,
	direction: Compass,
}