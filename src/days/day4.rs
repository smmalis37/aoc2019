use crate::solver::Solver;
use std::ops::RangeInclusive;

pub struct Day4 {}

type Num = u32;

impl Solver<'_> for Day4 {
    type Generated = RangeInclusive<Num>;
    type Output = usize;

    fn generator(input: &str) -> Self::Generated {
        let separator = input.find('-').unwrap();
        RangeInclusive::new(
            input[..separator].parse().unwrap(),
            input[separator + 1..].parse().unwrap(),
        )
    }

    fn part1(range: Self::Generated) -> Self::Output {
        range.filter(|&x| is_valid(x, false)).count()
    }

    fn part2(range: Self::Generated) -> Self::Output {
        range.filter(|&x| is_valid(x, true)).count()
    }
}

#[inline(always)]
fn is_valid(val: Num, part2: bool) -> bool {
    let digits = to_digits(val);
    let mut seen_valid_pair = false;
    let mut no_descent = true;

    for i in 1..digits.len() {
        if digits[i - 1] > digits[i] {
            no_descent = false;
        }

        if digits[i - 1] == digits[i] {
            if part2
                && ((i + 1 < digits.len() && digits[i + 1] == digits[i])
                    || (i > 1 && digits[i - 2] == digits[i]))
            {
            } else {
                seen_valid_pair = true;
            }
        }
    }

    seen_valid_pair && no_descent
}

type Digit = u8;

fn to_digits(mut val: Num) -> [Digit; 6] {
    let mut output = [0; 6];

    for indexish in (0..6).rev() {
        output[indexish] = (val % 10) as Digit;
        val /= 10;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d4p1() {
        assert_eq!(is_valid(111_111, false), true);
        assert_eq!(is_valid(223_450, false), false);
        assert_eq!(is_valid(123_789, false), false);
    }

    #[test]
    fn d4p2() {
        assert_eq!(is_valid(112_233, true), true);
        assert_eq!(is_valid(123_444, true), false);
        assert_eq!(is_valid(111_122, true), true);
    }
}
