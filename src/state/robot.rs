use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Robot {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Display for Robot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let representation = match self {
            Robot::Red => "R",
            Robot::Orange => "O",
            Robot::Yellow => "Y",
            Robot::Green => "G",
            Robot::Blue => "B",
            Robot::Purple => "P",
        };
        write!(f, "{}", representation)
    }
}
