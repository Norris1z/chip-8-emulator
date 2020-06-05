mod rom;
use rom::Rom;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut memory = [0; 0xFFF];
    let rom = Rom::new("chip8-roms/TETRIS")?;
    rom.load_into_memory(&mut memory);
    Ok(())
}
