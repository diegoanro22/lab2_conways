use gif::{Encoder, Frame, Repeat};
use raylib::prelude::*;
use std::fs::File;

pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        FrameBuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, color);
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn render_to_file(&self, file_path: &str) -> Result<(), String> {
        self.color_buffer.export_image(file_path);
        Ok(())
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            let mut d = window.begin_drawing(raylib_thread);
            d.clear_background(self.background_color); // Opcional
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        } else {
            eprintln!("Failed to load texture from image.");
        }
    }

    pub fn export_gif(
        &self,
        frames: Vec<Image>,
        path: &str,
        delay_centis: u16,
    ) -> Result<(), String> {
        let mut image_file = File::create(path).map_err(|e| e.to_string())?;
        let mut encoder = Encoder::new(&mut image_file, self.width as u16, self.height as u16, &[])
            .map_err(|e| e.to_string())?;
        encoder
            .set_repeat(Repeat::Infinite)
            .map_err(|e| e.to_string())?;

        for img in frames {
            let raw = img.get_image_data();
            let pixels = img
                .get_image_data()
                .iter()
                .flat_map(|color| vec![color.r, color.g, color.b])
                .collect::<Vec<u8>>();

            let mut frame = Frame::from_rgb(self.width as u16, self.height as u16, &pixels);
            frame.delay = delay_centis;
            encoder.write_frame(&frame).map_err(|e| e.to_string())?;
        }

        Ok(())
    }
}
