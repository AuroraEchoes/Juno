use std::ops::{Add, AddAssign};

pub trait Vector {
    type Coord;

    fn new(x: Self::Coord, y: Self::Coord) -> Self;
    fn x(&self) -> &Self::Coord;
    fn y(&self) -> &Self::Coord;
    fn add_x(&mut self, x: &Self::Coord);
    fn add_y(&mut self, y: &Self::Coord);
    fn subtract_x(&mut self, x: &Self::Coord);
    fn subtract_y(&mut self, y: &Self::Coord);
    fn transform(&mut self, t: &Self);
    fn in_bounds(&self, min: &Self, max: &Self) -> bool;
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IVec2 {
    x: i32,
    y: i32,
}

impl Vector for IVec2 {
    type Coord = i32;

    fn new(x: Self::Coord, y: Self::Coord) -> Self {
        return Self { x, y };
    }

    fn x(&self) -> &Self::Coord {
        return &self.x;
    }

    fn y(&self) -> &Self::Coord {
        return &self.y;
    }

    fn add_x(&mut self, x: &Self::Coord) {
        self.x += x;
    }

    fn add_y(&mut self, y: &Self::Coord) {
        self.y += y;
    }

    fn subtract_x(&mut self, x: &Self::Coord) {
        self.x -= x;
    }

    fn subtract_y(&mut self, y: &Self::Coord) {
        self.y -= y;
    }

    fn transform(&mut self, t: &Self) {
        self.x += t.x;
        self.y += t.y;
    }

    fn in_bounds(&self, min: &Self, max: &Self) -> bool {
        return self > min && self < max;
    }
}

impl Add<IVec2> for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: IVec2) -> Self::Output {
        return Self::new(self.x() + rhs.x(), self.y() + rhs.y());
    }
}

impl AddAssign<IVec2> for IVec2 {
    fn add_assign(&mut self, rhs: IVec2) {
        self.x += rhs.x();
        self.y += rhs.y();
    }
}

#[macro_export]
macro_rules! ivec {
    ($a: expr, $b: expr) => {
        IVec2::new($a, $b)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ivec_macro() {
        let vec = ivec!(2, 2);
        let equiv = IVec2::new(2, 2);
        assert_eq!(vec, equiv);
    }
}
