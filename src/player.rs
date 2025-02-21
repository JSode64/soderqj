use super::{
    laser::{Direction, Laser},
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

    /// The player's body
    body: FRect,

    /// The player's x-velocity
    vx: f32,

    /// The player's y-velocity
    vy: f32,

    /// True if the player is on the ground, else false
    on_ground: bool,
}

impl Player {
    /// Player body size (width and height)
    const S: f32 = 40.0;

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
            body: FRect::new(780.0, 0.0, Self::S, Self::S),
            vx: 0.0,
            vy: 0.0,
            on_ground: false,
        }
    }

    /// Draws the player
    pub fn draw(&self, cnv: &mut Canvas<Window>) {
        // Draw body
        cnv.set_draw_color(Self::COLOR);
        cnv.fill_rect(self.body).unwrap();

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
        let a = kbs.is_scancode_pressed(Scancode::A);
        let d = kbs.is_scancode_pressed(Scancode::D);
        let s = kbs.is_scancode_pressed(Scancode::Space);

        // Update x-velocity
        if a != d {
            if a {
                self.vx -= 0.5;
            }
            if d {
                self.vx += 0.5;
            }
        } else {
            // Deccelerate, setting x-velocity to zero if already slow
            self.vx -= f32::min(Self::DEC_VX, self.vx.abs()) * self.vx.signum();
        }
        self.vx = self.vx.clamp(-Self::MAX_VX, Self::MAX_VX);

        // Update y-velocity
        if s && self.on_ground {
            self.vy = Self::JMP_VY;
            self.on_ground = false;
        }
        if !self.on_ground {
            self.vy += Self::GRAVITY;
        }

        // Move
        self.body.x += self.vx;
        self.body.y += self.vy;
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
            self.laser = Laser::new(self.center(), crate::laser::Direction::Left);
        } else if kbs.is_scancode_pressed(Scancode::Right) {
            self.laser = Laser::new(self.center(), Direction::Right);
        } else if kbs.is_scancode_pressed(Scancode::Down) {
            self.laser = Laser::new(self.center(), Direction::Down);
        } else if kbs.is_scancode_pressed(Scancode::Up) {
            self.laser = Laser::new(self.center(), Direction::Up);
        }
    }

    /// Handles the player collision with the map
    fn do_collision(&mut self, map: &Map) {
        let c = self.center();
        self.on_ground = false;
        if let Some(tri) = map.tri_iter().find(|&tri| tri.contains_point(c)) {
            let p = tri.closest_to_point(c);

            // Handle grounding
            if p.y < c.y {
                self.on_ground = true;
                self.vy = 0.0;
            } else if self.vy < 0.0 {
                self.vy = 0.0;
            }

            // Move to be centered at the point
            self.body.x = p.x - (Self::S / 2.0);
            self.body.y = p.y - (Self::S / 2.0);
        }
    }

    /// Returns the player's center point
    fn center(&self) -> Vec2 {
        Vec2::new(self.body.x + (Self::S / 2.0), self.body.y + (Self::S / 2.0))
    }
}
