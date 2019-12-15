use crate::intcode::*;
use crate::solver::Solver;
use std::cmp::Ordering;

pub struct Day13 {}

impl<'a> Solver<'a> for Day13 {
    type Generated = IntCode;
    type Output = usize;

    fn generator(input: &'a str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        let outputs = intcode.run_single_threaded(&[]);
        outputs.chunks(3).filter(|&x| x[2] == 2).count()
    }

    fn part2(mut intcode: Self::Generated) -> Self::Output {
        intcode.replace_cell(0, 2);
        let (input_send, output_recv, thread) = intcode.spawn_multi_threaded(Some(1), None);
        let mut score = 0;
        let mut paddle_x = None;
        let mut ball_x = None;
        let mut buffer = [0; 3];
        let mut buffer_index = 0;

        loop {
            let recv_result = output_recv.recv();
            if recv_result.is_err() {
                break;
            }
            buffer[buffer_index] = recv_result.unwrap();
            buffer_index += 1;

            if buffer_index == 3 {
                let x = buffer[0];
                let y = buffer[1];
                let tile = buffer[2];
                buffer_index = 0;

                if x == -1 && y == 0 {
                    score = tile;
                } else if tile == 3 {
                    paddle_x = Some(x);
                } else if tile == 4 {
                    ball_x = Some(x);
                }
            }

            if paddle_x.is_none() || ball_x.is_none() {
                continue;
            }

            let action = match paddle_x.cmp(&ball_x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            input_send.send(action).unwrap();

            if action != 0 {
                paddle_x = None;
            }

            ball_x = None;
        }

        thread.join().unwrap();
        score as usize
    }
}
