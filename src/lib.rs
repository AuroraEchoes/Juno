use std::ops::{Add, AddAssign, Div, DivAssign};

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

// Add
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

impl Add<i32> for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: i32) -> Self::Output {
        return Self::new(self.x() + rhs, self.y() + rhs);
    }
}

impl AddAssign<i32> for IVec2 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs;
        self.y += rhs;
    }
}

// Divide
impl Div<IVec2> for IVec2 {
    type Output = IVec2;

    fn div(self, rhs: IVec2) -> Self::Output {
        return Self::new(self.x() / rhs.x(), self.y() / rhs.y());
    }
}

impl DivAssign<IVec2> for IVec2 {
    fn div_assign(&mut self, rhs: IVec2) {
        self.x /= rhs.x();
        self.y /= rhs.y();
    }
}

impl Div<i32> for IVec2 {
    type Output = IVec2;

    fn div(self, rhs: i32) -> Self::Output {
        return Self::new(self.x() / rhs, self.y() / rhs);
    }
}

impl DivAssign<i32> for IVec2 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! ivec {
        ($a: expr, $b: expr) => {
            IVec2::new($a, $b)
        };
    }
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

    #[test]
    fn test_add() {
        assert_eq!(ivec!(2, 5), ivec!(1, 4) + ivec!(1, 1));
        let mut add_eq = ivec!(1, 1);
        add_eq += ivec!(2, 4);
        assert_eq!(ivec!(3, 5), add_eq);
        assert_eq!(ivec!(2, 2), ivec!(1, 1) + 1);
        let mut add_eq_i32 = ivec!(1, 1);
        add_eq_i32 += 1;
        assert_eq!(ivec!(2, 2), add_eq_i32);
    }

    #[test]
    fn test_divide() {
        assert_eq!(ivec!(3, 2), ivec!(9, 8) / ivec!(3, 4));
        let mut div_eq = ivec!(9, 8);
        div_eq /= ivec!(3, 4);
        assert_eq!(ivec!(3, 2), div_eq);
        assert_eq!(ivec!(2, 4), ivec!(4, 8) / 2);
        let mut div_eq_i32 = ivec!(2, 4);
        div_eq_i32 /= 2;
        assert_eq!(ivec!(1, 2), div_eq_i32);
    }
}
