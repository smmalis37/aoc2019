use std::ops::*;

pub type SignedCoordinate = Coordinate<i32>;
pub type UnsignedCoordinate = Coordinate<usize>;

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

#[derive(Copy, Clone)]
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

impl Direction {
    pub(crate) fn to_unit(self) -> SignedCoordinate {
        use Direction::*;
        match self {
            Up => Coordinate { x: 0, y: 1 },
            Down => Coordinate { x: 0, y: -1 },
            Left => Coordinate { x: -1, y: 0 },
            Right => Coordinate { x: 1, y: 0 },
        }
    }
}
