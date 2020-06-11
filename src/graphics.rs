use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

pub const VIDEO_BUFFER_SIZE: usize = 64 * 32;
pub const SCREEN_WIDTH: u8 = 64;
pub const SCREEN_HEIGHT: u8 = 32;

pub fn create_window(title: &str, context: &Sdl) -> Result<WindowCanvas, Box<dyn Error>> {
    let video = context.video()?;
    let window = video.window(title, 640, 320).position_centered().build()?;
    Ok(window.into_canvas().build()?)
}

pub fn create_default_screen(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
}

pub fn update_screen(_canvas: &mut WindowCanvas, _video_buffer: [u32; VIDEO_BUFFER_SIZE]) {}
