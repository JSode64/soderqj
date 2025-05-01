use super::geometry::Vec2;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub struct Laser {
    /// The laser's base
    pub base: Vec2,

    /// The laser's direction
    pub dir: Direction,

    /// The laser's timer
    pub time: u8,
}

impl Laser {
    /// Returns a new laser with the given base and direction with a full timer
    pub fn new(base: Vec2, dir: Direction) -> Self {
        Self {
            base,
            dir,
            time: 255,
        }
    }

    /// Returns ann inactive laser
    pub fn new_inactive() -> Self {
        Self {
            base: Vec2::zero(),
            dir: Direction::Left,
            time: 0,
        }
    }

    /// Returns the end of the laser
    pub fn end(&self) -> Vec2 {
        match self.dir {
            Direction::Left => Vec2::new(0.0, self.base.y),
            Direction::Right => Vec2::new(1600.0, self.base.y),
            Direction::Down => Vec2::new(self.base.x, 900.0),
            Direction::Up => Vec2::new(self.base.x, 0.0),
        }
    }

    /// Updates the laser
    pub fn update(&mut self) {
        if self.time > 0 {
            self.time -= 15;
        }
    }

    /// True if the laser is active (timer isn't dont), else false
    pub fn is_active(&self) -> bool {
        self.time != 0
    }
}
