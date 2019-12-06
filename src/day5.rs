use crate::intcode::*;

pub fn generator(input: &str) -> Vec<isize> {
    parse_intcode(input)
}

pub fn part1(mut memory: Vec<isize>) -> isize {
    let outputs = run_intcode(&mut memory, vec![1]);
    assert!(outputs[..outputs.len() - 1].iter().all(|&x| x == 0));
    *outputs.last().unwrap()
}

pub fn part2(memory: Vec<isize>) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5p1() {
        fn test(mut memory: Vec<isize>, inputs: Vec<isize>, expected_output: &[isize]) {
            let outputs = run_intcode(&mut memory, inputs);
            assert_eq!(outputs, expected_output);
        }

        test(vec![3, 0, 4, 0, 99], vec![7], &[7]);
        test(vec![1002, 1, 4, 4, -1, 4, 99], vec![], &[4]);
    }

    #[test]
    fn d5p2() {}
}
