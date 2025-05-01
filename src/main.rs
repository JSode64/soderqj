mod config;
mod enemies;
mod entity;
mod geometry;
mod laser;
mod map;
mod player;
mod tile;

use config::{WIN_H, WIN_W};
use entity::Entity;
use map::Map;
use player::Player;
use sdl3::{event::Event, pixels::Color, render::BlendMode};
use std::time::{Duration, Instant};

fn main() {
    // Set up SDL3 window and renderer
    let sdl = sdl3::init().unwrap();
    let vss = sdl.video().unwrap();
    let win = vss
        .window("SoderqJ", WIN_W as _, WIN_H as _)
        .position_centered()
        .build()
        .unwrap();
    let mut cnv = win.into_canvas();
    let mut evp = sdl.event_pump().unwrap();
    cnv.set_blend_mode(BlendMode::Blend);

    // Framerate limiting (60fps)
    let gap = Duration::from_millis(16);
    let mut prv = Instant::now();

    let mut m = Map::get(0);
    let mut p = Player::new(&m);

    loop {
        // Stop running if the window was closed
        if evp.poll_iter().any(|e| matches!(e, Event::Quit { .. })) {
            break;
        }

        m.update(&evp);
        p.update(&evp, m.get_tiles());

        m.draw(&mut cnv);
        p.draw(&mut cnv);

        // Present
        cnv.present();
        cnv.set_draw_color(Color::BLACK);
        cnv.clear();

        // Wait for frame
        while Instant::now() - prv < gap {}
        prv = Instant::now();
    }
}
