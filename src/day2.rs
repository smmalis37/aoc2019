use crate::intcode::*;

pub fn generator(input: &str) -> Vec<isize> {
    parse_intcode(input)
}

pub fn part1(mut memory: Vec<isize>) -> isize {
    memory[1] = 12;
    memory[2] = 2;
    run_intcode(&mut memory)[0]
}

pub fn part2(start_memory: Vec<isize>) -> isize {
    for verb in 0..=99 {
        for noun in 0..=99 {
            let mut memory = start_memory.clone();
            memory[1] = noun;
            memory[2] = verb;
            run_intcode(&mut memory);
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
        assert_eq!(run_intcode(&mut [1, 0, 0, 0, 99]), &[2, 0, 0, 0, 99]);
        assert_eq!(run_intcode(&mut [2, 3, 0, 3, 99]), &[2, 3, 0, 6, 99]);
        assert_eq!(
            run_intcode(&mut [2, 4, 4, 5, 99, 0]),
            &[2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_intcode(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]),
            &[30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
