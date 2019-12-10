use crate::solver::Solver;
use noisy_float::prelude::*;
use std::f32::consts::*;
use std::f32::*;

pub struct Day10 {}

impl<'a> Solver<'a> for Day10 {
    type Generated = Vec<(usize, usize)>;
    type Output = ((usize, usize), usize);

    fn generator(input: &'a str) -> Self::Generated {
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter(|&(_, c)| c == '#')
                    .map(move |(x, _)| (x, y))
            })
            .collect()
    }

    fn part1(asteroid_coords: Self::Generated) -> Self::Output {
        let mut max_visible = 0;
        let mut max_coord = (0, 0);

        for &c in &asteroid_coords {
            let mut angles: Vec<_> = asteroid_coords
                .iter()
                .filter(|&&c2| c != c2)
                .map(|&c2| calc_angle_distance(c, c2).0)
                .collect();

            angles.sort_unstable();
            angles.dedup_by(|&mut a1, &mut a2| (a1 - a2).abs() < EPSILON);

            let count = angles.len();

            if count > max_visible {
                max_visible = count;
                max_coord = c;
            }
        }

        (max_coord, max_visible)
    }

    fn part2(asteroid_coords: Self::Generated) -> Self::Output {
        let part1_coord = (11, 13);

        let mut angles: Vec<_> = asteroid_coords
            .into_iter()
            .filter(|&c| c != part1_coord)
            .map(|c2| calc_angle_distance(part1_coord, c2))
            .collect();

        let mut subangles = &mut *angles;
        let mut seek = 199;
        loop {
            subangles.sort_unstable_by_key(|ad| ad.1);
            subangles.sort_by_key(|ad| ad.0);
            let (left, right) =
                subangles.partition_dedup_by(|&mut ad1, &mut ad2| (ad1.0 - ad2.0).abs() < EPSILON);

            if seek < left.len() {
                let (angle, distance) = left[seek];
                let coord = angle_distance_to_coord(part1_coord, angle, distance);
                return (coord, coord.0 * 100 + coord.1);
            } else {
                seek -= left.len();
                subangles = right;
            }
        }
    }
}

fn angle_distance_to_coord(origin: (usize, usize), angle: R32, distance: R32) -> (usize, usize) {
    let (x_unit, y_unit) = angle.sin_cos();
    let y_unit = y_unit * -1.0;
    let x = ((x_unit * distance) + origin.0 as f32).round().raw() as usize;
    let y = ((y_unit * distance) + origin.1 as f32).round().raw() as usize;
    (x, y)
}

fn calc_angle_distance(c1: (usize, usize), c2: (usize, usize)) -> (R32, R32) {
    let (x, y, x2, y2) = (
        R32::new(c1.0 as f32),
        R32::new(c1.1 as f32),
        R32::new(c2.0 as f32),
        R32::new(c2.1 as f32),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_distance() {
        fn test(c1: (usize, usize), c2: (usize, usize), exp: (f32, f32)) {
            let res = calc_angle_distance(c1, c2);
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
        fn test(c1: (usize, usize), ad: (f32, f32), exp: (usize, usize)) {
            let res = angle_distance_to_coord(c1, R32::new(ad.0), R32::new(ad.1));
            assert_eq!(res, exp);
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
            Day10::part1(Day10::generator(
                ".#..#
.....
#####
....#
...##"
            )),
            ((3, 4), 8)
        );

        assert_eq!(
            Day10::part1(Day10::generator(
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
            ((5, 8), 33)
        );

        assert_eq!(
            Day10::part1(Day10::generator(
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
            ((1, 2), 35)
        );

        assert_eq!(
            Day10::part1(Day10::generator(
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
            ((6, 3), 41)
        );

        assert_eq!(
            Day10::part1(Day10::generator(
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
            ((11, 13), 210)
        );
    }

    #[test]
    fn d10p2() {
        assert_eq!(
            Day10::part2(Day10::generator(
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
            ((8, 2), 802)
        );
    }
}
