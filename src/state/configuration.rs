use std::collections::HashMap;
use crate::state::{robot::Robot, command::Command, direction::Reachable};

#[derive(PartialEq, Debug)]
pub struct Configuration {
    width: usize,
    height: usize,
    robots: HashMap<Robot, (usize, usize)>,
}

impl From<(usize, usize, Vec<(Robot, usize, usize)>)> for Configuration {
    fn from ((width, height, locations): (usize, usize, Vec<(Robot, usize, usize)>)) -> Self {
        let mut robots = HashMap::new();
        for (robot, x, y) in locations {
            robots.insert(robot, (x, y));
        }

        Self {width, height, robots}
    }
}

impl Configuration {
    pub fn perform(&self, command: &Command) -> Result<Self, Error> {
        if let Some(start) = self.robots.get(&command.robot) {
            let candidates = self.robots.iter()
                .filter(|(robot, _)| **robot != command.robot)
                .filter(|(_, post)| (start, &command.direction).reaches(post))
                .map(|(_, post)| post)
                .map(|post| command.direction.opposite().before(post).unwrap() )
                .collect::<Vec<(usize, usize)>>();

            Err(Error::IllegalMove)
        } else {
            Err(Error::RobotNotInConfiguration)
        }
    }
}

pub enum Error {
    RobotNotInConfiguration,
    IllegalMove,
}