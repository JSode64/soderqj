use super::tri::Tri;
use sdl3::{pixels::Color, render::Canvas, video::Window};
use std::slice::Iter;

#[derive(Clone)]
pub struct Map {
    /// A slice of all map triangles
    tris: Box<[Tri]>,
}

impl Map {
    /// Creates a new map
    /// TODO:
    pub fn new() -> Self {
        Self {
            tris: vec![
                Tri::new_xy(0.0, 0.0, 800.0, 225.0, 0.0, 600.0),
                Tri::new_xy(0.0, 450.0, 1600.0, 450.0, 800.0, 900.0),
            ]
            .into_boxed_slice(),
        }
    }

    /// Draws the map
    pub fn draw(&self, cnv: &mut Canvas<Window>) {
        cnv.set_draw_color(Color::WHITE);
        for tri in self.tris.iter() {
            cnv.draw_line(tri.a, tri.b).unwrap();
            cnv.draw_line(tri.b, tri.c).unwrap();
            cnv.draw_line(tri.c, tri.a).unwrap();
        }
    }

    /// Returns an iterator over the map's triangles
    pub fn tri_iter(&self) -> Iter<'_, Tri> {
        self.tris.iter()
    }
}
