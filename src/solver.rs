pub trait Solver<'a> {
    type Generated: Clone;
    type Output: std::fmt::Debug;

    fn generator(input: &'a str) -> Self::Generated;
    fn part1(data: Self::Generated) -> Self::Output;
    fn part2(data: Self::Generated) -> Self::Output;
}
