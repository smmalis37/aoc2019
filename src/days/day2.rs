use crate::intcode::*;
use crate::solver::Solver;

pub struct Day2 {}

impl<'a> Solver<'a> for Day2 {
    type Generated = IntCode;
    type Output = IntCodeCell;

    fn generator(input: &'a str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        intcode.run_no_io(&[(1, 12), (2, 2)])[0]
    }

    fn part2(start_intcode: Self::Generated) -> Self::Output {
        for verb in 0..=99 {
            for noun in 0..=99 {
                let intcode = start_intcode.clone();
                if intcode.run_no_io(&[(1, noun), (2, verb)])[0] == 19_690_720 {
                    return 100 * noun + verb;
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1() {
        fn test(program: &str, expected_output: &[IntCodeCell]) {
            let finished_memory = program.parse::<IntCode>().unwrap().run_no_io(&[]);
            assert_eq!(finished_memory, expected_output);
        }

        test("1,0,0,0,99", &[2, 0, 0, 0, 99]);
        test("2,3,0,3,99", &[2, 3, 0, 6, 99]);
        test("2,4,4,5,99,0", &[2, 4, 4, 5, 99, 9801]);
        test("1,1,1,4,99,5,6,0,99", &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
