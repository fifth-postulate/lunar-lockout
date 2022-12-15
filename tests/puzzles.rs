extern crate lunarlockout;

use lunarlockout::{
    solver::{
        ids::{Depth, IterativeDeepening},
        Solution, Solver, Target,
    },
    state::{
        command::Command, configuration::Configuration, direction::Compass, position::Position,
        robot::Robot,
    },
};

mod preflight {
    use super::*;
    #[test]
    fn p01() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((2, 1))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(2));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some());
    }

    #[test]
    fn p02() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((4, 3))),
            (Robot::Green, Position::from((1, 2))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(3));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p03() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((0, 2))),
            (Robot::Orange, Position::from((3, 4))),
            (Robot::Green, Position::from((3, 1))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(3));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p04() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((0, 3))),
            (Robot::Orange, Position::from((2, 3))),
            (Robot::Green, Position::from((3, 1))),
            (Robot::Purple, Position::from((1, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(5));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some());
    }

    #[test]
    fn p05() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((3, 3))),
            (Robot::Green, Position::from((1, 1))),
            (Robot::Purple, Position::from((3, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(4));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p06() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((0, 3))),
            (Robot::Green, Position::from((2, 2))),
            (Robot::Purple, Position::from((2, 1))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(4));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p07() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 3))),
            (Robot::Orange, Position::from((0, 3))),
            (Robot::Green, Position::from((3, 3))),
            (Robot::Purple, Position::from((2, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(4));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p08() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 2))),
            (Robot::Orange, Position::from((2, 3))),
            (Robot::Green, Position::from((0, 2))),
            (Robot::Purple, Position::from((2, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(4));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p09() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 3))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Green, Position::from((0, 1))),
            (Robot::Purple, Position::from((2, 1))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(5));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p10() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((3, 1))),
            (Robot::Orange, Position::from((3, 3))),
            (Robot::Green, Position::from((1, 3))),
            (Robot::Purple, Position::from((0, 1))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(5));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }
}

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
            Some(Solution::from(vec![
                Command::from((Robot::Red, Compass::North)),
                Command::from((Robot::Red, Compass::West)),
                Command::from((Robot::Red, Compass::South)),
                Command::from((Robot::Red, Compass::West)),
            ]))
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
            Some(Solution::from(vec![
                Command::from((Robot::Red, Compass::North)),
                Command::from((Robot::Red, Compass::East)),
                Command::from((Robot::Red, Compass::North)),
                Command::from((Robot::Red, Compass::East)),
                Command::from((Robot::Red, Compass::South)),
                Command::from((Robot::Red, Compass::West)),
            ]))
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

mod advanced {
    use super::*;
    #[test]
    fn p21() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((2, 3))),
            (Robot::Yellow, Position::from((0, 1))),
            (Robot::Blue, Position::from((3, 0))),
            (Robot::Green, Position::from((2, 2))),
            (Robot::Purple, Position::from((4, 2))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(7));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p22() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((0, 3))),
            (Robot::Green, Position::from((4, 3))),
            (Robot::Purple, Position::from((3, 1))),
            (Robot::Yellow, Position::from((0, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(7));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p23() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((1, 0))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Yellow, Position::from((4, 1))),
            (Robot::Green, Position::from((0, 3))),
            (Robot::Purple, Position::from((3, 3))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(7));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p24() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((0, 0))),
            (Robot::Orange, Position::from((3, 3))),
            (Robot::Yellow, Position::from((4, 0))),
            (Robot::Green, Position::from((1, 2))),
            (Robot::Purple, Position::from((3, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(7));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p25() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((1, 0))),
            (Robot::Orange, Position::from((0, 3))),
            (Robot::Yellow, Position::from((4, 0))),
            (Robot::Green, Position::from((2, 3))),
            (Robot::Purple, Position::from((3, 3))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(8));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p26() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Yellow, Position::from((3, 1))),
            (Robot::Green, Position::from((4, 3))),
            (Robot::Purple, Position::from((0, 2))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(10));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p27() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Yellow, Position::from((3, 2))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Green, Position::from((4, 4))),
            (Robot::Purple, Position::from((0, 2))),
            (Robot::Blue, Position::from((0, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(8));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p28() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Green, Position::from((4, 4))),
            (Robot::Purple, Position::from((4, 2))),
            (Robot::Yellow, Position::from((0, 1))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(7));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p29() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Yellow, Position::from((4, 0))),
            (Robot::Green, Position::from((4, 4))),
            (Robot::Purple, Position::from((0, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(7));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p30() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Yellow, Position::from((1, 0))),
            (Robot::Orange, Position::from((0, 3))),
            (Robot::Green, Position::from((4, 3))),
            (Robot::Purple, Position::from((0, 2))),
            (Robot::Blue, Position::from((3, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(11));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }
}

mod expert {
    use super::*;
    #[test]
    fn p31() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((3, 1))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Yellow, Position::from((0, 2))),
            (Robot::Blue, Position::from((1, 0))),
            (Robot::Green, Position::from((2, 4))),
            (Robot::Purple, Position::from((4, 3))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(11));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p32() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((1, 0))),
            (Robot::Orange, Position::from((1, 4))),
            (Robot::Green, Position::from((4, 3))),
            (Robot::Purple, Position::from((0, 0))),
            (Robot::Yellow, Position::from((2, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(9));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p33() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Yellow, Position::from((0, 1))),
            (Robot::Green, Position::from((1, 4))),
            (Robot::Purple, Position::from((4, 3))),
            (Robot::Blue, Position::from((3, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(8));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[ignore]
    #[test]
    fn p34() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((2, 4))),
            (Robot::Yellow, Position::from((3, 1))),
            (Robot::Green, Position::from((4, 4))),
            (Robot::Purple, Position::from((0, 2))),
            (Robot::Blue, Position::from((0, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(11));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p35() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Yellow, Position::from((0, 0))),
            (Robot::Green, Position::from((4, 4))),
            (Robot::Purple, Position::from((2, 3))),
            (Robot::Blue, Position::from((4, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(11));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p36() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((2, 4))),
            (Robot::Orange, Position::from((0, 3))),
            (Robot::Yellow, Position::from((4, 1))),
            (Robot::Green, Position::from((4, 3))),
            (Robot::Purple, Position::from((0, 1))),
            (Robot::Blue, Position::from((2, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(9));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[test]
    fn p37() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((3, 1))),
            (Robot::Yellow, Position::from((0, 1))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Green, Position::from((3, 4))),
            (Robot::Purple, Position::from((4, 4))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(9));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[ignore]
    #[test]
    fn p38() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((1, 0))),
            (Robot::Orange, Position::from((2, 4))),
            (Robot::Green, Position::from((4, 4))),
            (Robot::Purple, Position::from((0, 2))),
            (Robot::Yellow, Position::from((1, 2))),
            (Robot::Blue, Position::from((4, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(13));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[ignore]
    #[test]
    fn p39() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((4, 0))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Yellow, Position::from((0, 2))),
            (Robot::Green, Position::from((2, 4))),
            (Robot::Purple, Position::from((4, 4))),
            (Robot::Blue, Position::from((0, 0))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(16));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }

    #[ignore]
    #[test]
    fn p40() {
        let configuration = Configuration::from(vec![
            (Robot::Red, Position::from((1, 0))),
            (Robot::Yellow, Position::from((4, 1))),
            (Robot::Orange, Position::from((0, 4))),
            (Robot::Green, Position::from((2, 4))),
            (Robot::Purple, Position::from((4, 4))),
        ]);
        let target = Target::from((Robot::Red, Position::from((2, 2))));
        let solver = IterativeDeepening::from(Depth::Finite(13));

        let solution = solver.solve(&configuration, &target);

        assert!(solution.is_some())
    }
}
