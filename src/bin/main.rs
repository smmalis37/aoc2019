use aoc2019::days::*;
use aoc2019::solver::Solver;
use std::time::Instant;

macro_rules! days {
    ( $( $d:expr ),* ) => {
        $(
            paste::expr! {
                run::<[<day $d>]::[<Day $d>]>($d, include_str!(concat!("..\\..\\input\\2019\\day", $d, ".txt")));
            }
        )*
    };
}

fn main() {
    println!("AOC 2019");
    days!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17);
}

fn run<'a, S: Solver<'a>>(day_number: u8, input: &'a str) {
    let trimmed_input = input.trim();

    let start_time = Instant::now();
    let generated = S::generator(trimmed_input);
    let final_time = Instant::now();

    println!(
        "\nDay {}:\n\tgenerator : {:?}",
        day_number,
        (final_time - start_time)
    );

    run_half(generated.clone(), 1, S::part1);
    run_half(generated, 2, S::part2);
}

fn run_half<T, O: std::fmt::Debug>(input: T, part_number: u8, part: impl Fn(T) -> O) {
    print!("Part {}: ", part_number);

    let start_time = Instant::now();
    let result = part(input);
    let final_time = Instant::now();

    println!("{:?}\n\trunner: {:?}", result, (final_time - start_time));
}
