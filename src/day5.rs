use crate::intcode::*;

pub fn generator(input: &str) -> Vec<isize> {
    parse_intcode(input)
}

pub fn part1(mut memory: Vec<isize>) -> isize {
    let outputs = run_intcode(&mut memory, vec![1]);
    assert!(outputs[..outputs.len() - 1].iter().all(|&x| x == 0));
    outputs[0]
}

pub fn part2(mut memory: Vec<isize>) -> isize {
    let outputs = run_intcode(&mut memory, vec![5]);
    outputs[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test(mut memory: Vec<isize>, inputs: Vec<isize>, expected_output: &[isize]) {
        let outputs = run_intcode(&mut memory, inputs);
        assert_eq!(outputs, expected_output);
    }

    #[test]
    fn d5p1() {
        test(vec![3, 0, 4, 0, 99], vec![7], &[7]);
        test(vec![1002, 1, 4, 4, -1, 4, 99], vec![], &[4]);
    }

    #[test]
    fn d5p2() {
        test(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![8], &[1]);
        test(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], vec![9], &[0]);
        test(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![-4], &[1]);
        test(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], vec![9], &[0]);
        test(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![8], &[1]);
        test(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], vec![9], &[0]);
        test(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![-4], &[1]);
        test(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], vec![9], &[0]);

        test(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![0],
            &[0],
        );
        test(
            vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            vec![743],
            &[1],
        );
        test(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![0],
            &[0],
        );
        test(
            vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            vec![743],
            &[1],
        );

        test(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![-4],
            &[999],
        );
        test(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![8],
            &[1000],
        );
        test(
            vec![
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ],
            vec![3512],
            &[1001],
        );
    }
}
