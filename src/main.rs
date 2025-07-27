mod framebuffer;
mod game_of_life;
mod line;

use framebuffer::FrameBuffer;
use game_of_life::GameOfLife;
use raylib::prelude::*;
use std::{thread, time::Duration};

fn main() {
    let window_size = 500;
    let grid_size = 100;
    let pixel_size = window_size / grid_size;

    let mut gif_frames = Vec::new();
    let capture_gif = true;

    let (mut window, rl) = raylib::init()
        .size(window_size, window_size)
        .title("Conway's Game of Life")
        .build();

    let mut framebuffer = FrameBuffer::new(window_size, window_size, Color::BLACK);
    let mut game: GameOfLife = GameOfLife::new(grid_size as usize, grid_size as usize);

    game.insert_pulsar(44, 44);
    game.insert_r_pentomino(48, 48);
    game.insert_mwss(50, 50);
    game.insert_hwss(56, 44);
    game.insert_lwss(60, 50);
    game.insert_boat(53, 53);
    game.insert_blinker(30, 44);
    game.insert_blinker(58, 44);
    game.insert_blinker(44, 30);
    game.insert_blinker(44, 58);
    game.insert_glider_gun(0, 0);
    game.insert_gosper_glider_gun(60, 0);
    game.insert_gosper_glider_gun(0, 70);
    game.insert_glider_gun(60, 70);

    for x in [5, 25, 45, 65, 85] {
        for y in [5, 25, 45, 65, 85] {
            game.insert_glider(x, y);
            game.insert_beacon(x + 2, y);
            game.insert_toad(x, y + 2);
            game.insert_loaf(x + 3, y + 3);
            game.insert_spaceship(x + 4, y + 4);
        }
    }

    for y in (0..100).step_by(20) {
        game.insert_r_pentomino(2, y + 2);
        game.insert_lwss(2, y + 6);
        game.insert_hwss(96, y + 4);
        game.insert_beehive(96, y + 8);
        game.insert_pulsar(43, y + 20);
    }

    for x in (0..100).step_by(18) {
        game.insert_blinker(x, 2);
        game.insert_tub(x + 2, 6);
        game.insert_acorn(x + 1, 90);
        game.insert_loaf(x + 3, 94);
        game.insert_pulsar(x + 4, 74);
        game.insert_pulsar(x + 4, 34);
    }

    while !window.window_should_close() {
        render(&mut framebuffer, &game, pixel_size);

        if capture_gif {
            gif_frames.push(framebuffer.color_buffer.clone());
        }

        framebuffer.swap_buffers(&mut window, &rl);
        thread::sleep(Duration::from_millis(100));
        game.tick();
    }

    if capture_gif {
        framebuffer
            .export_gif(gif_frames, "output.gif", 10)
            .unwrap();
    }
}

fn render(framebuffer: &mut FrameBuffer, game: &GameOfLife, pixel_size: i32) {
    framebuffer.clear();
    framebuffer.set_color(Color::WHITE);
    for x in 0..game.width {
        for y in 0..game.height {
            if game.cells[x][y] {
                for dx in 0..pixel_size {
                    for dy in 0..pixel_size {
                        framebuffer
                            .set_pixel(x as i32 * pixel_size + dx, y as i32 * pixel_size + dy);
                    }
                }
            }
        }
    }
}
