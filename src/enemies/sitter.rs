use crate::{
    entity::Entity,
    geometry::{Square, Vec2},
    map::TileIter,
};
use sdl3::{keyboard::KeyboardState, pixels::Color};

/// An enemy that sits on one place, unaffected by gravity.
pub struct Sitter {
    /// Sitter's body.
    body: Square,

    /// Living status boolean.
    is_alive: bool,
}

impl Sitter {
    /// Body side length.
    pub const S: f32 = 30.0;

    /// Returns a new jumper at the given position.
    pub const fn new(x: f32, y: f32) -> Self {
        Sitter {
            body: Square::new(x, y, Self::S),
            is_alive: true,
        }
    }
}

impl Entity for Sitter {
    fn get_body(&self) -> Square {
        self.body
    }

    fn get_v(&self) -> Vec2 {
        panic!()
    }

    fn get_color(&self) -> Color {
        Color {
            r: 225,
            g: 150,
            b: 50,
            a: 255,
        }
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn set_on_ground(&mut self, _: bool) {}

    fn set_pos(&mut self, p: Vec2) {
        self.body.x = p.x;
        self.body.y = p.y;
    }

    fn set_vx(&mut self, _: f32) {
        panic!()
    }

    fn set_vy(&mut self, _: f32) {
        panic!()
    }

    fn kill(&mut self) {
        self.is_alive = false;
    }

    fn on_col_x(&mut self) {
        panic!()
    }

    fn on_col_y(&mut self) {
        panic!()
    }

    fn update(&mut self, _: Option<&KeyboardState>, _: TileIter) {}
}
