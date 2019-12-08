use crate::solver::Solver;

pub struct DayNUM {}

impl<'a> Solver<'a> for DayNUM {
    type Generated = u32;
    type Output = u32;

    fn generator(input: &'a str) -> Self::Generated {
        0
    }

    fn part1(data: Self::Generated) -> Self::Output {
        0
    }

    fn part2(data: Self::Generated) -> Self::Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dNUMp1() {}

    #[test]
    fn dNUMp2() {}
}
