use crate::helpers::coord_system::{Direction::*, *};
use crate::helpers::intcode::*;
use crate::solver::Solver;
use std::collections::HashMap;

pub struct Day15 {}

type Coordinate = SignedCoordinate;

impl<'a> Solver<'a> for Day15 {
    type Generated = IntCode;
    type Output = u32;

    fn generator(input: &'a str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        let all_directions = vec![Up, Down, Left, Right];
        let mut position = Coordinate { x: 0, y: 0 };
        let mut distance = 0;
        let mut last_direction = Up;
        let mut data = HashMap::<Coordinate, (Vec<Direction>, Direction)>::new();
        data.insert(position, (all_directions.clone(), Up));

        let _ = intcode.run_demand_driven(|o| {
            if !o.is_empty() {
                let result = o[0];
                match result {
                    0 => (),
                    1 => {
                        position += last_direction.to_unit();
                        distance += 1;
                        let go_back = opposite(last_direction);
                        let mut next_steps = all_directions.clone();
                        next_steps.remove((to_value(go_back) - 1) as usize);
                        data.entry(position).or_insert((next_steps, go_back));
                    }
                    2 => {
                        distance += 1;
                        return 99;
                    }
                    _ => unreachable!(),
                }
            }

            let coord_data = data.get_mut(&position).unwrap();
            if coord_data.0.is_empty() {
                distance -= 2;
                last_direction = coord_data.1;
                to_value(coord_data.1)
            } else {
                last_direction = coord_data.0.pop().unwrap();
                to_value(last_direction)
            }
        });

        distance
    }

    fn part2(intcode: Self::Generated) -> Self::Output {
        0
    }
}

fn to_value(d: Direction) -> IntCodeCell {
    match d {
        Up => 1,
        Down => 2,
        Left => 3,
        Right => 4,
    }
}

fn opposite(d: Direction) -> Direction {
    match d {
        Up => Down,
        Down => Up,
        Left => Right,
        Right => Left,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d15p2() {}
}
