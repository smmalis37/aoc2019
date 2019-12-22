use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait Solver<'a> {
    type Generated: Clone;
    type Output: Debug + PartialEq;

    fn generator(input: &'a str) -> Self::Generated;
    fn part1(data: Self::Generated) -> Self::Output;
    fn part2(data: Self::Generated) -> Self::Output;
}
