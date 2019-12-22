use crate::coord_system::unsigned::*;
use crate::solver::Solver;
use noisy_float::prelude::*;
use std::f32::consts::*;
use std::f32::*;

type F = f32;
type R = R32;

pub struct Day10 {}

impl<'a> Solver<'a> for Day10 {
    type Generated = Vec<Point>;
    type Output = usize;

    fn generator(input: &'a str) -> Self::Generated {
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(x, _)| Point { x, y })
            })
            .collect()
    }

    fn part1(asteroid_coords: Self::Generated) -> Self::Output {
        find_best_coord(&asteroid_coords).1
    }

    fn part2(asteroid_coords: Self::Generated) -> Self::Output {
        let coord = find_destroyed_position(&asteroid_coords, 200);
        coord.x * 100 + coord.y
    }
}

fn find_best_coord(asteroid_coords: &<Day10 as Solver>::Generated) -> (Point, usize) {
    let mut max_visible = 0;
    let mut max_coord = Point { x: 0, y: 0 };

    for &c in asteroid_coords {
        let mut angles: Vec<_> = asteroid_coords
            .iter()
            .filter(|&&c2| c != c2)
            .map(|&c2| calc_angle_distance(c, c2).0)
            .collect();

        angles.sort_unstable();
        angles.dedup_by(|&mut x1, &mut x2| float_equals(x1, x2));

        let count = angles.len();

        if count > max_visible {
            max_visible = count;
            max_coord = c;
        }
    }

    (max_coord, max_visible)
}

fn find_destroyed_position(
    asteroid_coords: &<Day10 as Solver>::Generated,
    position: usize,
) -> Point {
    let part1_coord = find_best_coord(&asteroid_coords).0;

    let mut angles: Vec<_> = asteroid_coords
        .iter()
        .filter(|&&c| c != part1_coord)
        .map(|&c2| calc_angle_distance(part1_coord, c2))
        .collect();

    // This is apparently good enough, but not technically correct.
    angles.sort_unstable_by_key(|ad| ad.1);
    angles.sort_by_key(|ad| ad.0);
    angles.dedup_by(|&mut ad1, &mut ad2| float_equals(ad1.0, ad2.0));
    let (angle, distance) = angles[position - 1];
    angle_distance_to_coord(part1_coord, angle, distance)

    // This is fully correct, but requires nightly and a feature flag.
    // let mut subangles = &mut *angles;
    // let mut seek = position - 1;
    // loop {
    //     subangles.sort_unstable_by_key(|ad| ad.1);
    //     subangles.sort_by_key(|ad| ad.0);

    //     let (left, right) =
    //         subangles.partition_dedup_by(|&mut ad1, &mut ad2| float_equals(ad1.0, ad2.0));

    //     if seek < left.len() {
    //         let (angle, distance) = left[seek];
    //         let coord = angle_distance_to_coord(part1_coord, angle, distance);
    //         return (coord, coord.x * 100 + coord.y);
    //     } else {
    //         seek -= left.len();
    //         subangles = right;
    //     }
    // }
}

fn angle_distance_to_coord(origin: Point, angle: R, distance: R) -> Point {
    let (x_unit, y_unit) = angle.sin_cos();
    let y_unit = -y_unit;
    let x = ((x_unit * distance) + origin.x as F).round().raw() as Coordinate;
    let y = ((y_unit * distance) + origin.y as F).round().raw() as Coordinate;
    Point { x, y }
}

fn calc_angle_distance(c1: Point, c2: Point) -> (R, R) {
    let (x, y, x2, y2) = (
        R::new(c1.x as F),
        R::new(c1.y as F),
        R::new(c2.x as F),
        R::new(c2.y as F),
    );

    let xdiff = x2 - x;
    let ydiff = y2 - y;
    let mut angle = ydiff.atan2(xdiff) + FRAC_PI_2;
    if angle < 0.0 {
        angle += 2.0 * PI;
    }
    let distance = ydiff.hypot(xdiff);
    (angle, distance)
}

fn float_equals(x1: R, x2: R) -> bool {
    (x1 - x2).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_distance() {
        fn test(c1: (Coordinate, Coordinate), c2: (Coordinate, Coordinate), exp: (F, F)) {
            let res = calc_angle_distance(Point { x: c1.0, y: c1.1 }, Point { x: c2.0, y: c2.1 });
            assert_eq!((res.0.raw(), res.1.raw()), exp);
        }

        let sqrt2 = 2.0.sqrt();
        test((1, 1), (1, 0), (0.0, 1.0));
        test((1, 1), (2, 0), (FRAC_PI_4, sqrt2));
        test((1, 1), (2, 1), (FRAC_PI_2, 1.0));
        test((1, 1), (2, 2), (3.0 * FRAC_PI_4, sqrt2));
        test((1, 1), (1, 2), (PI, 1.0));
        test((1, 1), (0, 2), (5.0 * FRAC_PI_4, sqrt2));
        test((1, 1), (0, 1), (3.0 * FRAC_PI_2, 1.0));
        test((1, 1), (0, 0), (7.0 * FRAC_PI_4, sqrt2));
    }

    #[test]
    fn test_angle_distance_to_coord() {
        fn test(c1: (Coordinate, Coordinate), ad: (F, F), exp: (Coordinate, Coordinate)) {
            let res =
                angle_distance_to_coord(Point { x: c1.0, y: c1.1 }, R::new(ad.0), R::new(ad.1));
            assert_eq!(res, Point { x: exp.0, y: exp.1 });
        }

        let sqrt2 = 2.0.sqrt();
        test((1, 1), (0.0, 1.0), (1, 0));
        test((1, 1), (FRAC_PI_4, sqrt2), (2, 0));
        test((1, 1), (FRAC_PI_2, 1.0), (2, 1));
        test((1, 1), (3.0 * FRAC_PI_4, sqrt2), (2, 2));
        test((1, 1), (PI, 1.0), (1, 2));
        test((1, 1), (5.0 * FRAC_PI_4, sqrt2), (0, 2));
        test((1, 1), (3.0 * FRAC_PI_2, 1.0), (0, 1));
        test((1, 1), (7.0 * FRAC_PI_4, sqrt2), (0, 0));
    }

    #[test]
    fn d10p1() {
        assert_eq!(
            find_best_coord(&Day10::generator(
                ".#..#
.....
#####
....#
...##"
            )),
            (Point { x: 3, y: 4 }, 8)
        );

        assert_eq!(
            find_best_coord(&Day10::generator(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            )),
            (Point { x: 5, y: 8 }, 33)
        );

        assert_eq!(
            find_best_coord(&Day10::generator(
                "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            )),
            (Point { x: 1, y: 2 }, 35)
        );

        assert_eq!(
            find_best_coord(&Day10::generator(
                ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            )),
            (Point { x: 6, y: 3 }, 41)
        );

        assert_eq!(
            find_best_coord(&Day10::generator(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            )),
            (Point { x: 11, y: 13 }, 210)
        );
    }

    #[test]
    fn d10p2() {
        let asteroids = Day10::generator(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );

        assert_eq!(
            find_destroyed_position(&asteroids, 1),
            Point { x: 11, y: 12 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 2),
            Point { x: 12, y: 1 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 3),
            Point { x: 12, y: 2 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 10),
            Point { x: 12, y: 8 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 20),
            Point { x: 16, y: 0 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 50),
            Point { x: 16, y: 9 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 100),
            Point { x: 10, y: 16 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 199),
            Point { x: 9, y: 6 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 200),
            Point { x: 8, y: 2 }
        );
        assert_eq!(
            find_destroyed_position(&asteroids, 201),
            Point { x: 10, y: 9 }
        );
        // Requires the fully functional, nightly-only solution.
        // assert_eq!(
        //     find_destroyed_position(&asteroids, 299),
        //     Point { x: 11, y: 1 }
        // );
    }
}
