use crate::solver::Solver;
use noisy_float::prelude::*;

pub struct Day10 {}

impl<'a> Solver<'a> for Day10 {
    type Generated = Vec<Vec<bool>>;
    type Output = ((usize, usize), usize);

    fn generator(input: &'a str) -> Self::Generated {
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|x| match x {
                        '.' => false,
                        '#' => true,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(map: Self::Generated) -> Self::Output {
        let height = map.len();
        let width = map[0].len();
        let size = height * width;

        let asteroid_coords: Vec<_> = (0..size)
            .map(|c| decode_coord(c, width))
            .filter(|c| map[c.0][c.1])
            .collect();

        let mut max_visible = 0;
        let mut max_coord = (0, 0);

        for &c in &asteroid_coords {
            let mut angles = asteroid_coords
                .iter()
                .filter(|&&c2| c != c2)
                .map(|&c2| calc_angle_distance(c, c2).0)
                .collect::<Vec<_>>();

            angles.sort_unstable();
            angles.dedup_by(|&mut a1, &mut a2| (a1 - a2).abs() < std::f32::EPSILON);

            let count = angles.len();

            if count > max_visible {
                max_visible = count;
                max_coord = c;
            }
        }

        (max_coord, max_visible)
    }

    fn part2(map: Self::Generated) -> Self::Output {
        ((0, 0), 0)
    }
}

fn decode_coord(pos: usize, width: usize) -> (usize, usize) {
    (pos / width, pos % width)
}

fn calc_angle_distance(c1: (usize, usize), c2: (usize, usize)) -> (R32, R32) {
    let (y, x, y2, x2) = (
        R32::new(c1.0 as f32),
        R32::new(c1.1 as f32),
        R32::new(c2.0 as f32),
        R32::new(c2.1 as f32),
    );
    let ydiff = y2 - y;
    let xdiff = x2 - x;
    let angle = ydiff.atan2(xdiff) + std::f32::consts::FRAC_PI_2;
    let distance = (xdiff * xdiff + ydiff * ydiff).sqrt();
    (angle, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            ((4, 3), 8)
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
            ))
            .1,
            33
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
            ))
            .1,
            35
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
            ))
            .1,
            41
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
            ))
            .1,
            210
        );
    }

    #[test]
    fn d10p2() {}
}
