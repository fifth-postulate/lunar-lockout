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

mod beginner {
    use super::*;
    #[test]
    fn p01() {
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
    fn p02() {
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
    fn p03() {
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
    fn p04() {
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
    fn p05() {
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
    fn p06() {
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
    fn p07() {
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
    fn p08() {
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
    fn p09() {
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
    fn p10() {
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
}

mod intermediate {
    use super::*;
    #[test]
    fn p11() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 3))),
            (Robot::Orange, Position::from((2, 4))),
            (Robot::Yellow, Position::from((2, 2))),
            (Robot::Blue, Position::from((2, 1))),
            (Robot::Green, Position::from((0, 3))),
            (Robot::Purple, Position::from((4, 3))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(6));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p12() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Green, Position::from((4, 3))),
            (Robot::Purple, Position::from((1, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(5));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p13() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((3, 1))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Yellow, Position::from((2, 0))),
            (Robot::Green, Position::from((3, 3))),
            (Robot::Purple, Position::from((0, 1))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(9));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[ignore]
    #[test]
    fn p14() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((1, 0))),
            (Robot::Orange, Position::from((2, 4))),
            (Robot::Yellow, Position::from((3, 1))),
            (Robot::Green, Position::from((4, 3))),
            (Robot::Purple, Position::from((0, 2))),
            (Robot::Blue, Position::from((0, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(13));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p15() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((4, 4))),
            (Robot::Yellow, Position::from((2, 1))),
            (Robot::Green, Position::from((1, 3))),
            (Robot::Purple, Position::from((4, 2))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(6));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p16() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Yellow, Position::from((1, 0))),
            (Robot::Green, Position::from((1, 2))),
            (Robot::Purple, Position::from((4, 2))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(6));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p17() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((1, 0))),
            (Robot::Yellow, Position::from((3, 1))),
            (Robot::Orange, Position::from((2, 4))),
            (Robot::Green, Position::from((4, 4))),
            (Robot::Purple, Position::from((1, 2))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(8));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p18() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((0, 0))),
            (Robot::Green, Position::from((2, 0))),
            (Robot::Purple, Position::from((4, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(6));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p19() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Yellow, Position::from((0, 1))),
            (Robot::Green, Position::from((2, 3))),
            (Robot::Purple, Position::from((3, 2))),
            (Robot::Blue, Position::from((2, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(9));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p20() {
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

        assert!(solution.is_some())
    }
}
