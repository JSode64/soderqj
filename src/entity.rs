use crate::tile::TileID;

use super::{
    config::{WIN_B, WIN_H, WIN_W},
    geometry::{BBox, Square, Vec2},
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

    /// Called when the entity collides with something horizontally.
    /// Should handle updating the x-velocity.
    fn on_col_x(&mut self);

    /// Called when the eneity collides with something vertically.
    /// Should handle updating the y-velocity.
    fn on_col_y(&mut self);

    /// Sets the entity's "on ground" status based on the given boolean.
    fn set_on_ground(&mut self, b: bool);

    /// Updates the entity.
    fn update(&mut self, evp: &EventPump, map: &[(BBox, TileID)]);

    fn draw(&self, cnv: &mut Canvas<Window>) {
        cnv.set_draw_color(self.get_color());
        cnv.fill_rect(self.get_body()).unwrap();
    }

    /// Handles entity collision with the map.
    ///
    /// Takes two closures that return the new x and y velocity if collision occurs in either of the planes.
    fn do_map_collision(&mut self, map: &[(BBox, TileID)])
    where
        Self: Sized,
    {
        let body = self.get_body();
        let s = body.s;
        let v = self.get_v();
        let mut new_x = body.x + v.x;
        let mut new_y = body.y + v.y;

        // Check for out-of-bounds.
        let (in_x, in_y) = WIN_B.contains_sqr(&Square::new(new_x, new_y, s));

        if !in_x {
            new_x = new_x.clamp(0.0, WIN_W as f32 - s);
            self.on_col_x();
        }
        if !in_y {
            new_y = new_y.clamp(0.0, WIN_H as f32 - s);
            self.on_col_y();
        }

        // Set on ground until a landing collision is found.
        // If not done here, walking off an edge will not mark the player as not-grounded.
        self.set_on_ground(false);

        for (bbox, tile) in map {
            let mut hit = false;

            // Check for horizontal collision.
            let new_b = Square::new(new_x, body.y, s);

            if bbox.collides_with_sqr(&new_b) {
                if v.x > 0.0 {
                    // Hit from left.
                    new_x = bbox.x - s;
                } else {
                    // Hit from right.
                    new_x = bbox.a;
                }
                self.on_col_x();
                hit = true;
            }

            // Check for vertical collision.
            let new_b = Square::new(new_x, new_y, s);

            if bbox.collides_with_sqr(&new_b) {
                if v.y > 0.0 {
                    // Landing on ground.
                    new_y = bbox.y - s;
                    self.set_on_ground(true);
                } else {
                    // Hitting ceiling.
                    new_y = bbox.b;
                }
                self.on_col_y();
                hit = true;
            }

            // Run collision callback if collided.
            if hit {
                tile.do_collision(bbox, self);
            }
        }

        self.set_pos(Vec2::new(new_x, new_y));
    }
}
