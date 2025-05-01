use super::{
    entity::Entity,
    geometry::{Rect, Vec2},
};
use sdl3::pixels::Color;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum TileID {
    Blck, // Block: simple solid.
    LPad, // Launch pad: launches the player away.
    Ladr, // Ladder: allows the player to jump off of it.
}

#[derive(Debug)]
pub struct Tile {
    /// The callback that is called when an entity collides with the tile.
    col_cb: fn(&Rect, &mut dyn Entity),

    /// The color of the tile.
    color: Color,
}

impl TileID {
    /// Returns the tile that the ID represents.
    pub fn get_color(self) -> Color {
        Tile::TILES[self as usize].color
    }

    /// Calls the tile's collision function on the entity.
    pub fn do_collision(self, r: &Rect, e: &mut dyn Entity) {
        (Tile::TILES[self as usize].col_cb)(r, e)
    }
}

impl Tile {
    const TILES: [Tile; 4] = [
        // `Blck` (block):
        Tile {
            col_cb: |_, _| {},
            color: Color {
                r: 100,
                g: 105,
                b: 125,
                a: 255,
            },
        },
        // `LPad` (launch pad):
        Tile {
            col_cb: |r, e| {
                // Get the closest point on the tile to the entity's center.
                let c = e.get_body().center();
                let p = Vec2::new(c.x.clamp(r.x, r.x + r.w), c.y.clamp(r.y, r.y + r.h));

                // Set velocity to launch away.
                let (y, x) = p.dir_to(c).sin_cos();
                e.set_vx(x * 25.0);
                e.set_vy(y * 25.0);
            },
            color: Color {
                r: 25,
                g: 255,
                b: 100,
                a: 255,
            },
        },
        // `Ladr` (ladder):
        Tile {
            col_cb: |_, e| e.set_on_ground(),
            color: Color {
                r: 255,
                g: 225,
                b: 125,
                a: 255,
            },
        },
        Tile {
            col_cb: |_, _| (),
            color: Color::MAGENTA,
        },
    ];
}
