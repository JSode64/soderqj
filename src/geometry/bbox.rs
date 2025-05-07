use super::{Square, Vec2};
use sdl3::render::FRect;

/// A 2D bouding-box.
///
/// Note that due to SDL's coordinate system, graphically, `y` is the top and `b` is the bottom.
#[derive(Clone, Copy, Debug)]
pub struct BBox {
    /// Minimum x.
    pub x: f32,

    /// Minimum y.
    pub y: f32,

    /// Maximum x.
    pub a: f32,

    /// Maximum y.
    pub b: f32,
}

impl BBox {
    /// Returns a new bounding-box with the given minimum and maximum bounds.
    pub const fn new(x: f32, y: f32, a: f32, b: f32) -> Self {
        Self { x, y, a, b }
    }

    /// Returns the bounding-box's center.
    pub const fn center(&self) -> Vec2 {
        Vec2::new((self.a - self.x) / 2.0, (self.b - self.y) / 2.0)
    }

    /// Returns true if the bounding-box and square collide, else false.
    pub const fn collides_with_sqr(&self, other: &Square) -> bool {
        self.x < other.x + other.s
            && self.a > other.x
            && self.y < other.y + other.s
            && self.b > other.y
    }

    /// Returns the containing status of the square in the bounding-box.
    /// Returns a tuple for the x and y axes, with true meaning it is contained.
    pub const fn contains_sqr(&self, other: &Square) -> (bool, bool) {
        (
            self.x <= other.x && self.a >= other.x + other.s,
            self.y <= other.y && self.b >= other.y + other.s,
        )
    }
}

impl From<&BBox> for Option<FRect> {
    fn from(value: &BBox) -> Self {
        Some(FRect::new(
            value.x,
            value.y,
            value.a - value.x,
            value.b - value.y,
        ))
    }
}
