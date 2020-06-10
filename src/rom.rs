use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

const ROM_MEMORY: usize = 0xFFF - 0x200;

pub struct Rom {
    bytes: Vec<u8>,
}

impl Rom {
    pub fn new(file: &str) -> Result<Rom, Box<dyn Error>> {
        let mut f = File::open(file)?;
        let mut buffer: Vec<u8> = Vec::with_capacity(ROM_MEMORY);
        f.read_to_end(&mut buffer)?;
        Ok(Rom { bytes: buffer })
    }

    pub fn load_into_memory(&self, memory: &mut [u8; 0xFFF]) {
        for (index, &byte) in self.bytes.iter().enumerate() {
            memory[0x200 + index] = byte;
        }
    }
}
