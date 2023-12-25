use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub const DOWN: IVec2 = IVec2 { x: 0, y: 1 };
pub const LEFT: IVec2 = IVec2 { x: -1, y: 0 };
pub const RIGHT: IVec2 = IVec2 { x: 1, y: 0 };
pub const UP: IVec2 = IVec2 { x: 0, y: -1 };

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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

impl Add<&IVec2> for IVec2 {
    type Output = IVec2;

    fn add(self, rhs: &IVec2) -> Self::Output {
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

// Multiply
impl Mul<IVec2> for IVec2 {
    type Output = IVec2;

    fn mul(self, rhs: IVec2) -> Self::Output {
        return Self::new(self.x() * rhs.x(), self.y() * rhs.y());
    }
}

impl MulAssign<IVec2> for IVec2 {
    fn mul_assign(&mut self, rhs: IVec2) {
        self.x *= rhs.x();
        self.y *= rhs.y();
    }
}

impl Mul<i32> for IVec2 {
    type Output = IVec2;

    fn mul(self, rhs: i32) -> Self::Output {
        return Self::new(self.x() * rhs, self.y() * rhs);
    }
}

impl MulAssign<i32> for IVec2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
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

// Sub
impl Sub<IVec2> for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: IVec2) -> Self::Output {
        return Self::new(self.x() - rhs.x(), self.y() - rhs.y());
    }
}

impl Sub<&IVec2> for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: &IVec2) -> Self::Output {
        return Self::new(self.x() - rhs.x(), self.y() - rhs.y());
    }
}

impl Sub<&IVec2> for &IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: &IVec2) -> Self::Output {
        return IVec2::new(self.x() - rhs.x(), self.y() - rhs.y());
    }
}

impl SubAssign<IVec2> for IVec2 {
    fn sub_assign(&mut self, rhs: IVec2) {
        self.x -= rhs.x();
        self.y -= rhs.y();
    }
}

impl Sub<i32> for IVec2 {
    type Output = IVec2;

    fn sub(self, rhs: i32) -> Self::Output {
        return Self::new(self.x() - rhs, self.y() - rhs);
    }
}

impl SubAssign<i32> for IVec2 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs;
        self.y -= rhs;
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
