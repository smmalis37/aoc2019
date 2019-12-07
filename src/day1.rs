pub fn generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub fn part1(masses: Vec<u32>) -> u32 {
    masses.iter().map(|&x| calculate_fuel(x)).sum()
}

pub fn part2(masses: Vec<u32>) -> u32 {
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

fn calculate_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1() {
        assert_eq!(part1(vec![12]), 2);
        assert_eq!(part1(vec![14]), 2);
        assert_eq!(part1(vec![1969]), 654);
        assert_eq!(part1(vec![100_756]), 33583);
    }

    #[test]
    fn d1p2() {
        assert_eq!(part2(vec![14]), 2);
        assert_eq!(part2(vec![1969]), 966);
        assert_eq!(part2(vec![100_756]), 50346);
    }
}
