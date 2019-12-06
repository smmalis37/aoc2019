use aoc2019::*;
use std::{fmt::Display, io::prelude::*, time::Instant};

fn main() {
    println!("AOC 2019");
    run(1, day1::generator, day1::part1, day1::part2);
    run(2, day2::generator, day2::part1, day2::part2);
    run(3, day3::generator, day3::part1, day3::part2);
    run(4, day4::generator, day4::part1, day4::part2);
    run(5, day5::generator, day5::part1, day5::part2);
}

fn run<T, O1: Display, O2: Display>(
    day_number: usize,
    generator: impl Fn(&str) -> T + Copy,
    part1: impl Fn(T) -> O1,
    part2: impl Fn(T) -> O2,
) {
    let mut input = String::new();

    std::fs::File::open(format!("input/2019/day{}.txt", day_number))
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let trimmed_input = input.trim();
    run_half(trimmed_input, day_number, 1, generator, part1);
    run_half(trimmed_input, day_number, 2, generator, part2);
}

fn run_half<T, O: Display>(
    input: &str,
    day_number: usize,
    part_number: usize,
    generator: impl Fn(&str) -> T,
    part: impl Fn(T) -> O,
) {
    let start_time = Instant::now();
    let generated = generator(input);
    let inter_time = Instant::now();
    let result = part(generated);
    let final_time = Instant::now();

    println!(
        "Day {} - Part {} : {}\n\tgenerator: {:?},\n\trunner: {:?}\n",
        day_number,
        part_number,
        result,
        (inter_time - start_time),
        (final_time - inter_time)
    );
}
