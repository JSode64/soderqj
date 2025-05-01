use crate::tile::TileID;

use super::{
    config::WIN_W,
    geometry::{Rect, Square, Vec2},
    map::Map,
};
use sdl3::{pixels::Color, render::Canvas, video::Window, EventPump};

pub trait Entity {
    /// Returns the entity's body.
    fn get_body(&self) -> Square;

    /// Returns the entity's velocities.
    fn get_v(&self) -> Vec2;

    /// Returns the color of the entity.
    fn get_color(&self) -> Color;

    /// Sets the entity's position to the given one.
    fn set_pos(&mut self, p: Vec2);

    /// Sets the entity's x-velocity to the given one.
    fn set_vx(&mut self, v: f32);

    /// Sets the entity's y-velocity to the given one.
    fn set_vy(&mut self, v: f32);

    /// Sets the entity's `on_ground` variable to true.
    ///
    /// This essentially just enables the entity to jump.
    fn set_on_ground(&mut self);

    /// Updates the entity.
    fn update(&mut self, evp: &EventPump, map: &[(Rect, TileID)]);

    fn draw(&self, cnv: &mut Canvas<Window>) {
        cnv.set_draw_color(self.get_color());
        cnv.fill_rect(self.get_body()).unwrap();
    }

    /// Handles entity collision with the map.
    ///
    /// Takes two closures that return the new x and y velocity if collision occurs in either of the planes.
    fn do_map_collision(
        &mut self,
        map: &[(Rect, TileID)],
        vx_cb: fn(f32) -> f32,
        vy_cb: fn(f32) -> f32,
    ) where
        Self: Sized,
    {
        let body = self.get_body();
        let s = body.s;
        let v = self.get_v();
        let mut new_x = (body.x + v.x).clamp(0.0, WIN_W as f32 - s);
        let mut new_y = body.y + v.y;

        for (rect, tile) in map {
            let mut hit = false;

            // Check for horizontal collision.
            let new_b = Square::new(new_x, body.y, s);
            if new_b.hits_rect(&rect) {
                if v.x > 0.0 {
                    new_x = rect.x - s;
                } else {
                    new_x = rect.x + rect.w;
                }
                self.set_vx(vx_cb(v.x));
                hit = true;
            }

            // Check for vertical collision.
            let new_b = Square::new(new_x, new_y, s);
            if new_b.hits_rect(&rect) {
                if v.y > 0.0 {
                    // Landing on ground.
                    new_y = rect.y - s;
                    self.set_on_ground();
                } else {
                    new_y = rect.y + rect.h;
                }
                self.set_vy(vy_cb(v.y));
                hit = true;
            }

            if hit {
                tile.do_collision(rect, self);
            }
        }

        self.set_pos(Vec2::new(new_x, new_y));
    }
}
