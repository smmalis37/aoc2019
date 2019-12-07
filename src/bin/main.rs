use aoc2019::*;
use std::error::Error;
use std::fmt::Display;
use std::io::prelude::*;
use std::time::Instant;

fn main() -> Result<(), Box<dyn Error>> {
    println!("AOC 2019");
    run(1, day1::generator, day1::part1, day1::part2)?;
    run(2, day2::generator, day2::part1, day2::part2)?;
    run(3, day3::generator, day3::part1, day3::part2)?;
    run(4, day4::generator, day4::part1, day4::part2)?;
    run(5, day5::generator, day5::part1, day5::part2)?;
    //run(6, day6::generator, day6::part1, day6::part2)?;
    Ok(())
}

fn run<T: Clone, O1: Display, O2: Display>(
    day_number: usize,
    generator: impl Fn(&str) -> T,
    part1: impl Fn(T) -> O1,
    part2: impl Fn(T) -> O2,
) -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    std::fs::File::open(format!("input/2019/day{}.txt", day_number))?.read_to_string(&mut input)?;
    let trimmed_input = input.trim();

    let start_time = Instant::now();
    let generated = generator(trimmed_input);
    let final_time = Instant::now();

    println!(
        "\nDay {}:\n\tgenerator : {:?}",
        day_number,
        (final_time - start_time)
    );

    run_half(generated.clone(), 1, part1);
    run_half(generated, 2, part2);
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
