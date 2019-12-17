use std::ops::*;

pub(crate) type SignedCoordinate = Coordinate<i32>;
pub(crate) type UnsignedCoordinate = Coordinate<usize>;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::str::FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => unreachable!(),
        })
    }
}

use Direction::*;

impl Direction {
    pub(crate) fn to_unit(self) -> SignedCoordinate {
        match self {
            Up => Coordinate { x: 0, y: 1 },
            Down => Coordinate { x: 0, y: -1 },
            Left => Coordinate { x: -1, y: 0 },
            Right => Coordinate { x: 1, y: 0 },
        }
    }

    pub(crate) fn turn_left(self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    pub(crate) fn turn_right(self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}
