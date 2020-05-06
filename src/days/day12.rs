use crate::solver::Solver;
use std::cmp::Ordering;

pub struct Day12 {}

type Num = i64;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Planet {
    x: Num,
    y: Num,
    z: Num,
    vel_x: Num,
    vel_y: Num,
    vel_z: Num,
}

impl Planet {
    fn new(x: Num, y: Num, z: Num, vel_x: Num, vel_y: Num, vel_z: Num) -> Self {
        Self {
            x,
            y,
            z,
            vel_x,
            vel_y,
            vel_z,
        }
    }
}

impl Solver<'_> for Day12 {
    type Generated = Vec<Planet>;
    type Output = Num;

    fn generator(input: &str) -> Self::Generated {
        input
            .lines()
            .map(|l| {
                let mut sections = l.trim_matches(&['<', '>'][..]).split(&[',', '='][..]);
                Planet::new(
                    sections.nth(1).unwrap().parse().unwrap(),
                    sections.nth(1).unwrap().parse().unwrap(),
                    sections.nth(1).unwrap().parse().unwrap(),
                    0,
                    0,
                    0,
                )
            })
            .collect()
    }

    fn part1(mut planets: Self::Generated) -> Self::Output {
        for _ in 0..1000 {
            run_step(&mut planets);
        }

        planets.iter().map(energy).sum()
    }

    fn part2(start_planets: Self::Generated) -> Self::Output {
        let mut planets = start_planets.clone();
        let mut cycles = vec![None; 3];
        let mut steps = 0;

        while cycles.iter().any(|&x| x.is_none()) {
            run_step(&mut planets);
            steps += 1;

            if cycles[0].is_none() && check_axis(&planets, &start_planets, |p| p.x, |p| p.vel_x) {
                cycles[0] = Some(steps);
            }
            if cycles[1].is_none() && check_axis(&planets, &start_planets, |p| p.y, |p| p.vel_y) {
                cycles[1] = Some(steps);
            }
            if cycles[2].is_none() && check_axis(&planets, &start_planets, |p| p.z, |p| p.vel_z) {
                cycles[2] = Some(steps);
            }
        }

        cycles
            .into_iter()
            .map(|x| x.unwrap())
            .fold(1, num::integer::lcm)
    }
}

fn check_axis(
    planets: &[Planet],
    start_planets: &[Planet],
    position: impl Fn(&Planet) -> Num,
    velocity: impl Fn(&Planet) -> Num,
) -> bool {
    planets
        .iter()
        .zip(start_planets)
        .all(|(p, sp)| position(p) == position(sp) && velocity(p) == 0)
}

fn run_step(planets: &mut <Day12 as Solver>::Generated) {
    for i in 0..planets.len() {
        let (left, right) = planets.split_at_mut(i + 1);
        let p1 = &mut left[i];
        for p2 in right {
            gravity_adjust(p1.x, &mut p1.vel_x, p2.x, &mut p2.vel_x);
            gravity_adjust(p1.y, &mut p1.vel_y, p2.y, &mut p2.vel_y);
            gravity_adjust(p1.z, &mut p1.vel_z, p2.z, &mut p2.vel_z);
        }
    }

    for p in planets {
        velocity_adjust(p);
    }
}

fn gravity_adjust(pos1: Num, vel1: &mut Num, pos2: Num, vel2: &mut Num) {
    let factor = match pos1.cmp(&pos2) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    };

    *vel1 += factor;
    *vel2 += -factor;
}

fn velocity_adjust(planet: &mut Planet) {
    planet.x += planet.vel_x;
    planet.y += planet.vel_y;
    planet.z += planet.vel_z;
}

fn energy(p: &Planet) -> Num {
    let pos_energy = p.x.abs() + p.y.abs() + p.z.abs();
    let kin_energy = p.vel_x.abs() + p.vel_y.abs() + p.vel_z.abs();
    pos_energy * kin_energy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d12p1_1() {
        let mut planets = Day12::generator(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        );

        assert_eq!(
            planets,
            vec![
                Planet::new(-1, 0, 2, 0, 0, 0),
                Planet::new(2, -10, -7, 0, 0, 0),
                Planet::new(4, -8, 8, 0, 0, 0),
                Planet::new(3, 5, -1, 0, 0, 0),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(2, -1, 1, 3, -1, -1),
                Planet::new(3, -7, -4, 1, 3, 3),
                Planet::new(1, -7, 5, -3, 1, -3),
                Planet::new(2, 2, 0, -1, -3, 1),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(5, -3, -1, 3, -2, -2),
                Planet::new(1, -2, 2, -2, 5, 6),
                Planet::new(1, -4, -1, 0, 3, -6),
                Planet::new(1, -4, 2, -1, -6, 2),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(5, -6, -1, 0, -3, 0),
                Planet::new(0, 0, 6, -1, 2, 4),
                Planet::new(2, 1, -5, 1, 5, -4),
                Planet::new(1, -8, 2, 0, -4, 0),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(2, -8, 0, -3, -2, 1),
                Planet::new(2, 1, 7, 2, 1, 1),
                Planet::new(2, 3, -6, 0, 2, -1),
                Planet::new(2, -9, 1, 1, -1, -1),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(-1, -9, 2, -3, -1, 2),
                Planet::new(4, 1, 5, 2, 0, -2),
                Planet::new(2, 2, -4, 0, -1, 2),
                Planet::new(3, -7, -1, 1, 2, -2),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(-1, -7, 3, 0, 2, 1),
                Planet::new(3, 0, 0, -1, -1, -5),
                Planet::new(3, -2, 1, 1, -4, 5),
                Planet::new(3, -4, -2, 0, 3, -1),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(2, -2, 1, 3, 5, -2),
                Planet::new(1, -4, -4, -2, -4, -4),
                Planet::new(3, -7, 5, 0, -5, 4),
                Planet::new(2, 0, 0, -1, 4, 2),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(5, 2, -2, 3, 4, -3),
                Planet::new(2, -7, -5, 1, -3, -1),
                Planet::new(0, -9, 6, -3, -2, 1),
                Planet::new(1, 1, 3, -1, 1, 3),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(5, 3, -4, 0, 1, -2),
                Planet::new(2, -9, -3, 0, -2, 2),
                Planet::new(0, -8, 4, 0, 1, -2),
                Planet::new(1, 1, 5, 0, 0, 2),
            ]
        );

        run_step(&mut planets);
        assert_eq!(
            planets,
            vec![
                Planet::new(2, 1, -3, -3, -2, 1),
                Planet::new(1, -8, 0, -1, 1, 3),
                Planet::new(3, -6, 1, 3, 2, -3),
                Planet::new(2, 0, 4, 1, -1, -1),
            ]
        );

        assert_eq!(planets.iter().map(energy).sum::<Num>(), 179);
    }

    #[test]
    fn d12p1_2() {
        let mut planets = Day12::generator(
            "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        );

        assert_eq!(
            planets,
            vec![
                Planet::new(-8, -10, 0, 0, 0, 0),
                Planet::new(5, 5, 10, 0, 0, 0),
                Planet::new(2, -7, 3, 0, 0, 0),
                Planet::new(9, -8, -3, 0, 0, 0),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(-9, -10, 1, -2, -2, -1),
                Planet::new(4, 10, 9, -3, 7, -2),
                Planet::new(8, -10, -3, 5, -1, -2),
                Planet::new(5, -10, 3, 0, -4, 5),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(-10, 3, -4, -5, 2, 0),
                Planet::new(5, -25, 6, 1, 1, -4),
                Planet::new(13, 1, 1, 5, -2, 2),
                Planet::new(0, 1, 7, -1, -1, 2),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(15, -6, -9, -5, 4, 0),
                Planet::new(-4, -11, 3, -3, -10, 0),
                Planet::new(0, -1, 11, 7, 4, 3),
                Planet::new(-3, -2, 5, 1, 2, -3),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(14, -12, -4, 11, 3, 0),
                Planet::new(-1, 18, 8, -5, 2, 3),
                Planet::new(-5, -14, 8, 1, -2, 0),
                Planet::new(0, -12, -2, -7, -3, -3),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(-23, 4, 1, -7, -1, 2),
                Planet::new(20, -31, 13, 5, 3, 4),
                Planet::new(-4, 6, 1, -1, 1, -3),
                Planet::new(15, 1, -5, 3, -3, -3),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(36, -10, 6, 5, 0, 3),
                Planet::new(-18, 10, 9, -3, -7, 5),
                Planet::new(8, -12, -3, -2, 1, -7),
                Planet::new(-18, -8, -2, 0, 6, -1),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(-33, -6, 5, -5, -4, 7),
                Planet::new(13, -9, 2, -2, 11, 3),
                Planet::new(11, -8, 2, 8, -6, -7),
                Planet::new(17, 3, 1, -1, -1, -3),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(30, -8, 3, 3, 3, 0),
                Planet::new(-2, -4, 0, 4, -13, 2),
                Planet::new(-18, -7, 15, -8, 2, -2),
                Planet::new(-2, -1, -8, 1, 8, 0),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(-25, -1, 4, 1, -3, 4),
                Planet::new(2, -9, 0, -3, 13, -1),
                Planet::new(32, -8, 14, 5, -4, 6),
                Planet::new(-1, -2, -8, -3, -6, -9),
            ]
        );

        for _ in 0..10 {
            run_step(&mut planets);
        }
        assert_eq!(
            planets,
            vec![
                Planet::new(8, -12, -9, -7, 3, 0),
                Planet::new(13, 16, -3, 3, -11, -5),
                Planet::new(-29, -11, -1, -3, 7, 4),
                Planet::new(16, -13, 23, 7, 1, 1),
            ]
        );

        assert_eq!(planets.iter().map(energy).sum::<Num>(), 1940);
    }

    #[test]
    fn d12p2() {
        let planets = Day12::generator(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        );

        assert_eq!(Day12::part2(planets), 2772);

        let planets = Day12::generator(
            "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        );

        assert_eq!(Day12::part2(planets), 4_686_774_924);
    }
}
