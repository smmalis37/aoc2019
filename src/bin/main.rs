use aoc2019::*;
use std::error::Error;
use std::fmt::Display;
use std::io::prelude::*;
use std::time::Instant;

macro_rules! days {
    ( $( $d:expr ),* ) => {
        $(
            paste::expr! {
                run::<[<day $d>]::[<Day $d>]>($d)?;
            }
        )*
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("AOC 2019");
    days!(1, 2, 3, 4, 5, 6, 7);
    Ok(())
}

fn run<S: for<'a> solver::Solver<'a>>(day_number: usize) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    std::fs::File::open(format!("input/2019/day{}.txt", day_number))?.read_to_string(&mut input)?;
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
    Ok(())
}

fn run_half<T, O: Display>(input: T, part_number: usize, part: impl Fn(T) -> O) {
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
