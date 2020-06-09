mod chip8;
mod fontset;
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
    for _ in 0..rom.rom_size() {
        emulator.run();
    }
    Ok(())
}
