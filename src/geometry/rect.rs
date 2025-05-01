use super::vec2::Vec2;
use sdl3::render::FRect;

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    /// X-position (left bound).
    pub x: f32,

    /// Y-position (top bound).
    pub y: f32,

    /// Width.
    pub w: f32,

    /// Height.
    pub h: f32,
}

impl Rect {
    /// Returns a new rectangle with the given position and dimensions.
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Returns the rectangle's center.
    pub const fn center(&self) -> Vec2 {
        Vec2::new(self.x + (self.w / 2.0), self.y + (self.h / 2.0))
    }

    /// Returns true if the rectangles collide, else false.
    pub const fn do_collide(&self, other: &Self) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

impl From<Rect> for Option<FRect> {
    fn from(value: Rect) -> Self {
        Some(FRect::new(value.x, value.y, value.w, value.h))
    }
}
