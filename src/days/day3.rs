use crate::coord_system::direction::*;
use crate::coord_system::signed::*;
use crate::solver::Solver;
use arrayvec::ArrayVec;
use std::collections::HashMap;

type Distance = u32;

#[derive(Copy, Clone)]
struct PathSegment {
    distance: Distance,
    direction: Direction,
}

pub struct Day3 {}

impl<'a> Solver<'a> for Day3 {
    type Generated = [HashMap<Point, Distance>; 2];
    type Output = Distance;

    fn generator(input: &'a str) -> Self::Generated {
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
                .collect(),
        )
    }

    fn part1(paths: Self::Generated) -> Self::Output {
        hashmap_intersection(&paths[0], &paths[1])
            .map(|c| (c.x.abs() + c.y.abs()) as Distance)
            .min()
            .unwrap()
    }

    fn part2(paths: Self::Generated) -> Self::Output {
        hashmap_intersection(&paths[0], &paths[1])
            .map(|c| paths[0][c] + paths[1][c])
            .min()
            .unwrap()
    }
}

fn trace_wires(paths: ArrayVec<[Vec<PathSegment>; 2]>) -> [HashMap<Point, Distance>; 2] {
    let mut touched_coords_steps: [HashMap<Point, Distance>; 2] =
        [make_hashmap(&paths[0]), make_hashmap(&paths[1])];

    for path_index in 0..2 {
        let mut position = Point { x: 0, y: 0 };
        let mut steps = 0;
        let coords_steps = &mut touched_coords_steps[path_index];
        for segment in &paths[path_index] {
            for _ in 0..segment.distance {
                position = position.add_dir(segment.direction);
                steps += 1;
                coords_steps.entry(position).or_insert(steps);
            }
        }
    }

    touched_coords_steps
}

fn make_hashmap(paths: &[PathSegment]) -> HashMap<Point, Distance> {
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
            Day3::part1(Day3::generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )),
            159
        );
        assert_eq!(
            Day3::part1(Day3::generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            135
        );
    }

    #[test]
    fn d3p2() {
        assert_eq!(
            Day3::part2(Day3::generator(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
            )),
            610
        );
        assert_eq!(
            Day3::part2(Day3::generator(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )),
            410
        );
    }
}
