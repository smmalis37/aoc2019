use crate::coord_system::direction::*;
use crate::coord_system::signed::*;
use crate::intcode::*;
use crate::solver::Solver;
use std::collections::HashMap;

pub struct Day15 {}

type N = i32;

impl Solver<'_> for Day15 {
    type Generated = IntCode;
    type Output = N;

    fn generator(input: &str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        run_bot(intcode).0
    }

    fn part2(intcode: Self::Generated) -> Self::Output {
        run_bot(intcode).1
    }
}

fn run_bot(intcode: IntCode) -> (N, N) {
    use Direction::*;
    let all_directions: Vec<_> = ALL_DIRECTIONS[..].into();

    let position = Point { x: 0, y: 0 };
    let mut objective_distance = 0;
    let mut max_distance = 0;

    let mut reset = false;
    let origin = Point { x: 0, y: 0 };
    let distance = 0;
    let last_direction = Up;
    let mut data = HashMap::new();
    data.insert(position, (all_directions.clone(), Up));

    intcode.run_with_fns(
        (data, position, origin, distance, last_direction),
        |(data, position, origin, distance, last_direction), o| match o {
            0 => (),
            1 | 2 => {
                *position = position.add_dir(*last_direction);
                *distance += 1;

                if o == 2 && !reset {
                    objective_distance = *distance;

                    *origin = *position;
                    *distance = 0;
                    data.clear();
                    data.insert(*position, (all_directions.clone(), Up));
                    max_distance = 0;
                    reset = true;
                } else {
                    max_distance = std::cmp::max(max_distance, *distance);
                    let go_back = last_direction.opposite();
                    let mut next_steps = all_directions.clone();
                    next_steps.remove((to_value(go_back) - 1) as usize);
                    data.entry(*position).or_insert((next_steps, go_back));
                }
            }
            _ => unreachable!(),
        },
        |(data, position, origin, distance, last_direction)| {
            let coord_data = data.get_mut(position).unwrap();
            if coord_data.0.is_empty() {
                if position == origin {
                    99
                } else {
                    *distance -= 2;
                    *last_direction = coord_data.1;
                    to_value(coord_data.1)
                }
            } else {
                *last_direction = coord_data.0.pop().unwrap();
                to_value(*last_direction)
            }
        },
    );

    (objective_distance, max_distance)
}

fn to_value(d: Direction) -> IntCodeCell {
    use Direction::*;
    match d {
        Up => 1,
        Down => 2,
        Left => 3,
        Right => 4,
    }
}
