use super::{
    config::GRAVITY,
    entity::Entity,
    geometry::{BBox, Square, Vec2},
    laser::{Direction, Laser},
    map::Map,
    tile::TileID,
};
use sdl3::{keyboard::Scancode, pixels::Color, render::Canvas, video::Window, EventPump};

#[derive(Clone, Copy)]
pub struct Player {
    /// The player's laser.
    laser: Laser,

    /// The player's body.
    body: Square,

    /// The player's velocity.
    v: Vec2,

    /// Tracks whether the player is on the ground or not.
    on_ground: bool,
}

impl Player {
    /// Player body size (width and height).
    const S: f32 = 32.0;

    /// Max player x-velocity.
    const MAX_VX: f32 = 10.0;

    /// Player x-deccelleration.
    const DEC_VX: f32 = 0.75;

    /// Player jump y-velocity.
    const JMP_VY: f32 = -18.0;

    /// Player color.
    const COLOR: Color = Color {
        r: 50,
        g: 150,
        b: 255,
        a: 255,
    };

    /// Constructs a new player.
    pub fn new(map: &Map) -> Self {
        Self {
            laser: Laser::new_inactive(),
            body: Square::new(map.get_spawn().x, map.get_spawn().y, Self::S),
            v: Vec2::zero(),
            on_ground: false,
        }
    }

    /// Moves the player.
    /// Updates according to user input.
    fn do_movement(&mut self, evp: &EventPump) {
        // Get user movement inputs
        let kbs = evp.keyboard_state();
        let a = kbs.is_scancode_pressed(Scancode::A);
        let d = kbs.is_scancode_pressed(Scancode::D);
        let s = kbs.is_scancode_pressed(Scancode::Space);

        // Update x-velocity.
        if a != d {
            if a && self.v.x > -Self::MAX_VX {
                self.v.x = (self.v.x - 0.5).max(-Self::MAX_VX);
            }
            if d && self.v.x < Self::MAX_VX {
                self.v.x = (self.v.x + 0.5).min(Self::MAX_VX);
            }
        } else {
            self.v.x -= f32::min(Self::DEC_VX, self.v.x.abs()) * self.v.x.signum();
        }

        // Update y-velocity.
        self.v.y += GRAVITY;
        if self.on_ground && s {
            self.v.y = Self::JMP_VY;
            self.on_ground = false;
        }
    }

    /// Handles the user shooting.
    pub fn do_shoot(&mut self, evp: &EventPump, map: &[(BBox, TileID)]) {
        // Can't shoot if the laser is already active.
        if self.laser.is_active() {
            return;
        }

        // Shoot with the first key that is found down.
        let kbs = evp.keyboard_state();

        if kbs.is_scancode_pressed(Scancode::Left) {
            self.laser = Laser::new(self.body.center(), Direction::Left, map);
        } else if kbs.is_scancode_pressed(Scancode::Right) {
            self.laser = Laser::new(self.body.center(), Direction::Right, map);
        } else if kbs.is_scancode_pressed(Scancode::Down) {
            self.laser = Laser::new(self.body.center(), Direction::Down, map);
        } else if kbs.is_scancode_pressed(Scancode::Up) {
            self.laser = Laser::new(self.body.center(), Direction::Up, map);
        }
    }
}

impl Entity for Player {
    fn get_body(&self) -> Square {
        self.body
    }

    fn get_v(&self) -> Vec2 {
        self.v
    }

    fn get_color(&self) -> Color {
        Self::COLOR
    }

    fn set_on_ground(&mut self, b: bool) {
        self.on_ground = b;
    }

    fn set_pos(&mut self, p: Vec2) {
        self.body.x = p.x;
        self.body.y = p.y;
    }

    fn set_vx(&mut self, v: f32) {
        self.v.x = v;
    }

    fn set_vy(&mut self, v: f32) {
        self.v.y = v;
    }

    fn on_col_x(&mut self) {
        self.v.x = 0.0;
    }

    fn on_col_y(&mut self) {
        self.v.y = 0.0;
    }

    fn draw(&self, cnv: &mut Canvas<Window>) {
        // Draw laser.
        self.laser.draw(cnv);

        // Draw player.
        cnv.set_draw_color(self.get_color());
        cnv.fill_rect(self.get_body()).unwrap();
    }

    fn update(&mut self, evp: &EventPump, map: &[(BBox, TileID)]) {
        self.laser.update();
        self.do_movement(evp);
        self.do_shoot(evp, map);
        self.do_map_collision(map);
    }
}
