use crate::intcode::*;
use crate::solver::Solver;
use permutohedron::Heap;

pub struct Day7 {}

impl<'a> Solver<'a> for Day7 {
    type Generated = IntCode;
    type Output = isize;

    fn generator(input: &'a str) -> Self::Generated {
        parse_intcode(input)
    }

    fn part1(start_memory: Self::Generated) -> Self::Output {
        let mut phases = [0, 1, 2, 3, 4];
        let mut max_signal = 0;

        for settings in Heap::new(&mut phases) {
            let mut signal = 0;

            for phase in settings.iter() {
                let mut memory = start_memory.clone();
                let output = run_intcode(&mut memory, [*phase, signal].iter().copied());
                signal = output[0];
            }

            max_signal = std::cmp::max(max_signal, signal);
        }

        max_signal
    }

    fn part2(data: Self::Generated) -> Self::Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d7p1() {
        assert_eq!(
            Day7::part1(Day7::generator(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            )),
            43210
        );
        assert_eq!(
            Day7::part1(Day7::generator(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            )),
            54321
        );
        assert_eq!(
            Day7::part1(Day7::generator(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )),
            65210
        );
    }

    #[test]
    fn d7p2() {}
}
