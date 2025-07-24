mod framebuffer;
mod line;

use framebuffer::FrameBuffer;
use line::line;
use raylib::prelude::*;
use std::{thread, time::Duration};

fn main() {
    let window_width = 100;
    let window_height = 100;

    let width = 100;
    let height = 100;
    let background_color = Color::BLACK;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .log_level(TraceLogLevel::LOG_WARNING)
        .title("Line Drawing Example")
        .build();

    let mut framebuffer = FrameBuffer::new(width, height, background_color);

    let mut translate_x = 0.0;
    let mut translate_y = 0.0;

    while !window.window_should_close() {
        if translate_x > 400.0 {
            translate_x = 0.0;
            translate_y = 0.0;
        }
        translate_x += 1.0;
        translate_y += 1.0;

        // framebuffer.clear();

        render(&mut framebuffer, translate_x, translate_y);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}

fn render(framebuffer: &mut FrameBuffer, translate_x: f32, translate_y: f32) {
    framebuffer.set_color(Color::GREEN);

    line(
        framebuffer,
        Vector2::new(100.0 + translate_x, 100.0 + translate_y),
        Vector2::new(700.0 + translate_x, 500.0 + translate_y),
    );

    framebuffer.set_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(100.0 + translate_x, 500.0 + translate_y),
        Vector2::new(700.0 + translate_x, 100.0 + translate_y),
    );
}
