use crate::chip8::{Chip8, OpCode};
use std::collections::HashMap;

fn get_vx_and_vy(opcode: &OpCode) -> (usize, usize) {
    let vx = (opcode.code & 0x0F00) >> 8;
    let vy = (opcode.code & 0x00F0) >> 4;
    (vx as usize, vy as usize)
}

fn get_vx_and_number(opcode: &OpCode) -> (usize, u8) {
    let vx = (opcode.code & 0x0F00) >> 8;
    let number = opcode.code & 0x00FF;
    (vx as usize, number as u8)
}

fn call_subroutine(cpu: &mut Chip8, opcode: OpCode) {
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.sp += 1;
    cpu.pc = opcode.data;
}

fn clear_screen(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Clearing screen");
}

fn return_from_subroutine(cpu: &mut Chip8, _opcode: OpCode) {
    cpu.sp -= 1;
    cpu.pc = cpu.stack[cpu.sp as usize];
}

fn jump_to_address(cpu: &mut Chip8, opcode: OpCode) {
    cpu.pc = opcode.data;
}

fn jump_if_reg_values_are_equal(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    if cpu.registers[vx as usize] == cpu.registers[vy as usize] {
        cpu.pc += 2;
    }
}

fn jump_if_reg_value_is_equal_to_number(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, number) = get_vx_and_number(&opcode);
    if cpu.registers[vx as usize] == number as u8 {
        cpu.pc += 2;
    }
}

fn jump_if_reg_value_is_not_equal_to_number(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, number) = get_vx_and_number(&opcode);
    if cpu.registers[vx as usize] != number as u8 {
        cpu.pc += 2;
    }
}

fn jump_if_reg_values_are_not_equal(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    if cpu.registers[vx as usize] != cpu.registers[vy as usize] {
        cpu.pc += 2;
    }
}

fn store_number_in_register(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, number) = get_vx_and_number(&opcode);
    cpu.registers[vx as usize] = number as u8;
}

fn copy_register_value(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx as usize] = cpu.registers[vy as usize];
}

fn add_to_register_and_ignore_carry_flag(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Add to register and ignore carry flag");
}

fn bitwise_or_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx as usize] |= cpu.registers[vy as usize];
}

fn bitwise_and_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx as usize] &= cpu.registers[vy as usize];
}

fn bitwise_xor_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx as usize] ^= cpu.registers[vy as usize];
}

fn bitwise_shif_right_and_store(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Bitwise shift right and store");
}

fn add_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx as usize] += cpu.registers[vy as usize];
}

fn subtract_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx as usize] -= cpu.registers[vy as usize];
}

fn subtract_and_store_and_set_vf(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("SUBTRACT and store and set VF");
}

fn store_msb_and_left_shift(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Store MSB and left shift");
}

fn set_index_to_mem_address(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Set I to mem address");
}

fn jump_to_address_plus_v0(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Jump to address plus V0");
}

fn bitwise_on_a_random_number_and_store(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Bitwise random number and store");
}

fn draw_sprite(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Draw Sprite");
}

fn jump_if_key_is_pressed(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Jump if key is pressed");
}

fn jump_if_key_is_not_pressed(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Jump if key is not pressed");
}

fn get_delay_timer_and_set_to_vx(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Get delay timer and set to VX");
}

fn set_delay_timer_to_vx(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Set delay timer to VX");
}

fn set_sound_timer_to_vx(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Set Sound timer to VX");
}

fn get_key_press_and_store(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Get and store keypress");
}

fn add_vx_to_i_and_set_overflow(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Add VX to I and set Overflow");
}

fn set_i_to_location_of_sprite_in_vx(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Set I to location of sprite in VX");
}

fn store_binary_coded_decimal_representaion_of_vx(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Store BCD");
}

fn store_v0_to_vx_starting_at_address_i(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Store v0 to vx");
}

fn fill_v0_to_vx_starting_at_address_i(_cpu: &mut Chip8, _opcode: OpCode) {
    println!("Fill V0 to VX");
}

pub fn create_opcode_instructions_map() -> HashMap<u16, fn(&mut Chip8, OpCode)> {
    let mut map: HashMap<u16, fn(&mut Chip8, OpCode)> = HashMap::new();
    map.insert(0x00E0, clear_screen);
    map.insert(0x00EE, return_from_subroutine);
    map.insert(0x1000, jump_to_address);
    map.insert(0x2000, call_subroutine);
    map.insert(0x3000, jump_if_reg_value_is_equal_to_number);
    map.insert(0x4000, jump_if_reg_value_is_not_equal_to_number);
    map.insert(0x5000, jump_if_reg_values_are_equal);
    map.insert(0x6000, store_number_in_register);
    map.insert(0x7000, add_to_register_and_ignore_carry_flag);
    map.insert(0x8000, copy_register_value);
    map.insert(0x8001, bitwise_or_and_store);
    map.insert(0x8002, bitwise_and_and_store);
    map.insert(0x8003, bitwise_xor_and_store);
    map.insert(0x8004, add_and_store);
    map.insert(0x8005, subtract_and_store);
    map.insert(0x8006, bitwise_shif_right_and_store);
    map.insert(0x8007, subtract_and_store_and_set_vf);
    map.insert(0x800E, store_msb_and_left_shift);
    map.insert(0x9000, jump_if_reg_values_are_not_equal);
    map.insert(0xA000, set_index_to_mem_address);
    map.insert(0xB000, jump_to_address_plus_v0);
    map.insert(0xC000, bitwise_on_a_random_number_and_store);
    map.insert(0xD000, draw_sprite);
    map.insert(0xE09E, jump_if_key_is_pressed);
    map.insert(0xE0A1, jump_if_key_is_not_pressed);
    map.insert(0xF007, get_delay_timer_and_set_to_vx);
    map.insert(0xF00A, get_key_press_and_store);
    map.insert(0xF015, set_delay_timer_to_vx);
    map.insert(0xF018, set_sound_timer_to_vx);
    map.insert(0xF01E, add_vx_to_i_and_set_overflow);
    map.insert(0xF029, set_i_to_location_of_sprite_in_vx);
    map.insert(0xF033, store_binary_coded_decimal_representaion_of_vx);
    map.insert(0xF055, store_v0_to_vx_starting_at_address_i);
    map.insert(0xF065, fill_v0_to_vx_starting_at_address_i);
    map
}
