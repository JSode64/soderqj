use crate::maps::FaceType;

use super::{
    laser::{Direction, Laser},
    lseg::LSeg,
    map::Map,
    vec2::Vec2,
};
use sdl3::{
    keyboard::Scancode,
    pixels::Color,
    render::{Canvas, FRect},
    video::Window,
    EventPump,
};

#[derive(Clone, Copy)]
pub struct Player {
    /// The player's laser
    laser: Laser,

    /// The player's position
    p: Vec2,

    /// The player's velocity
    v: Vec2,

    /// Tracks whether the player is on the ground or not
    on_ground: bool,
}

impl Player {
    /// Player body size (width and height)
    const S: f32 = 40.0;

    /// Half the player body size
    const HALF_S: f32 = Self::S / 2.0;

    /// Max player x-velocity
    const MAX_VX: f32 = 10.0;

    /// Player x-deccelleration
    const DEC_VX: f32 = 0.75;

    /// Player jump y-velocity
    const JMP_VY: f32 = -20.0;

    /// Player gravity acceleration
    const GRAVITY: f32 = 0.75;

    /// Player color
    const COLOR: Color = Color {
        r: 50,
        g: 150,
        b: 255,
        a: 255,
    };

    /// Constructs a new player
    /// Spawns in the top-center of the window (assuming 1600x900)
    pub fn new() -> Self {
        Self {
            laser: Laser::new_inactive(),
            p: Vec2::new(800.0, 0.0),
            v: Vec2::zero(),
            on_ground: false,
        }
    }

    /// Draws the player
    pub fn draw(&self, cnv: &mut Canvas<Window>) {
        // Draw body
        cnv.set_draw_color(Self::COLOR);
        cnv.fill_rect(FRect::new(
            self.p.x - Self::HALF_S,
            self.p.y - Self::HALF_S,
            Self::S,
            Self::S,
        ))
        .unwrap();

        // Draw laser
        if self.laser.is_active() {
            let mut color = Self::COLOR;
            color.a = self.laser.time;
            cnv.set_draw_color(color);
            cnv.draw_line(self.laser.base, self.laser.end()).unwrap();
        }
    }

    /// Updates the player
    /// Move -> Shoot -> Collide
    pub fn update(&mut self, map: &Map, evp: &EventPump) {
        self.laser.update();
        self.do_movement(evp);
        self.do_shoot(evp);
        self.do_collision(map);
    }

    /// Moves the player
    /// Updates according to user input
    fn do_movement(&mut self, evp: &EventPump) {
        // Get user movement inputs
        let kbs = evp.keyboard_state();
        if kbs.is_scancode_pressed(Scancode::Tab) {
            *self = Self::new();
            return;
        }
        let a = kbs.is_scancode_pressed(Scancode::A);
        let d = kbs.is_scancode_pressed(Scancode::D);
        let s = kbs.is_scancode_pressed(Scancode::Space);

        // Update x-velocity
        if a != d {
            if a {
                self.v.x -= 0.5;
            }
            if d {
                self.v.x += 0.5;
            }
        } else {
            // Deccelerate, setting x-velocity to zero if already slow
            self.v.x -= f32::min(Self::DEC_VX, self.v.x.abs()) * self.v.x.signum();
        }
        self.v.x = self.v.x.clamp(-Self::MAX_VX, Self::MAX_VX);

        // Update y-velocity
        if !self.on_ground {
            self.v.y += Self::GRAVITY;
        } else if s {
            self.p.y += Self::JMP_VY;
            self.v.y = Self::JMP_VY;
            self.on_ground = false;
        }
    }

    /// Handles the user shooting
    pub fn do_shoot(&mut self, evp: &EventPump) {
        // Can't shoot if the laser is already active
        if self.laser.is_active() {
            return;
        }

        // Shoot with the first key that is found down
        let kbs = evp.keyboard_state();
        if kbs.is_scancode_pressed(Scancode::Left) {
            self.laser = Laser::new(self.p, crate::laser::Direction::Left);
        } else if kbs.is_scancode_pressed(Scancode::Right) {
            self.laser = Laser::new(self.p, Direction::Right);
        } else if kbs.is_scancode_pressed(Scancode::Down) {
            self.laser = Laser::new(self.p, Direction::Down);
        } else if kbs.is_scancode_pressed(Scancode::Up) {
            self.laser = Laser::new(self.p, Direction::Up);
        }
    }

    /// Handles the player collision with the map
    fn do_collision(&mut self, map: &Map) {
        let path = LSeg::new(self.p, self.p + self.v);
        let mut end = self.p + self.v;
        self.on_ground = false;

        map.segs_iter().for_each(|&(seg, face)| {
            if seg.hits(&path) {
                match face {
                    FaceType::Floor => {
                        // Floor; clamp, stop falling, prevent vertical movement
                        self.p = seg.closest(self.p + self.v);
                        self.v.y = 0.0;
                        self.on_ground = true;
                        end.y = self.p.y;
                    }
                    FaceType::Wall => {
                        // Wall; stop horizontal movement
                        self.v.x = 0.0;
                        end.x = self.p.x;
                    }
                    FaceType::Ceiling => {
                        // Ceiling; stop vertical movement
                        self.v.y = 0.0;
                        end.y = self.p.y;
                    }
                }
            }
        });

        self.p = end;
    }
}
