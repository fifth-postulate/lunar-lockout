use lunarlockout::{
    solver::{
        ids::{Depth, IterativeDeepening},
        Solver, Target,
    },
    state::{configuration::Configuration, position::Position, robot::Robot},
};

fn main() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((3, 1))),
        (Robot::Yellow, Position::from((1, 0))),
        (Robot::Orange, Position::from((1, 4))),
        (Robot::Green, Position::from((4, 3))),
        (Robot::Purple, Position::from((0, 2))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(9));

    let solution = solver.solve(&configuration, &target);

    println!("{:?}", solution);
}
