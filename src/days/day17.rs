use crate::coord_system::direction::*;
use crate::coord_system::grid::*;
use crate::coord_system::unsigned::*;
use crate::intcode::*;
use crate::solver::Solver;

pub struct Day17 {}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Scaffold,
    Robot(Direction),
}

#[derive(Debug, PartialEq, Eq)]
enum PathSegment {
    Function(char),
    Movement(Direction, usize),
}

use Cell::*;
use PathSegment::*;

impl<'a> Solver<'a> for Day17 {
    type Generated = IntCode;
    type Output = usize;

    fn generator(input: &'a str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        let mut outputs = intcode.run_predetermined(&[]);
        outputs.truncate(outputs.len() - 1);

        let grid = parse_grid(outputs.iter().map(|&x| x as u8 as char)).0;
        calculate_alignment(grid)
    }

    fn part2(mut intcode: Self::Generated) -> Self::Output {
        let mut outputs = intcode.clone().run_predetermined(&[]);
        outputs.truncate(outputs.len() - 1);

        let (grid, robot_pos) = parse_grid(outputs.iter().map(|&x| x as u8 as char));
        let path = compute_path(grid, robot_pos);
        let format_path = format_path(path);
        let input = format_path
            .chars()
            .map(|c| c as IntCodeCell)
            .collect::<Vec<_>>();

        intcode.replace_cell(0, 2);
        let outputs = intcode.run_predetermined(&input);
        *outputs.last().unwrap() as usize
    }
}

fn parse_grid(outputs: impl IntoIterator<Item = char>) -> (Grid<Cell>, Point) {
    let mut grid = Grid::new();
    let mut row = Vec::new();
    let mut robot_pos = Point { x: 0, y: 0 };

    for c in outputs {
        match c {
            '.' => row.push(Empty),
            '#' => row.push(Scaffold),
            '^' | '>' | 'v' | '<' => {
                robot_pos = Point {
                    y: grid.len(),
                    x: row.len(),
                };
                row.push(Robot(match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                }));
            }
            '\n' => {
                grid.push(row);
                row = Vec::new()
            }
            _ => unreachable!(),
        }
    }

    (grid, robot_pos)
}

fn calculate_alignment(grid: Grid<Cell>) -> usize {
    let mut result = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            let p = Point { x, y };
            if ALL_DIRECTIONS
                .iter()
                .all(|&d| grid[p.add_dir(d).unwrap()] == Scaffold)
                && grid[p] == Scaffold
            {
                result += y * x;
            }
        }
    }

    result
}

fn compute_path(grid: Grid<Cell>, mut pos: Point) -> Vec<PathSegment> {
    let mut direction = if let Robot(x) = grid[pos] {
        x
    } else {
        unreachable!()
    };

    let mut path = Vec::new();

    loop {
        let check_turn = |x: fn(Direction) -> Direction| {
            let new_pos = pos.add_dir(x(direction));
            if let Some(new_pos) = new_pos {
                grid.in_bounds(new_pos) && grid[new_pos] == Scaffold
            } else {
                false
            }
        };

        let turn = if check_turn(|d| d.turn_left()) {
            (direction.turn_left(), Direction::Left)
        } else if check_turn(|d| d.turn_right()) {
            (direction.turn_right(), Direction::Right)
        } else {
            break;
        };

        direction = turn.0;

        let mut distance = 0;
        while {
            let new_pos = pos.add_dir(direction);
            new_pos.is_some()
                && grid.in_bounds(new_pos.unwrap())
                && grid[new_pos.unwrap()] == Scaffold
        } {
            pos = pos.add_dir(direction).unwrap();
            distance += 1;
        }

        path.push(Movement(turn.1, distance));
    }

    path
}

fn format_path(mut path: Vec<PathSegment>) -> String {
    let mut output = String::new();

    for function in &['A', 'B', 'C'] {
        let (indexes, pattern_length) = longest(&path);

        for m in &path[indexes[0]..indexes[0] + pattern_length] {
            if let Movement(dir, dis) = m {
                output.push_str(&format!(
                    "{},{},",
                    match dir {
                        Direction::Left => 'L',
                        Direction::Right => 'R',
                        _ => unreachable!(),
                    },
                    dis
                ))
            } else {
                unreachable!()
            }
        }

        output.replace_range(output.len() - 1.., "\n");

        for i in indexes.into_iter().rev() {
            path[i] = Function(*function);
            path.drain(i + 1..i + pattern_length);
        }
    }
    output.push_str("n\n");

    output.insert(0, '\n');
    for f in path.into_iter().rev() {
        if let Function(c) = f {
            output.insert(0, c);
            output.insert(0, ',');
        } else {
            unreachable!();
        }
    }
    output.remove(0);
    output
}

fn longest(path: &[PathSegment]) -> (Vec<usize>, usize) {
    let mut length = 0;
    let mut indexes = vec![];

    for i in 0..path.len() {
        for j in i + 2..std::cmp::min(path.len(), i + 2 + 4) {
            let pattern = &path[i..j];
            if pattern.len() > length
                && pattern
                    .iter()
                    .all(|x| if let Movement(_, _) = x { true } else { false })
            {
                let mut instances = vec![i];
                let mut range_check = j..=path.len() - pattern.len();
                while let Some(k) = range_check.next() {
                    if pattern.iter().zip(&path[k..]).all(|(x, y)| x == y) {
                        instances.push(k);
                        range_check.nth(pattern.len() - 1);
                    }
                }

                if instances.len() > 1 {
                    indexes = instances;
                    length = pattern.len();
                }
            }
        }
    }

    (indexes, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d17p1() {
        let input = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..
";
        assert_eq!(calculate_alignment(parse_grid(input.chars()).0), 76);
    }

    #[test]
    fn d17p2() {
        let input = "#######...#####
#.....#...#...#
#.....#...#...#
......#...#...#
......#...###.#
......#.....#.#
^########...#.#
......#.#...#.#
......#########
........#...#..
....#########..
....#...#......
....#...#......
....#...#......
....#####......
";
        let (grid, robot_pos) = parse_grid(input.chars());
        let path = compute_path(grid, robot_pos);
        assert_eq!(
            path,
            &[
                Movement(Direction::Right, 8),
                Movement(Direction::Right, 8),
                Movement(Direction::Right, 4),
                Movement(Direction::Right, 4),
                Movement(Direction::Right, 8),
                Movement(Direction::Left, 6),
                Movement(Direction::Left, 2),
                Movement(Direction::Right, 4),
                Movement(Direction::Right, 4),
                Movement(Direction::Right, 8),
                Movement(Direction::Right, 8),
                Movement(Direction::Right, 8),
                Movement(Direction::Left, 6),
                Movement(Direction::Left, 2)
            ]
        );

        format_path(path);
    }
}
