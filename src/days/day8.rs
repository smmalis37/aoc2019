use crate::solver::Solver;

type Pixel = u8;
type Layer = Vec<Vec<Pixel>>;
type Image = Vec<Layer>;

pub struct Day8 {}

impl Solver<'_> for Day8 {
    type Generated = Image;
    type Output = usize;

    fn generator(input: &str) -> Self::Generated {
        parse_image(input, 25, 6)
    }

    fn part1(image: Self::Generated) -> Self::Output {
        let layer = image
            .iter()
            .min_by_key(|layer| layer.iter().flatten().filter(|&&x| x == 0).count())
            .unwrap();

        let one_count = layer.iter().flatten().filter(|&&x| x == 1).count();
        let two_count = layer.iter().flatten().filter(|&&x| x == 2).count();

        one_count * two_count
    }

    fn part2(image: Self::Generated) -> Self::Output {
        let height = image[0].len();
        let width = image[0][0].len();

        use std::io::Write;
        let stdout = std::io::stdout();
        let mut writer = stdout.lock();
        writeln!(writer).unwrap();

        for h in 0..height {
            for w in 0..width {
                for layer in &image {
                    let pixel = layer[h][w];
                    if pixel != 2 {
                        write!(
                            writer,
                            "{}",
                            match pixel {
                                0 => ' ',
                                1 => '█',
                                _ => unreachable!(),
                            }
                        )
                        .unwrap();
                        break;
                    }
                }
            }
            writeln!(writer).unwrap();
        }

        0
    }
}

fn parse_image(input: &str, width: usize, height: usize) -> Image {
    let layer_count = input.len() / width / height;
    let mut image = Image::with_capacity(layer_count);
    let mut digits = input.bytes().map(|x| x - b'0');

    for _ in 0..layer_count {
        let mut layer = Layer::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(digits.next().unwrap());
            }
            layer.push(row);
        }
        image.push(layer);
    }

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d8p1() {
        assert_eq!(
            parse_image("123456789012", 3, 2),
            &[&[&[1, 2, 3], &[4, 5, 6]], &[&[7, 8, 9], &[0, 1, 2]]]
        );
    }
}
