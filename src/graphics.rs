use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SCALE_FACTOR: u32 = 10;

pub fn create_window(title: &str, context: &Sdl) -> Result<WindowCanvas, Box<dyn Error>> {
    let video = context.video()?;
    let window = video
        .window(
            title,
            SCREEN_WIDTH as u32 * SCALE_FACTOR,
            SCREEN_HEIGHT as u32 * SCALE_FACTOR,
        )
        .position_centered()
        .build()?;
    Ok(window.into_canvas().build()?)
}

pub fn create_default_screen(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();
}

pub fn update_screen(canvas: &mut WindowCanvas, video_buffer: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
    canvas.clear();
    for (y, pixels) in video_buffer.iter().enumerate() {
        for (x, &pixel) in pixels.iter().enumerate() {
            let x = (x as u32) * SCALE_FACTOR;
            let y = (y as u32) * SCALE_FACTOR;

            if pixel == 1 {
                canvas.set_draw_color(Color::WHITE);
            } else {
                canvas.set_draw_color(Color::BLACK);
            }
            let _ = canvas.fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
        }
    }
    canvas.present();
}
