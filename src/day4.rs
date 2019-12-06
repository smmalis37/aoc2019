use std::ops::RangeInclusive;

// Not using the generator macro so we can consume the output.
fn generator(input: &str) -> RangeInclusive<u32> {
    let mut inputs = input.split('-').map(|x| x.parse().unwrap());
    RangeInclusive::new(inputs.next().unwrap(), inputs.next().unwrap())
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    generator(input).filter(|&x| is_valid(x, false)).count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    generator(input).filter(|&x| is_valid(x, true)).count()
}

#[inline(always)]
fn is_valid(val: u32, part2: bool) -> bool {
    let digits = to_digits(val);
    let mut seen_valid_pair = false;
    let mut no_descent = true;

    for i in 0..digits.len() - 1 {
        if digits[i + 1] < digits[i] {
            no_descent = false;
        }

        if digits[i + 1] == digits[i] {
            if part2
                && ((i + 2 < digits.len() && digits[i + 2] == digits[i])
                    || (i > 0 && digits[i - 1] == digits[i]))
            {
            } else {
                seen_valid_pair = true;
            }
        }
    }

    seen_valid_pair && no_descent
}

fn to_digits(mut val: u32) -> Vec<u32> {
    let mut output = Vec::with_capacity(6);

    loop {
        output.push(val % 10);
        val /= 10;

        if val == 0 {
            output.reverse();
            return output;
        }
    }
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
