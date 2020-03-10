use crate::coord_system::direction::*;
use crate::coord_system::signed::*;
use crate::solver::Solver;
use std::collections::HashMap;

type Distance = u32;

#[derive(Copy, Clone)]
struct PathSegment {
    distance: Distance,
    direction: Direction,
}

pub struct Day3 {}

impl Solver<'_> for Day3 {
    type Generated = Vec<(Point, Distance)>;
    type Output = Distance;

    fn generator(input: &str) -> Self::Generated {
        let paths = input
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|x| PathSegment {
                        direction: x[..1].parse().unwrap(),
                        distance: x[1..].parse().unwrap(),
                    })
                    .collect()
            })
            .collect();
        find_intersections(paths)
    }

    fn part1(intersections: Self::Generated) -> Self::Output {
        intersections
            .into_iter()
            .map(|(p, _)| (p.x.abs() + p.y.abs()) as Distance)
            .min()
            .unwrap()
    }

    fn part2(intersections: Self::Generated) -> Self::Output {
        intersections.into_iter().min_by_key(|&(_, s)| s).unwrap().1
    }
}

fn find_intersections<'a>(paths: Vec<Vec<PathSegment>>) -> <Day3 as Solver<'a>>::Generated {
    assert_eq!(paths.len(), 2);
    let mut coords_steps =
        HashMap::with_capacity(paths[0].iter().map(|x| x.distance as usize).sum());
    trace_wire(&paths[0], |position, steps| {
        coords_steps.entry(position).or_insert(steps);
    });

    let mut intersections = Vec::new();
    trace_wire(&paths[1], |position, steps| {
        if coords_steps.contains_key(&position) {
            intersections.push((position, steps + coords_steps[&position]));
        }
    });

    intersections
}

fn trace_wire(path: &[PathSegment], mut step_action: impl FnMut(Point, Distance)) {
    let mut position = Point { x: 0, y: 0 };
    let mut steps = 0;

    for segment in path {
        for _ in 0..segment.distance {
            position = position.add_dir(segment.direction);
            steps += 1;
            step_action(position, steps);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3p1() {
        assert_eq!(
            Day3::part1(Day3::generator(
                "R8,U5,L5,D3
U7,R6,D4,L4"
            )),
            6
        );
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
                "R8,U5,L5,D3
U7,R6,D4,L4"
            )),
            30
        );
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
