use crate::state::position::{Absolutable, Position};

pub enum Compass {
    North,
    East,
    South,
    West,
}

impl Compass {
    pub fn opposite(&self) -> Self {
        match *self {
            Compass::North => Compass::South,
            Compass::East => Compass::West,
            Compass::South => Compass::North,
            Compass::West => Compass::East,
        }
    }
}

pub trait Reachable<T> {
    fn reaches(&self, target: &T) -> bool;
}

impl<T> Reachable<Position<T>> for (&Position<T>, &Compass)
where
    T: PartialEq + Eq + PartialOrd + Ord + Absolutable,
{
    fn reaches(&self, target: &Position<T>) -> bool {
        match *self {
            (position, Compass::North) => target.0 == position.0 && target.1 > position.1,
            (position, Compass::East) => target.0 > position.0 && target.1 == position.1,
            (position, Compass::South) => target.0 == position.0 && target.1 < position.1,
            (position, Compass::West) => target.0 < position.0 && target.1 == position.1,
        }
    }
}

pub trait Moveable {
    fn move_to(&self, direction: &Compass) -> Self;
}
