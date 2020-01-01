pub mod ids;

use crate::state::{
    command::Command,
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
    fn reached_in(&self, configuration: &Configuration<T>) -> bool {
        configuration.robot_at(&self.0, &self.1)
    }
}

impl<T> From<(Robot, Position<T>)> for Target<T> where 
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

pub trait Solver {
    fn solve<T>(
        &self,
        configuration: &Configuration<T>,
        target: &Target<T>,
    ) -> Option<Vec<Command>>
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
            + Sub<Output = T>;
}
