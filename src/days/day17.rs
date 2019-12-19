use crate::coord_system::direction::*;
use crate::coord_system::unsigned::*;
use crate::intcode::*;
use crate::solver::Solver;

pub struct Day17 {}

#[derive(PartialEq, Eq)]
enum Cell {
    Empty,
    Scaffold,
    Robot(Direction),
}

use Cell::*;

impl<'a> Solver<'a> for Day17 {
    type Generated = IntCode;
    type Output = IntCodeCell;

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
        let path = compute_path(grid, robot_pos)
            .chars()
            .map(|c| c as IntCodeCell)
            .collect::<Vec<_>>();

        intcode.replace_cell(0, 2);
        let outputs = intcode.run_predetermined(&path);
        *outputs.last().unwrap()
    }
}

fn parse_grid(outputs: impl IntoIterator<Item = char>) -> (Vec<Vec<Cell>>, Point) {
    let mut grid = Vec::new();
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

fn calculate_alignment<'a>(grid: Vec<Vec<Cell>>) -> <Day17 as Solver<'a>>::Output {
    let mut result = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .all(|&(ymod, xmod)| grid[y + ymod][x + xmod] == Scaffold)
            {
                result += y * x;
            }
        }
    }

    result
}

fn compute_path(grid: Vec<Vec<Cell>>, mut pos: Point) -> String {
    let mut direction = if let Robot(x) = grid[pos] {
        x
    } else {
        unreachable!()
    };

    let mut path = Vec::new();

    loop {
        let check_turn = |x: fn(Direction) -> Direction| {
            let new_pos = pos + x(direction).to_unit();
            grid.in_bounds(new_pos) && grid[new_pos] == Scaffold
        };
        let turn = if check_turn(|d| d.turn_left()) {
            (direction.turn_left(), Direction::Left)
        } else if check_turn(|d| d.turn_right()) {
            (direction.turn_right(), Direction::Right)
        } else {
            break;
        };

        println!("{:?}", turn);
        direction = turn.0;

        let mut distance = 0;
        while grid.in_bounds(pos + direction.to_unit())
            && grid[pos + direction.to_unit()] == Scaffold
        {
            pos.add_dir(direction);
            distance += 1;
        }

        println!("{} {:?}", distance, pos);

        path.push((turn.1, distance));
    }

    println!("{:?}", path);

    String::new()
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
..#####...^..";
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
....#####......";
        let (grid, robot_pos) = parse_grid(input.chars());
        assert_eq!(
            compute_path(grid, robot_pos),
            "A,B,C,B,A,C\nR,8,R,8\nR,4,R,4,R,8\nL,6,L,2\nn\n"
        );
    }
}
