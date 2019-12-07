use crate::intcode::*;
use std::iter::empty;

pub fn generator(input: &str) -> Vec<isize> {
    parse_intcode(input)
}

pub fn part1(mut memory: Vec<isize>) -> isize {
    memory[1] = 12;
    memory[2] = 2;
    run_intcode(&mut memory, empty());
    memory[0]
}

pub fn part2(start_memory: Vec<isize>) -> isize {
    for verb in 0..=99 {
        for noun in 0..=99 {
            let mut memory = start_memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            run_intcode(&mut memory, empty());
            if memory[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d2p1() {
        fn test(mut memory: Vec<isize>, expected_output: &[isize]) {
            run_intcode(&mut memory, empty());
            assert_eq!(memory, expected_output);
        }

        test(vec![1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
        test(vec![2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
        test(vec![2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
        test(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            &[30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }
}
