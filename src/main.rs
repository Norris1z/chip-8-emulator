mod chip8;
mod rom;
use chip8::Chip8;
use rom::Rom;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut emulator = Chip8::new();
    let rom = Rom::new("chip8-roms/TETRIS")?;
    rom.load_into_memory(&mut emulator.memory);
    for _ in 0..rom.rom_size() {
        println!(
            "pc {} --- opcode {}",
            emulator.pc,
            emulator.get_next_opcode()
        );
        emulator.pc += 2;
    }
    Ok(())
}
