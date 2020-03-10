use crate::coord_system::direction::*;
use crate::coord_system::signed::*;
use crate::intcode::*;
use crate::solver::Solver;
use std::collections::HashMap;

pub struct Day11 {}

impl Solver<'_> for Day11 {
    type Generated = IntCode;
    type Output = usize;

    fn generator(input: &str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        run_bot(intcode, 0).len()
    }

    fn part2(intcode: Self::Generated) -> Self::Output {
        let grid = run_bot(intcode, 1);

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

        use std::io::Write;
        let stdout = std::io::stdout();
        let mut writer = stdout.lock();

        writeln!(writer).unwrap();
        for y in (miny..=maxy).rev() {
            for x in minx..=maxx {
                write!(
                    writer,
                    "{}",
                    match grid.get(&Point { x, y }).unwrap_or(&0) {
                        0 => ' ',
                        1 => '█',
                        _ => unreachable!(),
                    },
                )
                .unwrap()
            }
            writeln!(writer).unwrap();
        }

        0
    }
}

fn run_bot(intcode: IntCode, start_value: IntCodeCell) -> HashMap<Point, IntCodeCell> {
    let position = Point { x: 0, y: 0 };
    let mut direction = Direction::Up;
    let mut grid = HashMap::new();
    let mut previous_output = None;

    grid.insert(position, start_value);

    intcode.run_demand_driven(
        (&mut grid, position),
        |(grid, position), o| {
            if let Some(pos) = previous_output {
                grid.insert(*position, pos);

                if o == 0 {
                    direction = direction.turn_left();
                } else {
                    direction = direction.turn_right();
                }

                *position = position.add_dir(direction);
                previous_output = None;
            } else {
                previous_output = Some(o);
            }
        },
        |(grid, position)| *grid.get(position).unwrap_or(&0),
    );

    grid
}
