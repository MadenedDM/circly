use std::ops::Add;

use crate::types::point::Point;

#[derive(Debug, Default)]
pub struct Position {
    pub inner: Point,
}

impl Position {
    #[must_use]
    pub fn new(value: Point) -> Self {
        Self { inner: value }
    }
}

impl From<Point> for Position {
    fn from(value: Point) -> Self {
        Self { inner: value }
    }
}

impl Add<Point> for Position {
    type Output = Position;

    fn add(self, rhs: Point) -> Self::Output {
        #[expect(clippy::arithmetic_side_effects)]
        Position::new(self.inner + rhs)
    }
}
