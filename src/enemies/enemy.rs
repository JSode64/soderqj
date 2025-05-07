use super::super::{entity::Entity, map::TileIter, player::Player};
use sdl3::{render::Canvas, video::Window};

/// A vector containing enemies.
pub type EnemyVec = Vec<Box<dyn Entity>>;

/// Updates the enemies.
pub fn update_enemies(e: &mut EnemyVec, p: &Player, map: TileIter) {
    // Update enemies.
    e.iter_mut().for_each(|e| e.update(None, map.clone()));

    // Delete enemies that are killed by the laser.
    e.retain(|e| !p.get_laser().hits_square(&e.get_body()));
}

/// Draws the enemies.
pub fn draw_enemies(e: &mut EnemyVec, cnv: &mut Canvas<Window>) {
    e.into_iter().for_each(|e| e.draw(cnv));
}
