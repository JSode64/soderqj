use super::{entity::Entity, geometry::BBox};
use sdl3::pixels::Color;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum TileID {
    Blck, // Block: simple solid.
    VPad, // Vertical launch pad: launches the player away vertically.
    HPad, // Horiontal launch pad: launches the player away horizontally.
    Ladr, // Ladder: allows the player to jump off of it.
}

#[derive(Debug)]
pub struct Tile {
    /// The callback that is called when an entity collides with the tile.
    col_cb: fn(&BBox, &mut dyn Entity),

    /// The color of the tile.
    color: Color,
}

impl TileID {
    /// Returns the tile that the ID represents.
    pub fn get_color(self) -> Color {
        Tile::TILES[self as usize].color
    }

    /// Calls the tile's collision function on the entity.
    pub fn do_collision(self, r: &BBox, e: &mut dyn Entity) {
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
        // `VPad` (vertical launch pad):
        Tile {
            col_cb: |b, e| {
                // Launch up if above and down if below.
                e.set_vy(if e.get_body().center().y >= b.center().y {
                    -25.0
                } else {
                    25.0
                });

                // Prevents jumping on the pad from being a normal jump.
                e.set_on_ground(false);
            },
            color: Color {
                r: 25,
                g: 255,
                b: 200,
                a: 255,
            },
        },
        // `HPad` (horizontal launch pad):
        Tile {
            col_cb: |b, e| {
                // Launch left if to the left and right if to the riht.
                e.set_vx(if e.get_body().center().x >= b.center().x {
                    -25.0
                } else {
                    25.0
                })
            },
            color: Color {
                r: 25,
                g: 200,
                b: 255,
                a: 255,
            },
        },
        // `Ladr` (ladder):
        Tile {
            col_cb: |_, e| e.set_on_ground(true),
            color: Color {
                r: 255,
                g: 225,
                b: 125,
                a: 255,
            },
        },
    ];
}
