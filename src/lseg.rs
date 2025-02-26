use super::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct LSeg {
    pub a: Vec2,
    pub b: Vec2,
}

impl LSeg {
    /// Returns a new line segment with the given points
    pub const fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    /// Returns a new line segment with the givenn x and y coordinates
    pub const fn new_xy(ax: f32, ay: f32, bx: f32, by: f32) -> Self {
        Self {
            a: Vec2::new(ax, ay),
            b: Vec2::new(bx, by),
        }
    }

    /// Returns the closest point on the line segment to the given point
    pub fn closest(&self, p: Vec2) -> Vec2 {
        let ab = self.b - self.a;
        let t = ((p - self.a).dot(ab) / ab.mag_sqr()).clamp(0.0, 1.0);
        Vec2::new(self.a.x + (ab.x * t), self.a.y + (ab.y * t))
    }

    /// Returns true if the point is in the line segment's box, else false
    pub fn contains(&self, p: Vec2) -> bool {
        (p.x - self.a.x).signum() != (p.x - self.b.x)
            && (p.y - self.a.y).signum() != (p.y - self.b.y).signum()
    }

    /// Returns true if the line segments collide, false otherwise.
    /// Does not check for colinear collosion
    pub fn hits(&self, other: &Self) -> bool {
        let a1 = (self.b - self.a).cross(other.a - self.a).signum();
        let a2 = (self.b - self.a).cross(other.b - self.a).signum();
        let b1 = (other.b - other.a).cross(self.a - other.a).signum();
        let b2 = (other.b - other.a).cross(self.b - other.a).signum();
        a1 != a2 && b1 != b2
    }
}
