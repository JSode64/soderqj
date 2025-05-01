use crate::tile::Tile;

use super::{
    enemies::{Jumper, Walker},
    entity::Entity,
    geometry::{Rect, Vec2},
    tile::TileID,
};
use sdl3::{render::Canvas, video::Window, EventPump};
use std::slice::Iter;

#[derive(Clone, Copy)]
struct MapLayout {
    /// The map tiles.
    tiles: &'static [(Rect, TileID)],

    /// The map enemies.
    enemies: &'static [&'static dyn Entity],

    /// The player spawn on the map.
    spawn: Vec2,
}

pub struct Map {
    tiles: Box<[(Rect, TileID)]>,
    enemies: Vec<Box<dyn Entity>>,
    spawn: Vec2,
}

impl MapLayout {
    const MAPS: [MapLayout; 1] = [MapLayout {
        tiles: &[
            (Rect::new(400.0, 701.0, 400.0, 95.0), TileID::LPad),
            (Rect::new(500.0, 405.0, 300.0, 100.0), TileID::Blck),
            (Rect::new(700.0, 505.0, 100.0, 100.0), TileID::Blck),
            (Rect::new(0.0, 700.0, 400.0, 100.0), TileID::Blck),
            (Rect::new(0.0, 0.0, 100.0, 700.0), TileID::Ladr),
            (Rect::new(200.0, 150.0, 200.0, 100.0), TileID::Blck),
            (Rect::new(750.0, 300.0, 50.0, 50.0), TileID::LPad),
        ],
        enemies: &[&Jumper::new(1.0, 1.0)],
        spawn: Vec2::new(350.0, 600.0),
    }];
}

impl Map {
    /// Returns the grid from the given index.
    pub fn get(i: usize) -> Self {
        let m = MapLayout::MAPS[i];

        Self {
            tiles: Box::from(m.tiles),
            enemies: vec![
                Box::new(Jumper::new(600.0, 100.0)),
                //Box::new(Jumper::new(500.0, 300.0)),
            ],
            spawn: m.spawn,
        }
    }

    /// Returns the map's player spawn location.
    pub fn get_spawn(&self) -> Vec2 {
        self.spawn
    }

    /// Returns the map's tiles.
    pub fn get_tiles(&self) -> &[(Rect, TileID)] {
        &self.tiles
    }

    /// Returns an iterator over the map's tiles.
    pub fn tile_iter(&self) -> Iter<'_, (Rect, TileID)> {
        self.tiles.iter()
    }

    /// Updates the map's enemies.
    pub fn update(&mut self, evp: &EventPump) {
        for e in self.enemies.iter_mut() {
            e.update(evp, &self.tiles);
        }
    }

    /// Draws the grid.
    pub fn draw(&self, cnv: &mut Canvas<Window>) {
        // Draw tiles.
        for &(body, tile) in &self.tiles {
            cnv.set_draw_color(tile.get_color());
            cnv.fill_rect(body).unwrap();
        }

        // Draw enemies.
        for e in &self.enemies {
            e.draw(cnv);
        }
    }
}
