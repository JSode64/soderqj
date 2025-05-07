use sdl3::render::FRect;

use super::Vec2;

#[derive(Clone, Copy)]
pub struct Square {
    /// X-position (left bound).
    pub x: f32,

    /// Y-position (top bound).
    pub y: f32,

    /// Side length (width and height).
    pub s: f32,
}

impl Square {
    /// Returns a new square with the given position and side length.
    pub const fn new(x: f32, y: f32, s: f32) -> Self {
        Self { x, y, s }
    }

    /// Returns the square's center point.
    pub const fn center(&self) -> Vec2 {
        let half_s = self.s / 2.0;
        Vec2::new(self.x + half_s, self.y + half_s)
    }
}

impl From<&Square> for Option<FRect> {
    fn from(value: &Square) -> Self {
        Some(FRect::new(value.x, value.y, value.s, value.s))
    }
}
