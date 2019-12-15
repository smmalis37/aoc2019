use crate::coord_system::*;
use crate::intcode::*;
use crate::solver::Solver;
use crossbeam::channel::unbounded;
use std::collections::HashMap;

pub struct Day11 {}

impl<'a> Solver<'a> for Day11 {
    type Generated = IntCode;
    type Output = usize;

    fn generator(input: &'a str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        let mut position = SignedCoordinate { x: 0, y: 0 };
        let mut direction = Direction::Up;
        let mut grid = HashMap::<SignedCoordinate, IntCodeCell>::new();
        let (input_send, input_recv) = unbounded();
        let (output_send, output_recv) = unbounded();

        let thread = std::thread::spawn(|| intcode.run_multi_threaded(input_recv, output_send));

        let _: Result<_, Box<dyn std::error::Error>> = try {
            loop {
                input_send.send(*grid.get(&position).unwrap_or(&0))?;
                grid.insert(position, output_recv.recv()?);
                let turn = output_recv.recv()?;

                if turn == 0 {
                    direction = direction.turn_left();
                } else {
                    direction = direction.turn_right();
                }

                position += direction.to_unit();
            }
        };

        thread.join().unwrap();
        grid.len()
    }

    fn part2(intcode: Self::Generated) -> Self::Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d11p2() {}
}
