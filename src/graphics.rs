use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;

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
