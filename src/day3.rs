use stackvec::prelude::*;
use std::collections::*;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        })
    }
}

impl Direction {
    fn to_unit(self) -> Coordinate {
        match self {
            Direction::Up => Coordinate { x: 0, y: 1 },
            Direction::Down => Coordinate { x: 0, y: -1 },
            Direction::Left => Coordinate { x: -1, y: 0 },
            Direction::Right => Coordinate { x: 1, y: 0 },
        }
    }
}

#[derive(Copy, Clone)]
struct PathSegment {
    distance: u32,
    direction: Direction,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coordinate {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

pub fn generator(input: &str) -> [HashMap<Coordinate, u32>; 2] {
    trace_wires(
        input
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|x| PathSegment {
                        direction: x[..1].parse().unwrap(),
                        distance: x[1..].parse().unwrap(),
                    })
                    .collect()
            })
            .try_collect::<[Vec<_>; 2]>()
            .unwrap(),
    )
}

pub fn part1(paths: [HashMap<Coordinate, u32>; 2]) -> i32 {
    hashmap_intersection(&paths[0], &paths[1])
        .map(|c| c.x.abs() + c.y.abs())
        .min()
        .unwrap()
}

pub fn part2(paths: [HashMap<Coordinate, u32>; 2]) -> u32 {
    hashmap_intersection(&paths[0], &paths[1])
        .map(|c| paths[0][c] + paths[1][c])
        .min()
        .unwrap()
}

fn trace_wires(paths: [Vec<PathSegment>; 2]) -> [HashMap<Coordinate, u32>; 2] {
    let mut touched_coords_steps: [HashMap<Coordinate, u32>; 2] =
        [make_hashmap(&paths[0]), make_hashmap(&paths[1])];

    for path_index in 0..2 {
        let mut position = Coordinate { x: 0, y: 0 };
        let mut steps = 0;
        let coords_steps = &mut touched_coords_steps[path_index];
        for segment in &paths[path_index] {
            let unit = segment.direction.to_unit();
            for _ in 0..segment.distance {
                position += unit;
                steps += 1;
                coords_steps.entry(position).or_insert(steps);
            }
        }
    }

    touched_coords_steps
}

fn make_hashmap(paths: &[PathSegment]) -> HashMap<Coordinate, u32> {
    HashMap::with_capacity(paths.iter().map(|x| x.distance as usize).sum())
}

fn hashmap_intersection<'a, K: Eq + std::hash::Hash, V>(
    h1: &'a HashMap<K, V>,
    h2: &'a HashMap<K, V>,
) -> impl Iterator<Item = &'a K> {
    h1.keys().filter(move |x| h2.contains_key(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3p1() {
        assert_eq!(
            part1(generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        );
        assert_eq!(
            part1(generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        );
    }

    #[test]
    fn d3p2() {
        assert_eq!(
            part2(generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
        assert_eq!(
            part2(generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );
    }
}
