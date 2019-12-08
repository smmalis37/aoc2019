use crate::solver::Solver;

pub struct Day1 {}

type Mass = u32;

impl<'a> Solver<'a> for Day1 {
    type Generated = Vec<Mass>;
    type Output = Mass;

    fn generator(input: &'a str) -> Self::Generated {
        input.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn part1(masses: Self::Generated) -> Self::Output {
        masses.iter().map(|&x| calculate_fuel(x)).sum()
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

    #[test]
    fn d1p1() {
        assert_eq!(Day1::part1(vec![12]), 2);
        assert_eq!(Day1::part1(vec![14]), 2);
        assert_eq!(Day1::part1(vec![1969]), 654);
        assert_eq!(Day1::part1(vec![100_756]), 33583);
    }

    #[test]
    fn d1p2() {
        assert_eq!(Day1::part2(vec![14]), 2);
        assert_eq!(Day1::part2(vec![1969]), 966);
        assert_eq!(Day1::part2(vec![100_756]), 50346);
    }
}
