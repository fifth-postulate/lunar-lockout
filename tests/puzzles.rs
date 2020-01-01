extern crate lunarlockout;

use lunarlockout::{
    solver::{
        ids::{Depth, IterativeDeepening},
        Solver, Target,
    },
    state::{
        command::Command, configuration::Configuration, direction::Compass, position::Position,
        robot::Robot,
    },
};

#[test]
fn beginner_01() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((4, 0))),
        (Robot::Orange, Position::from((4, 4))),
        (Robot::Yellow, Position::from((3, 1))),
        (Robot::Green, Position::from((2, 3))),
        (Robot::Purple, Position::from((1, 2))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(5));

    let solution = solver.solve(&configuration, &target);

    assert_eq!(
        solution,
        Some(vec![
            Command::from((Robot::Red, Compass::North)),
            Command::from((Robot::Red, Compass::West)),
            Command::from((Robot::Red, Compass::South)),
            Command::from((Robot::Red, Compass::West)),
        ])
    )
}

#[test]
fn beginner_02() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((1, 0))),
        (Robot::Orange, Position::from((2, 4))),
        (Robot::Yellow, Position::from((3, 1))),
        (Robot::Green, Position::from((4, 3))),
        (Robot::Purple, Position::from((1, 2))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(7));

    let solution = solver.solve(&configuration, &target);

    assert_eq!(
        solution,
        Some(vec![
            Command::from((Robot::Red, Compass::North)),
            Command::from((Robot::Red, Compass::East)),
            Command::from((Robot::Red, Compass::North)),
            Command::from((Robot::Red, Compass::East)),
            Command::from((Robot::Red, Compass::South)),
            Command::from((Robot::Red, Compass::West)),
        ])
    )
}

#[test]
fn beginner_03() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((4, 0))),
        (Robot::Orange, Position::from((3, 4))),
        (Robot::Yellow, Position::from((1, 0))),
        (Robot::Green, Position::from((1, 3))),
        (Robot::Purple, Position::from((3, 1))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(5));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}

#[test]
fn beginner_04() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((1, 0))),
        (Robot::Orange, Position::from((0, 4))),
        (Robot::Yellow, Position::from((2, 1))),
        (Robot::Green, Position::from((4, 4))),
        (Robot::Purple, Position::from((3, 3))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(5));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}

#[test]
fn beginner_05() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((1, 0))),
        (Robot::Orange, Position::from((3, 4))),
        (Robot::Green, Position::from((1, 3))),
        (Robot::Purple, Position::from((3, 0))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(5));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}

#[test]
fn beginner_06() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((2, 4))),
        (Robot::Orange, Position::from((0, 4))),
        (Robot::Green, Position::from((4, 1))),
        (Robot::Purple, Position::from((1, 0))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(5));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}

#[test]
fn beginner_07() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((1, 0))),
        (Robot::Yellow, Position::from((3, 0))),
        (Robot::Orange, Position::from((2, 4))),
        (Robot::Green, Position::from((0, 3))),
        (Robot::Purple, Position::from((3, 2))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(6));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}

#[test]
fn beginner_08() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((3, 1))),
        (Robot::Yellow, Position::from((0, 0))),
        (Robot::Orange, Position::from((0, 4))),
        (Robot::Green, Position::from((3, 4))),
        (Robot::Purple, Position::from((1, 3))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(6));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}

#[test]
fn beginner_09() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((2, 4))),
        (Robot::Yellow, Position::from((3, 0))),
        (Robot::Orange, Position::from((4, 4))),
        (Robot::Green, Position::from((2, 2))),
        (Robot::Purple, Position::from((1, 1))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(6));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}

#[test]
fn beginner_10() {
    let configuration = Configuration::from(vec![
        (Robot::Red, Position::from((4, 0))),
        (Robot::Yellow, Position::from((2, 0))),
        (Robot::Orange, Position::from((1, 3))),
        (Robot::Green, Position::from((4, 2))),
        (Robot::Purple, Position::from((0, 0))),
    ]);
    let target = Target::from((Robot::Red, Position::from((2, 2))));
    let solver = IterativeDeepening::from(Depth::Finite(7));

    let solution = solver.solve(&configuration, &target);

    assert!(solution.is_some())
}
