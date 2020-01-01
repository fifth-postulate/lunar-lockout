use crate::state::direction::{Compass, Moveable};
use std::ops::Sub;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Clone, Debug)]
pub struct Position<T>(pub T, pub T)
where
    T: PartialEq + Eq + PartialOrd + Ord;

impl<T> From<(T, T)> for Position<T>
where
    T: PartialEq + Eq + PartialOrd + Ord,
{
    fn from((x, y): (T, T)) -> Self {
        Self(x, y)
    }
}

impl<T> Sub<Position<T>> for Position<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> Moveable for Position<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Copy + Clone + Incrementable + Decrementable,
{
    fn move_to(&self, direction: &Compass) -> Self {
        match direction {
            Compass::North => Position(self.0, self.1.increment()),
            Compass::East => Position(self.0.increment(), self.1),
            Compass::South => Position(self.0, self.1.decrement()),
            Compass::West => Position(self.0.decrement(), self.1),
        }
    }
}

pub trait Incrementable {
    fn increment(&self) -> Self;
}

pub trait Decrementable {
    fn decrement(&self) -> Self;
}

impl Incrementable for i64 {
    fn increment(&self) -> Self {
        self + 1i64
    }
}

impl Decrementable for i64 {
    fn decrement(&self) -> Self {
        self - 1i64
    }
}
