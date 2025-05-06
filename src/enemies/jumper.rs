use crate::{
    config::GRAVITY,
    entity::Entity,
    geometry::{BBox, Square, Vec2},
    tile::TileID,
};
use sdl3::pixels::Color;

pub struct Jumper {
    /// Jumper's body.
    body: Square,

    /// Jumper's velocity.
    v: Vec2,
}

impl Jumper {
    /// Body side length.
    const S: f32 = 24.0;

    /// Horizontal speed.
    const VX: f32 = 5.1;

    /// Jumping velocity.
    const JMP_VY: f32 = -5.0;

    /// Returns a new jumper at the given position.
    pub const fn new(x: f32, y: f32) -> Self {
        Jumper {
            body: Square::new(x, y, Self::S),
            v: Vec2::new(Self::VX, Self::JMP_VY),
        }
    }
}

impl Entity for Jumper {
    fn get_body(&self) -> Square {
        self.body
    }

    fn get_v(&self) -> Vec2 {
        self.v
    }

    fn get_color(&self) -> Color {
        Color {
            r: 255,
            g: 100,
            b: 50,
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
        self.v.y = if self.v.y >= 0.0 { -25.0 } else { 0.0 };
    }

    fn update(&mut self, _: &sdl3::EventPump, map: &[(BBox, TileID)]) {
        // Fall.
        self.v.y += GRAVITY;

        // Handle map collision. Turn around on walls, handle ceilings and floors regularly.
        self.do_map_collision(map);
    }
}
