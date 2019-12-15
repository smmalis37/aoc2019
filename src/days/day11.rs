use crate::helpers::coord_system::*;
use crate::helpers::intcode::*;
use crate::solver::Solver;
use std::collections::HashMap;

pub struct Day11 {}

impl<'a> Solver<'a> for Day11 {
    type Generated = impl FnOnce(IntCodeCell) -> HashMap<SignedCoordinate, IntCodeCell> + Clone;
    type Output = usize;

    fn generator(input: &'a str) -> Self::Generated {
        let intcode = input.parse().unwrap();
        move |x| run_bot(intcode, x)
    }

    fn part1(bot: Self::Generated) -> Self::Output {
        bot(0).len()
    }

    fn part2(bot: Self::Generated) -> Self::Output {
        let grid = bot(1);

        let (mut minx, mut maxx, mut miny, mut maxy) = (0, 0, 0, 0);

        for c in grid.keys() {
            if c.x < minx {
                minx = c.x;
            }
            if c.x > maxx {
                maxx = c.x;
            }
            if c.y < miny {
                miny = c.y;
            }
            if c.y > maxy {
                maxy = c.y;
            }
        }

        println!();
        for y in (miny..=maxy).rev() {
            for x in minx..=maxx {
                print!(
                    "{}",
                    match grid.get(&SignedCoordinate { x, y }).unwrap_or(&0) {
                        0 => ' ',
                        1 => '█',
                        _ => unreachable!(),
                    },
                )
            }
            println!();
        }

        0
    }
}

fn run_bot(intcode: IntCode, start_value: IntCodeCell) -> HashMap<SignedCoordinate, IntCodeCell> {
    let mut position = SignedCoordinate { x: 0, y: 0 };
    let mut direction = Direction::Up;
    let mut grid = HashMap::new();

    grid.insert(position, start_value);

    let _ = intcode.run_demand_driven(|outputs| {
        for o in outputs.chunks(2) {
            grid.insert(position, o[0]);

            if o[1] == 0 {
                direction = direction.turn_left();
            } else {
                direction = direction.turn_right();
            }

            position += direction.to_unit();
        }

        *grid.get(&position).unwrap_or(&0)
    });

    grid
}
