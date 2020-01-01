use crate::state::{
    command::Command,
    direction::{Moveable, Reachable},
    position::{Decrementable, Incrementable, Position},
    robot::Robot,
};
use std::collections::HashMap;
use std::ops::Sub;

#[derive(PartialEq, Debug)]
pub struct Configuration<T>
where
    T: PartialEq + Eq + PartialOrd + Ord,
{
    robots: HashMap<Robot, Position<T>>,
}

impl<T> From<Vec<(Robot, Position<T>)>> for Configuration<T>
where
    T: PartialEq + Eq + PartialOrd + Ord,
{
    fn from(locations: Vec<(Robot, Position<T>)>) -> Self {
        let mut robots = HashMap::new();
        for (robot, position) in locations {
            robots.insert(robot, position);
        }

        Self { robots }
    }
}

impl<T> Configuration<T>
where
    T: PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Copy
        + Clone
        + Sub<Output = T>
        + Incrementable
        + Decrementable,
{
    pub fn perform(&self, command: &Command) -> Result<Self, Error> {
        if let Some(start) = self.robots.get(&command.robot) {
            let mut candidates = self
                .robots
                .iter()
                .filter(|(robot, _)| **robot != command.robot)
                .filter(|(_, post)| (start, &command.direction).reaches(post))
                .map(|(_, post)| post)
                .map(|post| post.move_to(&command.direction.opposite()))
                .collect::<Vec<Position<T>>>();
            candidates.sort_by(|left, right| (*left - *start).cmp(&(*right - *start)));
            if let Some(stop) = candidates.iter().next() {
                let mut robots: HashMap<Robot, Position<T>> = self.robots.clone();
                robots.insert(command.robot, *stop);
                Ok(Self { robots })
            } else {
                Err(Error::IllegalMove)
            }
        } else {
            Err(Error::RobotNotInConfiguration)
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Error {
    RobotNotInConfiguration,
    IllegalMove,
}
