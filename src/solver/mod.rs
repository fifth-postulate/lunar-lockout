pub mod ids;
mod solution;
mod target;

pub use solution::Solution;
pub use target::Target;

use crate::state::{
    configuration::Configuration,
    position::{Absolutable, Decrementable, Incrementable},
};
use std::ops::{Add, Sub};

pub trait Solver {
    fn solve<T>(&self, configuration: &Configuration<T>, target: &Target<T>) -> Option<Solution>
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
