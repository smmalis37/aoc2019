use std::ops::RangeInclusive;

pub fn generator(input: &str) -> RangeInclusive<u32> {
    let mut inputs = input.split('-').map(|x| x.parse().unwrap());
    RangeInclusive::new(inputs.next().unwrap(), inputs.next().unwrap())
}

pub fn part1(range: RangeInclusive<u32>) -> usize {
    range.filter(|&x| is_valid(x, false)).count()
}

pub fn part2(range: RangeInclusive<u32>) -> usize {
    range.filter(|&x| is_valid(x, true)).count()
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

fn to_digits(mut val: u32) -> [u32; 6] {
    let mut output = [0; 6];

    for indexish in 0..6 {
        output[5 - indexish] = val % 10;
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
