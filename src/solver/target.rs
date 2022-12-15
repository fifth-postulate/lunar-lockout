use crate::state::{
    configuration::Configuration,
    position::{Absolutable, Decrementable, Incrementable, Position},
    robot::Robot,
};
use std::ops::{Add, Sub};

pub struct Target<T>(Robot, Position<T>)
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable;

impl<T> Target<T>
where
    T: PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Clone
        + Copy
        + Absolutable
        + Incrementable
        + Decrementable
        + Add<Output = T>
        + Sub<Output = T>,
{
    pub fn reached_in(&self, configuration: &Configuration<T>) -> bool {
        configuration.robot_at(&self.0, &self.1)
    }
}

impl<T> From<(Robot, Position<T>)> for Target<T>
where
    T: PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Clone
        + Copy
        + Absolutable
        + Incrementable
        + Decrementable
        + Add<Output = T>
        + Sub<Output = T>,
{
    fn from((robot, position): (Robot, Position<T>)) -> Self {
        Self(robot, position)
    }
}
