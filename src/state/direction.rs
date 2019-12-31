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

    pub fn before(&self, (x, y): &(usize, usize)) -> Option<(usize, usize)> {
        match *self {
            Compass::North => { if *y > 0 { Some((*x, *y - 1))} else { None }},
            Compass::East => { if *x > 0 { Some((*x - 1, *y))} else { None }},
            Compass::South => { Some((*x, *y + 1)) },
            Compass::West => { Some((*x + 1, *y)) },
           }
    }
}

pub trait Reachable<T> {
    fn reaches(&self, target: &T) -> bool;
}

impl Reachable<(usize, usize)> for (&(usize, usize), &Compass) {
    fn reaches(&self, target: &(usize, usize)) -> bool {
        match *self {
            ((x,y), Compass::North) => { target.0 == *x && target.1 > *y },
            ((x,y), Compass::East)  => { target.0 > *x && target.1 == *y },
            ((x,y), Compass::South) => { target.0 == *x && target.1 < *y },
            ((x,y), Compass::West)  => { target.0 < *x && target.1 == *y }
        }
    }
}