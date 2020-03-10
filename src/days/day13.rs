use crate::intcode::*;
use crate::solver::Solver;
use std::cmp::Ordering;

pub struct Day13 {}

impl Solver<'_> for Day13 {
    type Generated = IntCode;
    type Output = IntCodeCell;

    fn generator(input: &str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        let outputs = intcode.run_predetermined(&[]);
        outputs.chunks(3).filter(|&x| x[2] == 2).count() as IntCodeCell
    }

    fn part2(mut intcode: Self::Generated) -> Self::Output {
        intcode.replace_cell(0, 2);
        let mut score = 0;
        let mut previous_outputs = [None, None];

        intcode.run_demand_driven(
            (0, 0),
            |(paddle_x, ball_x), o| {
                if let Some(x) = previous_outputs[0] {
                    if let Some(y) = previous_outputs[1] {
                        let tile = o;

                        if x == -1 && y == 0 {
                            score = tile;
                        } else if tile == 3 {
                            *paddle_x = x;
                        } else if tile == 4 {
                            *ball_x = x;
                        }
                        previous_outputs = [None, None];
                    } else {
                        previous_outputs[1] = Some(o);
                    }
                } else {
                    previous_outputs[0] = Some(o);
                }
            },
            |(paddle_x, ball_x)| match paddle_x.cmp(&ball_x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
        );

        score
    }
}
