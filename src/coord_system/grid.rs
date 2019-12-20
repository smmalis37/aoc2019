use super::unsigned::*;
use std::ops::*;

pub(crate) struct Grid<T>(Vec<Vec<T>>);

impl<T> Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, index: Point) -> &Self::Output {
        &self.0[index.y][index.x]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.0[index.y][index.x]
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = Vec<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> Grid<T> {
    pub(crate) fn new() -> Self {
        Self(Vec::new())
    }

    pub(crate) fn in_bounds(&self, p: Point) -> bool {
        p.y < self.0.len() && p.x < self.0[0].len()
    }
}
