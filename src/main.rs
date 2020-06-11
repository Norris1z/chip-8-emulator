mod chip8;
mod fontset;
mod graphics;
mod input;
mod instructions;
mod rom;
use chip8::Chip8;
use rom::Rom;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut emulator = Chip8::new();
    let rom = Rom::new("chip8-roms/TETRIS")?;
    rom.load_into_memory(&mut emulator.memory);
    emulator.load_font_set(fontset::FONT_SET);
    let context = sdl2::init()?;
    let mut canvas = graphics::create_window("chip8 emulator", &context)?;
    graphics::create_default_screen(&mut canvas);
    let mut events = context.event_pump()?;
    'emulator_loop: loop {
        let quit = input::capture(&mut events, &mut emulator);
        if quit {
            break 'emulator_loop;
        }
        emulator.run();
        if emulator.update_display {
            emulator.update_display = false;
            graphics::update_screen(&mut canvas, emulator.video);
        }
    }
    Ok(())
}
