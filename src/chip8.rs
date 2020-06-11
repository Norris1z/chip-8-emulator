use crate::graphics;
use crate::instructions;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

pub struct Chip8 {
    pub memory: [u8; 0xFFF],
    pub registers: [u8; 16],
    pub keys: [bool; 16],
    pub index: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub instruction_fns: HashMap<u16, fn(&mut Chip8, OpCode)>,
    pub update_display: bool,
    pub video: [u32; graphics::VIDEO_BUFFER_SIZE],
}

pub struct OpCode {
    pub code: u16,
    pub decoded: u16,
    pub data: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 0xFFF],
            registers: [0; 16],
            keys: [false; 16],
            index: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            instruction_fns: instructions::create_opcode_instructions_map(),
            update_display: false,
            video: [0; graphics::VIDEO_BUFFER_SIZE],
        }
    }

    pub fn get_next_opcode(&mut self) -> OpCode {
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;
        OpCode {
            code: opcode,
            decoded: opcode & 0xF000,
            data: opcode & 0x0FFF,
        }
    }
    pub fn load_font_set(&mut self, fontset: [u8; 80]) {
        for offset in 0..80 {
            self.memory[0x050 + offset] = fontset[offset];
        }
    }

    pub fn run(&mut self) {
        let opcode = self.get_next_opcode();
        let handler_key = match opcode.decoded {
            0x0000 | 0xE000 | 0xF000 => opcode.code & 0xF0FF,
            0x8000 => opcode.code & 0xF00F,
            code => code,
        };
        if let Some(handler) = self.instruction_fns.get(&handler_key) {
            handler(self, opcode);
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "code: 0x{:X?}, decoded: 0x{:X?} data: 0x{:X?}",
            self.code, self.decoded, self.data
        )
    }
}
