use super::{
    config::{WIN_H, WIN_W},
    geometry::{Square, Vec2},
    map::TileIter,
};
use sdl3::{pixels::Color, render::Canvas, video::Window};

/// A 2D direction.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// A laser the player can shoot enemies with.
#[derive(Clone, Copy)]
pub struct Laser {
    /// The laser's beggining point.
    beg: Vec2,

    /// The laser's ending point.
    end: Vec2,

    /// The laser's timer.
    time: u8,
}

impl Laser {
    /// Returns a new laser with the given base and direction with a full timer.
    pub fn new(beg: Vec2, dir: Direction, map: TileIter) -> Self {
        Self {
            beg,
            end: Self::get_laser_end(beg, dir, map),
            time: 255,
        }
    }

    /// Returns ann inactive laser.
    pub fn new_inactive() -> Self {
        Self {
            beg: Vec2::zero(),
            end: Vec2::zero(),
            time: 0,
        }
    }

    /// True if the laser is active and hits the given square, else false.
    pub fn hits_square(&self, sqr: &Square) -> bool {
        self.time > 0
            && self.beg.x.min(self.end.x) <= sqr.x + sqr.s
            && self.beg.x.max(self.end.x) >= sqr.x
            && self.beg.y.min(self.end.y) <= sqr.y + sqr.s
            && self.beg.y.max(self.end.y) >= sqr.y
    }

    /// True if the laser is active (timer isn't done), else false.
    pub fn is_active(&self) -> bool {
        self.time != 0
    }

    /// Draws the laser.
    pub fn draw(&self, cnv: &mut Canvas<Window>) {
        if self.is_active() {
            cnv.set_draw_color(Color {
                r: 255,
                g: 255,
                b: 255,
                a: self.time,
            });
            cnv.draw_line(self.beg, self.end).unwrap();
        }
    }

    /// Updates the laser.
    pub fn update(&mut self) {
        if self.time > 0 {
            self.time -= 15;
        }
    }

    /// Returns the laser's end based on its start, end, and direction.
    /// Stops the laser short from passing through tiles.
    fn get_laser_end(beg: Vec2, dir: Direction, map: TileIter) -> Vec2 {
        let mut result = match dir {
            Direction::Left => Vec2::new(0.0, beg.y),
            Direction::Right => Vec2::new(WIN_W as _, beg.y),
            Direction::Up => Vec2::new(beg.x, 0.0),
            Direction::Down => Vec2::new(beg.x, WIN_H as _),
        };

        for (bbox, _) in map {
            match dir {
                Direction::Left => {
                    if bbox.y <= beg.y && beg.y <= bbox.b && bbox.a < beg.x {
                        result.x = result.x.max(bbox.a);
                    }
                }
                Direction::Right => {
                    if bbox.y <= beg.y && beg.y <= bbox.b && bbox.x > beg.x {
                        result.x = result.x.min(bbox.x);
                    }
                }
                Direction::Up => {
                    if bbox.x <= beg.x && beg.x <= bbox.a && bbox.b < beg.y {
                        result.y = result.y.max(bbox.b);
                    }
                }
                Direction::Down => {
                    if bbox.x <= beg.x && beg.x <= bbox.a && bbox.y > beg.y {
                        result.y = result.y.min(bbox.y);
                    }
                }
            }
        }

        result
    }
}
