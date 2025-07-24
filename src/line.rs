use crate::framebuffer::FrameBuffer;
use raylib::prelude::*;

pub fn line(framebuffer: &mut FrameBuffer, start: Vector2, end: Vector2) {
    let x1 = start.x as i32;
    let y1 = start.y as i32;
    let x2 = end.x as i32;
    let y2 = end.y as i32;

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        framebuffer.set_pixel(x, y);
        if x == x2 && y == y2 {
            break;
        }
        let err2 = err * 2;
        if err2 > -dy {
            err -= dy;
            x += sx;
        }
        if err2 < dx {
            err += dx;
            y += sy;
        }
    }
}
