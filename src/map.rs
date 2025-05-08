use super::{
    config::{WIN_H, WIN_W},
    enemies::EnemyVec,
    enemies::{Jumper, Sitter, Walker},
    entity::Entity,
    geometry::{BBox, Vec2},
    player::Player,
    tile::TileID,
};
use sdl3::{
    keyboard::{KeyboardState, Scancode},
    render::Canvas,
    video::Window,
};
use std::slice::Iter;

/// A layout for a map.
#[derive(Clone, Copy)]
pub struct Map {
    /// The map tiles.
    tiles: &'static [(BBox, TileID)],

    /// The map enemies.
    enemies: &'static [fn() -> Box<dyn Entity>],

    /// The player spawn on the map.
    spawn: Vec2,

    /// The map index.
    i: usize,
}

/// The iterator type for map tiles.
pub type TileIter = Iter<'static, (BBox, TileID)>;

impl Map {
    /// Returns an initialized game state from the given index.
    pub fn init_game(i: usize) -> (&'static Self, Player, EnemyVec) {
        let m = &Self::MAPS[i];

        (
            &Self::MAPS[i],
            Player::new(m.spawn),
            m.enemies.into_iter().map(|f| f()).collect(),
        )
    }

    /// Returns an iterator of the map's tiles.
    pub fn tile_iter(&self) -> TileIter {
        self.tiles.iter()
    }

    /// Updates the game state; resetting the state if tab was pressed.
    pub fn update(&self, kbs: &KeyboardState, p: &mut Player, e: &mut EnemyVec) {
        // If tab is pressed or the player was killed, reset.
        if kbs.is_scancode_pressed(Scancode::Tab) || !p.is_alive() {
            (_, *p, *e) = Self::init_game(self.i);
        }
    }

    /// Draws the map.
    pub fn draw(&self, cnv: &mut Canvas<Window>) {
        for (b, t) in self.tiles.into_iter() {
            cnv.set_draw_color(t.get_color());
            cnv.fill_rect(b).unwrap();
        }
    }

    /// The number of maps.
    pub const N: usize = 4;

    /// Constant initial states for all maps in the game.
    const MAPS: [Map; Self::N] = [
        Map {
            tiles: &[
                (
                    BBox::new(0.0, WIN_H as f32 - 50.0, WIN_W as f32 / 2.0, WIN_H as _),
                    TileID::Blck,
                ),
                (
                    BBox::new(0.0, WIN_H as f32 / 2.0, 50.0, WIN_H as f32 - 50.0),
                    TileID::Ladr,
                ),
                (
                    BBox::new(
                        WIN_W as f32 / 2.0,
                        WIN_H as f32 - 50.0,
                        WIN_W as f32 * 0.75,
                        WIN_H as _,
                    ),
                    TileID::VPad,
                ),
                (
                    BBox::new(
                        (WIN_W as f32 * 0.75) - 50.0,
                        (WIN_H as f32 * 0.75) - 50.0,
                        WIN_W as f32 * 0.75,
                        WIN_H as f32 - 50.0,
                    ),
                    TileID::HPad,
                ),
                (
                    BBox::new(
                        WIN_W as f32 * 0.75,
                        WIN_H as f32 - 50.0,
                        WIN_W as _,
                        WIN_H as _,
                    ),
                    TileID::Fire,
                ),
                (BBox::new(0.0, 0.0, 50.0, WIN_H as f32 / 2.0), TileID::VPad),
                (
                    BBox::new(100.0, 0.0, 150.0, WIN_H as f32 / 2.0),
                    TileID::VPad,
                ),
            ],
            enemies: &[
                || Box::new(Walker::new(50.0, 0.0)),
                || {
                    Box::new(Sitter::new(
                        200.0 - (Sitter::S / 2.0),
                        50.0 - (Sitter::S / 2.0),
                    ))
                },
            ],
            spawn: Vec2::new((WIN_W as f32) / 3.0, (WIN_H as f32) - 50.0 - Player::S),
            i: 0,
        },
        Map {
            tiles: &[
                (
                    BBox::new(50.0, WIN_H as f32 - 50.0, WIN_W as f32 - 50.0, WIN_H as _),
                    TileID::Blck,
                ),
                (BBox::new(0.0, 0.0, 50.0, WIN_H as _), TileID::Ladr),
                (
                    BBox::new(WIN_W as f32 - 50.0, 0.0, WIN_W as _, WIN_H as _),
                    TileID::Ladr,
                ),
                (
                    BBox::new(100.0, 250.0, WIN_W as f32 - 50.0, 300.0),
                    TileID::Blck,
                ),
                (
                    BBox::new(50.0, 550.0, WIN_W as f32 - 100.0, 600.0),
                    TileID::Blck,
                ),
                (
                    BBox::new(
                        WIN_W as f32 - Player::S - 120.0,
                        0.0,
                        WIN_W as f32 - Player::S - 70.0,
                        100.0,
                    ),
                    TileID::Blck,
                ),
            ],
            enemies: &[
                || {
                    Box::new(Sitter::new(
                        WIN_W as f32 - Sitter::S - 60.0,
                        Sitter::S + 10.0,
                    ))
                },
                || Box::new(Walker::new(100.0, 0.0)),
                || Box::new(Jumper::new(150.0, 100.0)),
                || Box::new(Walker::new(200.0, 180.0)),
                || Box::new(Jumper::new(300.0, 50.0)),
                || Box::new(Walker::new(400.0, 120.0)),
                || Box::new(Jumper::new(500.0, 160.0)),
                || Box::new(Walker::new(600.0, 90.0)),
                || Box::new(Jumper::new(700.0, 200.0)),
                || Box::new(Walker::new(150.0, 440.0)),
                || Box::new(Jumper::new(250.0, 460.0)),
                || Box::new(Walker::new(350.0, 420.0)),
                || Box::new(Jumper::new(450.0, 500.0)),
                || Box::new(Walker::new(550.0, 470.0)),
                || Box::new(Jumper::new(650.0, 480.0)),
            ],
            spawn: Vec2::new(60.0 + Player::S, WIN_H as f32 - Player::S - 50.0),
            i: 1,
        },
        Map {
            tiles: &[
                (
                    BBox::new(0.0, WIN_H as f32 - 50.0, WIN_W as _, WIN_H as _),
                    TileID::Blck,
                ),
                (
                    BBox::new(420.0, WIN_H as f32 - 100.0, WIN_W as _, WIN_H as _),
                    TileID::Blck,
                ),
                (
                    BBox::new(WIN_W as f32 - 50.0, 0.0, WIN_W as _, WIN_H as f32 - 100.0),
                    TileID::HPad,
                ),
                (
                    BBox::new(0.0, 200.0, WIN_W as f32 * 0.8, 250.0),
                    TileID::Blck,
                ),
                (
                    BBox::new(
                        (WIN_W as f32 * 0.8) - 50.0,
                        250.0,
                        WIN_W as f32 * 0.8,
                        425.0,
                    ),
                    TileID::HPad,
                ),
            ],
            enemies: &[
                || Box::new(Walker::new(400.0, WIN_H as f32 - Walker::S - 50.0)),
                || {
                    Box::new(Walker::new(
                        WIN_W as f32 - 350.0 - Walker::S,
                        WIN_H as f32 - Walker::S - 100.0,
                    ))
                },
                || Box::new(Jumper::new(0.0, 0.0)),
                || Box::new(Jumper::new(100.0, 55.0)),
                || Box::new(Jumper::new(200.0, 60.0)),
                || Box::new(Jumper::new(300.0, 10.0)),
                || Box::new(Jumper::new(350.0, 20.0)),
                || Box::new(Jumper::new(400.0, 5.0)),
                || Box::new(Jumper::new(500.0, 50.0)),
                || Box::new(Walker::new(0.0, 50.0)),
                || Box::new(Walker::new(250.0, 60.0)),
                || Box::new(Walker::new(500.0, 75.0)),
            ],
            spawn: Vec2::new(Player::S + 10.0, WIN_H as f32 - Player::S - 50.0),
            i: 2,
        },
        Map {
            tiles: &[
                (
                    BBox::new(
                        0.0,
                        WIN_H as f32 - 50.0,
                        (WIN_W as f32 + 630.0) / 2.0,
                        WIN_H as _,
                    ),
                    TileID::Blck,
                ),
                (
                    BBox::new(100.0, WIN_H as f32 - 125.0, 150.0, WIN_H as _),
                    TileID::Fire,
                ),
                (
                    BBox::new(250.0, WIN_H as f32 - 125.0, 305.0, WIN_H as _),
                    TileID::Fire,
                ),
                (
                    BBox::new(405.0, WIN_H as f32 - 125.0, 465.0, WIN_H as _),
                    TileID::Fire,
                ),
                (
                    BBox::new(565.0, WIN_H as f32 - 125.0, 630.0, WIN_H as _),
                    TileID::Fire,
                ),
                (
                    BBox::new(
                        (WIN_W as f32 + 630.0) / 2.0,
                        WIN_H as f32 - 50.0,
                        WIN_W as _,
                        WIN_H as _,
                    ),
                    TileID::VPad,
                ),
                (
                    BBox::new(
                        625.0,
                        WIN_H as f32 / 2.0,
                        710.0,
                        (WIN_H as f32 / 2.0) + 50.0,
                    ),
                    TileID::VPad,
                ),
                (
                    BBox::new(0.0, Sitter::S + 20.0, Sitter::S + 20.0, Sitter::S + 70.0),
                    TileID::Blck,
                ),
            ],
            enemies: &[
                || Box::new(Sitter::new(10.0, 10.0)),
                || Box::new(Sitter::new(200.0, (WIN_H as f32 / 2.0) + 12.5)),
                || Box::new(Sitter::new(355.0, (WIN_H as f32 / 2.0) + 12.5)),
                || Box::new(Sitter::new(510.0, (WIN_H as f32 / 2.0) + 12.5)),
            ],
            spawn: Vec2::new(25.0, WIN_H as f32 - 82.0),
            i: 3,
        },
    ];
}
