use super::super::{
    config::GRAVITY,
    entity::Entity,
    geometry::{Square, Vec2},
    map::TileIter,
};
use sdl3::{keyboard::KeyboardState, pixels::Color};

/// An enemy that simply walks.
/// Once it hits a wall, it turns around.
pub struct Walker {
    /// Body.
    body: Square,

    /// Velocity.
    v: Vec2,
}

impl Walker {
    /// Body side length.
    const S: f32 = 28.0;

    /// Horizontal speed.
    const VX: f32 = 7.6;

    /// Returns a new walker with the given position.
    pub const fn new(x: f32, y: f32) -> Self {
        Self {
            body: Square::new(x, y, Self::S),
            v: Vec2::new(Self::VX, 0.0),
        }
    }
}

impl Entity for Walker {
    fn get_body(&self) -> Square {
        self.body
    }

    fn get_v(&self) -> Vec2 {
        self.v
    }

    fn get_color(&self) -> Color {
        Color {
            r: 225,
            g: 50,
            b: 150,
            a: 255,
        }
    }

    fn set_on_ground(&mut self, _: bool) {}

    fn set_pos(&mut self, p: Vec2) {
        self.body.x = p.x;
        self.body.y = p.y;
    }

    fn set_vx(&mut self, v: f32) {
        self.v.x = v;
    }

    fn set_vy(&mut self, v: f32) {
        self.v.y = v;
    }

    fn on_col_x(&mut self) {
        self.v.x = -self.v.x.signum() * Self::VX;
    }

    fn on_col_y(&mut self) {
        self.v.y = 0.0;
    }

    fn update(&mut self, _: Option<&KeyboardState>, map: TileIter) {
        // Fall.
        self.v.y += GRAVITY;

        // Handle map collision. Turn around on walls, handle ceilings and floors regularly.
        self.do_map_collision(map);
    }
}
