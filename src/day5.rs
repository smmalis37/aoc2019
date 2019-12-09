use crate::intcode::*;
use crate::solver::Solver;

pub struct Day5 {}

impl<'a> Solver<'a> for Day5 {
    type Generated = IntCode;
    type Output = IntCodeCell;

    fn generator(input: &str) -> Self::Generated {
        parse_intcode(input)
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        let outputs = intcode.run_single_threaded(vec![1]);
        assert!(outputs[..outputs.len() - 1].iter().all(|&x| x == 0));
        *outputs.last().unwrap()
    }

    fn part2(intcode: Self::Generated) -> Self::Output {
        let outputs = intcode.run_single_threaded(vec![5]);
        outputs[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(program: &str, inputs: Vec<IntCodeCell>, expected_output: &[IntCodeCell]) {
        let outputs = parse_intcode(program).run_single_threaded(inputs);
        assert_eq!(outputs, expected_output);
    }

    #[test]
    fn d5p1() {
        test("3,0,4,0,99", vec![7], &[7]);
        test("1002,1,4,4,-1,4,99", vec![], &[4]);
    }

    #[test]
    fn d5p2() {
        test("3,9,8,9,10,9,4,9,99,-1,8", vec![8], &[1]);
        test("3,9,8,9,10,9,4,9,99,-1,8", vec![9], &[0]);
        test("3,9,7,9,10,9,4,9,99,-1,8", vec![-4], &[1]);
        test("3,9,7,9,10,9,4,9,99,-1,8", vec![9], &[0]);
        test("3,3,1108,-1,8,3,4,3,99", vec![8], &[1]);
        test("3,3,1108,-1,8,3,4,3,99", vec![9], &[0]);
        test("3,3,1107,-1,8,3,4,3,99", vec![-4], &[1]);
        test("3,3,1107,-1,8,3,4,3,99", vec![9], &[0]);

        test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![0], &[0]);
        test("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", vec![743], &[1]);
        test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![0], &[0]);
        test("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", vec![743], &[1]);

        test(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            vec![-4],
            &[999],
        );
        test(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            vec![8],
            &[1000],
        );
        test(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            vec![3512],
            &[1001],
        );
    }
}
