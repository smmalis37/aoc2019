use crate::solver::Solver;

pub struct Day1 {}

type Mass = u32;

impl Solver<'_> for Day1 {
    type Generated = Vec<Mass>;
    type Output = Mass;

    fn generator(input: &str) -> Self::Generated {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn part1(masses: Self::Generated) -> Self::Output {
        masses.into_iter().map(calculate_fuel).sum()
    }

    fn part2(masses: Self::Generated) -> Self::Output {
        let mut total = 0;

        for mass in masses {
            let mut fuel = calculate_fuel(mass);

            while fuel > 0 {
                total += fuel;
                fuel = calculate_fuel(fuel);
            }
        }

        total
    }
}

fn calculate_fuel(mass: Mass) -> Mass {
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test<'a>(
        part: impl Fn(<Day1 as Solver<'a>>::Generated) -> <Day1 as Solver<'a>>::Output,
        input: Mass,
        expected_output: <Day1 as Solver>::Output,
    ) {
        assert_eq!(part(vec![input]), expected_output);
    }

    #[test]
    fn d1p1() {
        let test_part1 = |x, y| test(Day1::part1, x, y);
        test_part1(12, 2);
        test_part1(14, 2);
        test_part1(1969, 654);
        test_part1(100_756, 33583);
    }

    #[test]
    fn d1p2() {
        let test_part2 = |x, y| test(Day1::part2, x, y);
        test_part2(14, 2);
        test_part2(1969, 966);
        test_part2(100_756, 50346);
    }
}
