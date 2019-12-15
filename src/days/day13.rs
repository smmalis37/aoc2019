use crate::helpers::intcode::*;
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
        let outputs = intcode.run_predetermined(&[]);
        outputs.chunks(3).filter(|&x| x[2] == 2).count()
    }

    fn part2(mut intcode: Self::Generated) -> Self::Output {
        intcode.replace_cell(0, 2);
        let mut score = 0;
        let mut paddle_x = None;

        let final_outputs = intcode.run_demand_driven(|outputs| {
            let mut ball_x = None;

            for o in outputs.chunks(3) {
                let x = o[0];
                let y = o[1];
                let tile = o[2];

                if x == -1 && y == 0 {
                    score = tile;
                } else if tile == 3 {
                    paddle_x = Some(x);
                } else if tile == 4 {
                    ball_x = Some(x);
                }
            }

            let action = match paddle_x.unwrap().cmp(&ball_x.unwrap()) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            };

            if action != 0 {
                paddle_x = None;
            }

            action
        });

        for o in final_outputs.chunks(3) {
            if o[0] == -1 && o[1] == 0 {
                score = o[2];
            }
        }

        score as usize
    }
}
