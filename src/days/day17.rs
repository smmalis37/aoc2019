use crate::helpers::coord_system::{Direction, Direction::*};
use crate::helpers::intcode::*;
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
    type Output = usize;

    fn generator(input: &'a str) -> Self::Generated {
        input.parse().unwrap()
    }

    fn part1(intcode: Self::Generated) -> Self::Output {
        let mut outputs = intcode.run_predetermined(&[]);
        outputs.truncate(outputs.len() - 1);

        let grid = parse_grid(outputs.iter().map(|&x| x as u8 as char));
        calculate_alignment(grid)
    }

    fn part2(mut intcode: Self::Generated) -> Self::Output {
        intcode.replace_cell(0, 2);
        let outputs = intcode.run_predetermined(
            &"A,B,B,A,C,A,C,A,C,B\nR,6,R,6,R,8,L,10,L,4\nR,6,L,10,R,8\nL,4,L,12,R,6,L,10\nn\n"
                .bytes()
                .map(|x| x as i64)
                .collect::<Vec<_>>(),
        );

        *outputs.last().unwrap() as usize
    }
}

fn parse_grid(outputs: impl IntoIterator<Item = char>) -> Vec<Vec<Cell>> {
    let mut grid = Vec::new();
    let mut row = Vec::new();

    for c in outputs {
        match c {
            '.' => row.push(Empty),
            '#' => row.push(Scaffold),
            '^' => row.push(Robot(Up)),
            '>' => row.push(Robot(Right)),
            'v' => row.push(Robot(Down)),
            '<' => row.push(Robot(Left)),
            '\n' => {
                grid.push(row);
                row = Vec::new()
            }
            _ => unreachable!(),
        }
    }

    grid
}

fn calculate_alignment<'a>(grid: Vec<Vec<Cell>>) -> <Day17 as Solver<'a>>::Output {
    let mut result = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .all(|&(ymod, xmod)| grid[add(y, ymod)][add(x, xmod)] == Scaffold)
            {
                result += y * x;
            }
        }
    }

    result
}

fn add(a: usize, b: i32) -> usize {
    ((a as i32) + b) as usize
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
        assert_eq!(calculate_alignment(parse_grid(input.chars())), 76);
    }
}
