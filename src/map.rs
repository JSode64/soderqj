use super::{
    enemies::EnemyVec,
    enemies::{Jumper, Walker},
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
        // If tab is pressed, reset.
        if kbs.is_scancode_pressed(Scancode::Tab) {
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

    /// Constant initial states for all maps in the game.
    const MAPS: [Map; 1] = [Map {
        tiles: &[
            (BBox::new(400.0, 701.0, 800.0, 796.0), TileID::VPad),
            (BBox::new(500.0, 405.0, 800.0, 505.0), TileID::Blck),
            (BBox::new(700.0, 505.0, 800.0, 605.0), TileID::Blck),
            (BBox::new(0.0, 700.0, 400.0, 800.0), TileID::Blck),
            (BBox::new(0.0, 0.0, 100.0, 700.0), TileID::Ladr),
            (BBox::new(200.0, 150.0, 400.0, 250.0), TileID::Blck),
            (BBox::new(750.0, 0.0, 800.0, 405.0), TileID::HPad),
        ],
        enemies: &[
            || Box::new(Jumper::new(150.0, 50.0)),
            || Box::new(Walker::new(400.0, 0.0)),
        ],
        spawn: Vec2::new(350.0, 600.0),
        i: 0,
    }];
}
