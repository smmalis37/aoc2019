use crate::intcode::*;
use crate::solver::Solver;

pub struct Day9 {}

impl<'a> Solver<'a> for Day9 {
    type Generated = IntCode;
    type Output = IntCodeCell;

    fn generator(input: &'a str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        intcode.run_single_threaded(&[1])[0]
    }

    fn part2(intcode: Self::Generated) -> Self::Output {
        intcode.run_single_threaded(&[2])[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(program: &str, expected_output: &[IntCodeCell]) {
        let outputs = program.parse::<IntCode>().unwrap().run_single_threaded(&[]);
        assert_eq!(outputs, expected_output);
    }

    #[test]
    fn d9p1() {
        test(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
            &[
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
        );

        test(
            "1102,34915192,34915192,7,4,7,99,0",
            &[1_219_070_632_396_864],
        );

        test("104,1125899906842624,99", &[1_125_899_906_842_624]);
    }

    #[test]
    fn d9p2() {}
}
