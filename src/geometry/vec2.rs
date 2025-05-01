use sdl3::render::FPoint;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Returns a new vector with the given x and y coordinates.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns a new vector with zero x and y.
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Returns the direction to the given vector in radians.
    pub fn dir_to(self, other: Self) -> f32 {
        (other.y - self.y).atan2(other.x - self.x)
    }
}

impl From<Vec2> for FPoint {
    fn from(value: Vec2) -> Self {
        Self::new(value.x, value.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
