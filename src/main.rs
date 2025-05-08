mod config;
mod enemies;
mod entity;
mod geometry;
mod laser;
mod map;
mod player;
mod tile;

use config::{WIN_H, WIN_W};
use enemies::{draw_enemies, update_enemies};
use entity::Entity;
use map::Map;
use sdl3::{event::Event, pixels::Color, render::BlendMode};
use std::time::{Duration, Instant};

fn main() {
    // Set up SDL3 window and renderer.
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

    // Framerate limiting (60fps).
    let gap = Duration::from_millis(16);
    let mut prv = Instant::now();

    // Prepare game state.
    let mut i = 0;
    let (mut m, mut p, mut e) = Map::init_game(i);

    loop {
        // Stop running if the window was closed.
        if evp.poll_iter().any(|e| matches!(e, Event::Quit { .. })) {
            break;
        }

        // Update game state.
        let kbs = evp.keyboard_state();

        p.update(Some(&kbs), m.tile_iter());
        update_enemies(&mut e, &p, m.tile_iter());
        p.do_enemy_check(&e);
        m.update(&kbs, &mut p, &mut e);

        // If no enemies left, go to the next map.
        if e.is_empty() {
            i = (i + 1) % Map::N;
            (m, p, e) = Map::init_game(i);
        }

        // Draw game state.
        m.draw(&mut cnv);
        p.draw(&mut cnv);
        draw_enemies(&mut e, &mut cnv);

        // Present.
        cnv.present();
        cnv.set_draw_color(Color::BLACK);
        cnv.clear();

        // Wait for frame.
        while Instant::now() - prv < gap {}
        prv = Instant::now();
    }
}
