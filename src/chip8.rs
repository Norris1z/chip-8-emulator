use std::fmt::{Display, Formatter, Result};

pub struct Chip8 {
    pub memory: [u8; 0xFFF],
    pub registers: [u8; 16],
    pub index: u16,
    pub pc: u16,
    pub sp: u8,
    pub stack: [u16; 16],
}

pub struct OpCode {
    code: u16,
    decoded: u16,
    data: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 0xFFF],
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn get_next_opcode(&self) -> OpCode {
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        OpCode {
            code: opcode,
            decoded: opcode & 0xF000,
            data: opcode & 0x0FFF,
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
