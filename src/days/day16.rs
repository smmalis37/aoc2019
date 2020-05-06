use crate::solver::Solver;

pub struct Day16 {}

type Num = u32;

impl Solver<'_> for Day16 {
    type Generated = Vec<Num>;
    type Output = Num;

    fn generator(input: &str) -> Self::Generated {
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as Num)
            .collect()
    }

    fn part1(mut data: Self::Generated) -> Self::Output {
        for _ in 0..100 {
            data = run_phase(data);
        }

        to_number(&data[0..8])
    }

    fn part2(mut data: Self::Generated) -> Self::Output {
        let offset = to_number(&data[0..7]) as usize;
        assert!(offset > data.len() * 10000 / 2);

        let new_len = 10000 * data.len() - offset;
        data = data.into_iter().rev().cycle().take(new_len).collect();

        for _ in 0..100 {
            for j in 1..data.len() {
                data[j] = (data[j] + data[j - 1]) % 10;
            }
        }

        to_number(data.iter().rev().take(8))
    }
}

fn run_phase<'a>(input: <Day16 as Solver>::Generated) -> <Day16 as Solver<'a>>::Generated {
    let mut output = Vec::with_capacity(input.len());
    let input = ISizeVec(input);
    let input_len = input.len();

    for output_index in 0..input_len {
        let mut value: i32 = 0;
        let mut index = -1;
        let pattern_length = output_index + 1;

        while index < input_len {
            use std::cmp::min;
            // 0
            index = min(index + pattern_length, input_len);

            // 1
            let end_index = min(index + pattern_length, input_len);
            value += input[index..end_index].iter().sum::<Num>() as i32;
            index = min(index + pattern_length, input_len);

            // 0
            index = min(index + pattern_length, input_len);

            // -1
            let end_index = min(index + pattern_length, input_len);
            value -= input[index..end_index].iter().sum::<Num>() as i32;
            index = min(index + pattern_length, input_len);
        }

        output.push((value.abs() % 10) as Num);
    }

    output
}

struct ISizeVec<T>(Vec<T>);

impl<T> std::ops::Index<std::ops::Range<isize>> for ISizeVec<T> {
    type Output = [T];
    fn index(&self, index: std::ops::Range<isize>) -> &Self::Output {
        self.0.index(index.start as usize..index.end as usize)
    }
}

impl<T> ISizeVec<T> {
    fn len(&self) -> isize {
        self.0.len() as isize
    }
}

fn to_number<'a>(x: impl IntoIterator<Item = &'a Num>) -> Num {
    x.into_iter().fold(0, |b, v| b * 10 + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d16p1() {
        let mut data = Day16::generator("12345678");
        assert_eq!(data, &[1, 2, 3, 4, 5, 6, 7, 8]);
        data = run_phase(data);
        assert_eq!(data, &[4, 8, 2, 2, 6, 1, 5, 8]);
        data = run_phase(data);
        assert_eq!(data, &[3, 4, 0, 4, 0, 4, 3, 8]);
        data = run_phase(data);
        assert_eq!(data, &[0, 3, 4, 1, 5, 5, 1, 8]);
        data = run_phase(data);
        assert_eq!(data, &[0, 1, 0, 2, 9, 4, 9, 8]);

        data = Day16::generator("80871224585914546619083218645595");
        assert_eq!(Day16::part1(data), 24_176_176);

        data = Day16::generator("19617804207202209144916044189917");
        assert_eq!(Day16::part1(data), 73_745_418);

        data = Day16::generator("69317163492948606335995924319873");
        assert_eq!(Day16::part1(data), 52_432_133);
    }

    #[test]
    fn d16p2() {
        assert_eq!(
            Day16::part2(Day16::generator("03036732577212944063491565474664")),
            84_462_026
        );
        assert_eq!(
            Day16::part2(Day16::generator("02935109699940807407585447034323")),
            78_725_270
        );
        assert_eq!(
            Day16::part2(Day16::generator("03081770884921959731165446850517")),
            53_553_731
        );
    }
}
