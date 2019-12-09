use aoc2019::*;
use std::error::Error;
use std::fmt::Display;
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

fn main() -> Result<(), Box<dyn Error>> {
    println!("AOC 2019");
    days!(1, 2, 3, 4, 5, 6, 7, 8, 9);
    Ok(())
}

fn run<'a, S: solver::Solver<'a>>(day_number: u8, input: &'a str) {
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

fn run_half<T, O: Display>(input: T, part_number: u8, part: impl Fn(T) -> O) {
    let start_time = Instant::now();
    let result = part(input);
    let final_time = Instant::now();

    println!(
        "Part {}: {}\n\trunner: {:?}",
        part_number,
        result,
        (final_time - start_time)
    );
}
