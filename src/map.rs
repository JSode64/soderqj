use super::{
    enemies::{Jumper, Walker},
    entity::Entity,
    geometry::{BBox, Vec2},
    tile::TileID,
};
use sdl3::{render::Canvas, video::Window, EventPump};

#[derive(Clone, Copy)]
struct MapLayout {
    /// The map tiles.
    tiles: &'static [(BBox, TileID)],

    /// The map enemies.
    enemies: &'static [fn() -> Box<dyn Entity>],

    /// The player spawn on the map.
    spawn: Vec2,
}

pub struct Map {
    /// The map tiles.
    tiles: Box<[(BBox, TileID)]>,

    /// The map enemies.
    enemies: Vec<Box<dyn Entity>>,

    /// The player spawn on the map.
    spawn: Vec2,
}

impl MapLayout {
    const MAPS: [MapLayout; 1] = [MapLayout {
        tiles: &[
            (BBox::new(400.0, 701.0, 800.0, 796.0), TileID::VPad),
            (BBox::new(500.0, 405.0, 800.0, 505.0), TileID::Blck),
            (BBox::new(700.0, 505.0, 800.0, 605.0), TileID::Blck),
            (BBox::new(0.0, 700.0, 400.0, 800.0), TileID::Blck),
            (BBox::new(0.0, 0.0, 100.0, 700.0), TileID::Ladr),
            (BBox::new(200.0, 150.0, 400.0, 250.0), TileID::Blck),
            (BBox::new(750.0, 0.0, 800.0, 405.0), TileID::HPad),
        ],
        enemies: &[|| Box::new(Jumper::new(1.0, 1.0))],
        spawn: Vec2::new(350.0, 600.0),
    }];
}

impl Map {
    /// Returns the grid from the given index.
    pub fn get(i: usize) -> Self {
        let m = MapLayout::MAPS[i];

        Self {
            tiles: Box::from(m.tiles),
            enemies: m.enemies.iter().map(|f| f()).collect(),
            spawn: m.spawn,
        }
    }

    /// Returns the map's player spawn location.
    pub fn get_spawn(&self) -> Vec2 {
        self.spawn
    }

    /// Returns the map's tiles.
    pub fn get_tiles(&self) -> &[(BBox, TileID)] {
        &self.tiles
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
