use crate::state::direction::{Compass, Moveable};
use std::cmp::{Ord, Ordering};
use std::ops::{Add, Sub};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Position<T>(pub T, pub T)
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable;

impl<T> From<(T, T)> for Position<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable,
{
    fn from((x, y): (T, T)) -> Self {
        Self(x, y)
    }
}

impl<T> Ord for Position<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable + Add<Output = T>,
{
    fn cmp(&self, other: &Self) -> Ordering {
        let left = self.0.absolute() + self.1.absolute();
        let right = other.0.absolute() + other.1.absolute();
        left.cmp(&right)
    }
}

impl<T> PartialOrd for Position<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable + Add<Output = T>,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Sub<Position<T>> for Position<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> Moveable for Position<T>
where
    T: PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Copy
        + Clone
        + Absolutable
        + Incrementable
        + Decrementable,
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

pub trait Absolutable {
    fn absolute(&self) -> Self;
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

impl Absolutable for i64 {
    fn absolute(&self) -> Self {
        self.abs()
    }
}
