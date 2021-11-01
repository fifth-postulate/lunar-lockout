use crate::state::{
    command::Command,
    direction::{Compass, Moveable, Reachable},
    position::{Absolutable, Decrementable, Incrementable, Position},
    robot::Robot,
};
use std::collections::HashMap;
use std::ops::{Add, Sub};

#[derive(PartialEq, Debug)]
pub struct Configuration<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable,
{
    robots: HashMap<Robot, Position<T>>,
}

impl<T> From<Vec<(Robot, Position<T>)>> for Configuration<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable,
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
        + Add<Output = T>
        + Absolutable
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
            candidates.sort_by_key(|left| (*left - *start));
            if let Some(stop) = candidates.get(0) {
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

    pub fn options(&self) -> Vec<Command> {
        let mut options: Vec<Command> = Vec::new();
        for (robot, start) in self.robots.iter() {
            for direction in vec![Compass::North, Compass::East, Compass::South, Compass::West] {
                let count = self
                    .robots
                    .iter()
                    .filter(|(r, _)| robot != *r)
                    .filter(|(_, post)| (start, &direction).reaches(post))
                    .count();
                if count > 0 {
                    options.push(Command::from((*robot, direction)));
                }
            }
        }
        options
    }

    pub fn robot_at(&self, robot: &Robot, position: &Position<T>) -> bool {
        self.robots.get(robot) == Some(position)
    }
}

#[derive(PartialEq, Debug)]
pub enum Error {
    RobotNotInConfiguration,
    IllegalMove,
}
