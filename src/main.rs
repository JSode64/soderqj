mod laser;
mod lseg;
mod map;
mod maps;
mod player;
mod tri;
mod vec2;

use map::Map;
use player::Player;
use sdl3::{event::Event, pixels::Color, render::BlendMode};
use std::time::{Duration, Instant};

fn main() {
    // Set up SDL3 window and renderer
    let sdl = sdl3::init().unwrap();
    let vss = sdl.video().unwrap();
    let win = vss
        .window("SoderqJ", 1600, 900)
        .position_centered()
        .build()
        .unwrap();
    let mut cnv = win.into_canvas();
    let mut evp = sdl.event_pump().unwrap();
    cnv.set_blend_mode(BlendMode::Blend);

    // Framerate limiting (60fps)
    let gap = Duration::from_millis(16);
    let mut prv = Instant::now();

    let mut p = Player::new();
    let mut map = Map::new(0);

    loop {
        // Stop running if the window was closed
        if evp.poll_iter().any(|e| matches!(e, Event::Quit { .. })) {
            break;
        }

        map.update(&evp);
        p.update(&map, &evp);
        map.draw(&mut cnv);
        p.draw(&mut cnv);

        // Present and wait
        cnv.present();
        cnv.set_draw_color(Color::BLACK);
        cnv.clear();
        while Instant::now() - prv < gap {}
        prv = Instant::now();
    }
}
