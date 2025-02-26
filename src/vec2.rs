use sdl3::render::FPoint;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    /// Returns a new vec2 with the given x and y coordinates
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Returns a new vec2 with the given x and y coordinate
    pub const fn from(xy: f32) -> Self {
        Self { x: xy, y: xy }
    }

    /// Returns a new vec2 with zero x and y
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Returns the closest point to P (self) that is on the line segment AB
    pub fn closest_to_seg(self, a: Vec2, b: Vec2) -> Vec2 {
        let edge = b - a;
        let p = self - a;
        a + (edge * Vec2::from((p.dot(edge) / edge.mag_sqr()).clamp(0.0, 1.0)))
    }

    /// Returns the cross product of the vec2s (A x B)
    pub fn cross(self, other: Self) -> f32 {
        (self.x * other.y) - (self.y * other.x)
    }

    /// Returs the direction in radians to the given point
    pub fn dir_to(self, other: Self) -> f32 {
        f32::atan2(other.y - self.y, other.x - self.x)
    }

    /// Returns the dot product of the two vec2s
    pub fn dot(self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    /// Returns the distance between the vec2s squared
    pub fn dst_to_sqr(self, other: Self) -> f32 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        (x * x) + (y * y)
    }

    /// Returns the magnitude of the vec2 squared
    pub fn mag_sqr(self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
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
