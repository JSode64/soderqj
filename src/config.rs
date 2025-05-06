use super::geometry::BBox;

/// Window width in pixels.
pub const WIN_W: usize = 800;

/// Window height in pixels.
pub const WIN_H: usize = 800;

/// Window span in a bounding-box.
pub const WIN_B: BBox = BBox::new(0.0, 0.0, WIN_W as _, WIN_H as _);

/// Gravity.
pub const GRAVITY: f32 = 1.0;
