use super::{
    lseg::LSeg,
    maps::{FaceType, MAPS},
};
use sdl3::{keyboard::Scancode, pixels::Color, render::Canvas, video::Window, EventPump};
use std::slice::Iter;

#[derive(Clone)]
pub struct Map {
    /// A slice of all map triangles
    segs: &'static [(LSeg, FaceType)],

    /// The map index
    i: usize,
}

impl Map {
    /// Creates a new map
    pub fn new(i: usize) -> Self {
        Self { segs: MAPS[i], i }
    }

    /// Draws the map
    pub fn draw(&self, cnv: &mut Canvas<Window>) {
        cnv.set_draw_color(Color::WHITE);
        self.segs
            .iter()
            .for_each(|(seg, _)| cnv.draw_line(seg.a, seg.b).unwrap());
    }

    /// Returns an iterator over the map's triangles
    pub fn segs_iter(&self) -> Iter<'_, (LSeg, FaceType)> {
        self.segs.iter()
    }

    /// Updates the map
    pub fn update(&mut self, evp: &EventPump) {
        if evp.keyboard_state().is_scancode_pressed(Scancode::R) {
            *self = Self::new((self.i + 1) % MAPS.len());
        }
    }
}
