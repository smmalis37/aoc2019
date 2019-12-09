use crate::intcode::*;
use crate::solver::Solver;
use crossbeam::channel::*;
use permutohedron::Heap;

pub struct Day7 {}

impl<'a> Solver<'a> for Day7 {
    type Generated = IntCode;
    type Output = IntCodeCell;

    fn generator(input: &'a str) -> Self::Generated {
        parse_intcode(input)
    }

    fn part1(start_intcode: Self::Generated) -> Self::Output {
        let mut phases = [0, 1, 2, 3, 4];
        let mut max_signal = 0;

        for settings in Heap::new(&mut phases) {
            let mut signal = 0;

            for phase in settings.iter() {
                let intcode = start_intcode.clone();
                let output = intcode.run_single_threaded(&[*phase, signal]);
                signal = output[0];
            }

            max_signal = std::cmp::max(max_signal, signal);
        }

        max_signal
    }

    fn part2(start_intcode: Self::Generated) -> Self::Output {
        let mut phases = [5, 6, 7, 8, 9];
        let mut max_signal = 0;
        let channels = [
            unbounded(),
            unbounded(),
            unbounded(),
            unbounded(),
            unbounded(),
        ];
        let threads = rayon::ThreadPoolBuilder::new()
            .num_threads(5)
            .build()
            .unwrap();

        for settings in Heap::new(&mut phases) {
            assert!(channels.iter().all(|x| x.0.is_empty() && x.1.is_empty()));
            let wg = crossbeam::sync::WaitGroup::new();

            for (index, phase) in settings.iter().enumerate() {
                let intcode = start_intcode.clone();
                let wg = wg.clone();

                let (input_send, input_recv) = channels[index].clone();
                input_send.send(*phase).unwrap();
                let (output_send, _) = if index == 4 {
                    channels[0].clone()
                } else {
                    channels[index + 1].clone()
                };

                threads.spawn(move || {
                    intcode.run_multi_threaded(input_recv, output_send);
                    wg.wait();
                });
            }

            channels[0].0.send(0).unwrap();
            wg.wait();
            let signal = channels[0].1.recv().unwrap();
            max_signal = std::cmp::max(max_signal, signal);
        }

        max_signal
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
    fn d7p2() {
        assert_eq!(
            Day7::part2(Day7::generator(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            )),
            139_629_729
        );
        assert_eq!(
            Day7::part2(Day7::generator(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            )),
            18216
        );
    }
}
