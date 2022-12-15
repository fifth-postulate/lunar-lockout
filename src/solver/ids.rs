use crate::{
    solver::{Solution, Solver, Target},
    state::{
        configuration::Configuration,
        position::{Absolutable, Decrementable, Incrementable},
    },
};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::ops::{Add, Sub};

pub struct IterativeDeepening {
    maximum_depth: Depth,
}

impl Default for IterativeDeepening {
    fn default() -> Self {
        Self::new()
    }
}

impl IterativeDeepening {
    pub fn new() -> Self {
        Self::from(Depth::Infinite)
    }

    fn solve_to_depth<T>(
        &self,
        depth: Depth,
        configuration: &Configuration<T>,
        target: &Target<T>,
    ) -> Option<Solution>
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
        if depth == Depth::Finite(0) {
            if target.reached_in(configuration) {
                Some(Solution::from(vec![]))
            } else {
                None
            }
        } else {
            for command in configuration.options() {
                let next = configuration
                    .perform(&command)
                    .expect("command to be an option");
                if let Some(mut solution) =
                    self.solve_to_depth(depth.decrement().unwrap(), &next, target)
                {
                    solution.push(command);
                    return Some(solution);
                }
            }
            None
        }
    }
}

impl Solver for IterativeDeepening {
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
            + Sub<Output = T>,
    {
        let mut current_depth = Depth::Finite(0);
        while current_depth < self.maximum_depth {
            if let Some(mut solution) = self.solve_to_depth(current_depth, configuration, target) {
                solution.reverse();
                return Some(solution);
            }
            current_depth = current_depth.increment();
        }
        None
    }
}

impl From<usize> for IterativeDeepening {
    fn from(depth: usize) -> Self {
        Self::from(Depth::Finite(depth))
    }
}

impl From<Depth> for IterativeDeepening {
    fn from(maximum_depth: Depth) -> Self {
        Self { maximum_depth }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Depth {
    Finite(usize),
    Infinite,
}

impl Depth {
    pub fn increment(&self) -> Self {
        match self {
            Depth::Finite(depth) => Depth::Finite(depth + 1),

            Depth::Infinite => Depth::Infinite,
        }
    }

    pub fn decrement(&self) -> Option<Self> {
        match self {
            Depth::Finite(depth) => {
                if *depth > 0 {
                    Some(Depth::Finite(depth - 1))
                } else {
                    None
                }
            }
            Depth::Infinite => Some(Depth::Infinite),
        }
    }
}

impl PartialOrd for Depth {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Depth {
    fn cmp(&self, other: &Self) -> Ordering {
        match (*self, *other) {
            (Depth::Infinite, Depth::Infinite) => Ordering::Equal,
            (Depth::Infinite, _) => Ordering::Greater,
            (_, Depth::Infinite) => Ordering::Less,
            (Depth::Finite(left), Depth::Finite(right)) => left.cmp(&right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        solver::Target,
        state::{command::Command, direction::Compass, position::Position, robot::Robot},
    };

    #[test]
    fn ordering_of_depth() {
        assert!(Depth::Finite(0) < Depth::Finite(1));
        assert!(Depth::Finite(0) < Depth::Infinite);
    }

    #[test]
    fn should_solve_puzzle() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((0, 0))),
            (Robot::Yellow, Position::from((3, 0))),
            (Robot::Orange, Position::from((2, 3))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::new();

        let solution = solver.solve(&configuration, &target);

        assert_eq!(
            solution,
            Some(Solution::from(vec![
                Command::from((Robot::Red, Compass::East)),
                Command::from((Robot::Red, Compass::North))
            ]))
        )
    }
}
