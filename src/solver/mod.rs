use crate::state::{
    command::Command,
    configuration::Configuration,
    position::{Absolutable, Position},
    robot::Robot,
};

pub struct Target<T>(Robot, Position<T>)
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable;

pub trait Solver<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable,
{
    fn solve(configuration: Configuration<T>, target: Target<T>) -> Vec<Command>;
}
