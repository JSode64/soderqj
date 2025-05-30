use super::{
    config::GRAVITY,
    enemies::EnemyVec,
    entity::Entity,
    geometry::{Square, Vec2},
    laser::{Direction, Laser},
    map::TileIter,
};
use sdl3::{
    keyboard::{KeyboardState, Scancode},
    pixels::Color,
    render::Canvas,
    video::Window,
};

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

    /// Tracks whether the player is dead or not.
    is_alive: bool,
}

impl Player {
    /// Player body size (width and height).
    pub const S: f32 = 32.0;

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

    /// Constructs a new player with the given position.
    pub fn new(p: Vec2) -> Self {
        Self {
            laser: Laser::new_inactive(),
            body: Square::new(p.x, p.y, Self::S),
            v: Vec2::zero(),
            on_ground: false,
            is_alive: true,
        }
    }

    /// Returns a reference to the player's laser.
    pub fn get_laser(&self) -> &Laser {
        &self.laser
    }

    /// Updates the player's living status based on the given enemies.
    pub fn do_enemy_check(&mut self, es: &EnemyVec) {
        // If the player collides with an enemies, kill the player.
        for e in es {
            if e.get_body().collides_with(&self.body) {
                self.kill();
                break;
            }
        }
    }

    /// Updates the player's velocity based on user input.
    fn do_movement(&mut self, kbs: &KeyboardState) {
        // Get user movement inputs
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
    fn do_shoot(&mut self, kbs: &KeyboardState, map: TileIter) {
        // Can't shoot if the laser is already active.
        if self.laser.is_active() {
            return;
        }

        // Shoot with the first key that is found down.
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

    fn is_alive(&self) -> bool {
        self.is_alive
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

    fn kill(&mut self) {
        self.is_alive = false;
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
        cnv.fill_rect(&self.body).unwrap();
    }

    fn update(&mut self, evp: Option<&KeyboardState>, map: TileIter) {
        self.laser.update();
        self.do_movement(evp.unwrap());
        self.do_shoot(evp.unwrap(), map.clone());
        self.do_map_collision(map);
    }
}
