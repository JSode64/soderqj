use super::vec2::Vec2;

#[derive(Clone, Copy)]
pub struct Tri {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2,
}

impl Tri {
    /// Returns a new triangle from the given points
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self { a, b, c }
    }

    /// Returns a new triangle from the given x-y coordinates
    pub fn new_xy(ax: f32, ay: f32, bx: f32, by: f32, cx: f32, cy: f32) -> Self {
        Self {
            a: Vec2::new(ax, ay),
            b: Vec2::new(bx, by),
            c: Vec2::new(cx, cy),
        }
    }

    /// Returnns the closest point on the triangle's edges to the point
    pub fn closest_to_point(&self, p: Vec2) -> Vec2 {
        // Get closest points
        let a = p.closest_to_seg(self.a, self.b);
        let b = p.closest_to_seg(self.b, self.c);
        let c = p.closest_to_seg(self.c, self.a);

        // Get the points' distances to the target
        let da = a.dst_to_sqr(p);
        let db = b.dst_to_sqr(p);
        let dc = c.dst_to_sqr(p);

        // Find the smallest distance and return its point
        if da <= db && da <= dc {
            a
        } else if db <= dc {
            b
        } else {
            c
        }
    }

    /// Returns the closest point on the triangle's edges to the point (0)
    /// Also returns points A and B (1 and 2) where AB is the triangle's edge the point is nearest
    pub fn closest_to_point_ex(&self, p: Vec2) -> (Vec2, Vec2, Vec2) {
        // Get closest points
        let a = p.closest_to_seg(self.a, self.b);
        let b = p.closest_to_seg(self.b, self.c);
        let c = p.closest_to_seg(self.c, self.a);

        // Get the points' distances to the target
        let da = a.dst_to_sqr(p);
        let db = b.dst_to_sqr(p);
        let dc = c.dst_to_sqr(p);

        // Find the smallest distance and return its point
        if da <= db && da <= dc {
            (a, self.a, self.b)
        } else if db <= dc {
            (b, self.b, self.c)
        } else {
            (c, self.c, self.a)
        }
    }

    /// Returns true if the point is inside the triangle (not including on-edge)
    pub fn contains_point(&self, p: Vec2) -> bool {
        let a = (self.b - self.a).cross(p - self.a).signum() as i32;
        let b = (self.c - self.b).cross(p - self.b).signum() as i32;
        let c = (self.a - self.c).cross(p - self.c).signum() as i32;
        a == b && b == c
    }
}
