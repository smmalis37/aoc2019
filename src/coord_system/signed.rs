use super::direction::Direction;
use std::ops::*;

pub(crate) type Coordinate = i32;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point {
    pub(crate) fn add_dir(&mut self, d: Direction) {
        use Direction::*;
        match d {
            Up => self.y += 1,
            Down => self.y -= 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
    }
}
