use crate::solver::Solver;

pub struct Day16 {}

type N = i32;

impl<'a> Solver<'a> for Day16 {
    type Generated = Vec<N>;
    type Output = N;

    fn generator(input: &'a str) -> Self::Generated {
        input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as N)
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

        data = std::iter::repeat(data.iter())
            .take(10000)
            .flatten()
            .skip(offset)
            .copied()
            .collect();

        for _ in 0..100 {
            for j in (0..data.len() - 1).rev() {
                data[j] = (data[j] + data[j + 1]) % 10;
            }
        }

        to_number(&data[..8])
    }
}

fn run_phase<'a>(input: <Day16 as Solver>::Generated) -> <Day16 as Solver<'a>>::Generated {
    let pattern = [0, 1, 0, -1];

    (0..input.len())
        .map(|i| {
            (input
                .iter()
                .skip(i)
                .zip(
                    pattern
                        .iter()
                        .flat_map(|&x| std::iter::repeat(x).take(i + 1))
                        .cycle()
                        .skip(1)
                        .skip(i),
                )
                .map(|(x, y)| x * y)
                .sum::<i32>()
                % 10)
                .abs()
        })
        .collect()
}

fn to_number(x: &[N]) -> N {
    x.iter().fold(0, |b, v| b * 10 + v)
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
