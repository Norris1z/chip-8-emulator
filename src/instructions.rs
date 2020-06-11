use crate::chip8::{Chip8, OpCode};
use crate::graphics;
use rand::prelude::*;
use std::collections::HashMap;

const VF: usize = 0xF;

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

fn clear_screen(cpu: &mut Chip8, _opcode: OpCode) {
    cpu.video = [0; graphics::VIDEO_BUFFER_SIZE];
    cpu.update_display = true;
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
    if cpu.registers[vx] == cpu.registers[vy] {
        cpu.pc += 2;
    }
}

fn jump_if_reg_value_is_equal_to_number(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, number) = get_vx_and_number(&opcode);
    if cpu.registers[vx] == number {
        cpu.pc += 2;
    }
}

fn jump_if_reg_value_is_not_equal_to_number(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, number) = get_vx_and_number(&opcode);
    if cpu.registers[vx] != number {
        cpu.pc += 2;
    }
}

fn jump_if_reg_values_are_not_equal(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    if cpu.registers[vx] != cpu.registers[vy] {
        cpu.pc += 2;
    }
}

fn store_number_in_register(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, number) = get_vx_and_number(&opcode);
    cpu.registers[vx] = number;
}

fn copy_register_value(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx] = cpu.registers[vy];
}

fn add_to_register_and_ignore_carry_flag(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, number) = get_vx_and_number(&opcode);
    cpu.registers[vx] = cpu.registers[vx].wrapping_add(number);
}

fn bitwise_or_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx] |= cpu.registers[vy];
}

fn bitwise_and_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx] &= cpu.registers[vy];
}

fn bitwise_xor_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    cpu.registers[vx] ^= cpu.registers[vy];
}

fn bitwise_shif_right_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    cpu.registers[vx] >>= 1;
}

fn add_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    let sum = cpu.registers[vx] as u16 + cpu.registers[vy] as u16;

    if sum > std::u8::MAX as u16 {
        cpu.registers[VF] = 1;
    } else {
        cpu.registers[VF] = 0;
    }
    cpu.registers[vx] = (sum & 0xFF) as u8;
}

fn subtract_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    if cpu.registers[vx] > cpu.registers[vy] {
        cpu.registers[VF] = 1;
    } else {
        cpu.registers[VF] = 0;
    }
    cpu.registers[vx] -= cpu.registers[vy];
}

fn subtract_and_store_and_set_vf(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    if cpu.registers[vy] > cpu.registers[vx] {
        cpu.registers[VF] = 1;
    } else {
        cpu.registers[VF] = 0;
    }
    cpu.registers[vx] = cpu.registers[vy] - cpu.registers[vx];
}

fn store_msb_and_left_shift(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    let number = cpu.registers[vx];
    cpu.registers[VF] = number >> 7;
    cpu.registers[vx] <<= 1;
}

fn set_index_to_mem_address(cpu: &mut Chip8, opcode: OpCode) {
    cpu.index = opcode.code & 0x0FFF;
}

fn jump_to_address_plus_v0(cpu: &mut Chip8, opcode: OpCode) {
    cpu.pc = cpu.registers[0] as u16 + (opcode.code & 0x0FFF);
}

fn bitwise_on_a_random_number_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let random_number: u8 = random();
    let (vx, number) = get_vx_and_number(&opcode);
    cpu.registers[vx] = random_number & number;
}

fn draw_sprite(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, vy) = get_vx_and_vy(&opcode);
    let height = opcode.code & 0x000F;

    let x_position = cpu.registers[vx].rem_euclid(graphics::SCREEN_WIDTH) as u16;
    let y_position = cpu.registers[vy].rem_euclid(graphics::SCREEN_HEIGHT) as u16;

    cpu.registers[VF] = 0;

    for h in 0..height {
        let sprite = cpu.video[(cpu.index + h) as usize];
        for w in 0..8 {
            let pixel = sprite & (0x80 >> w);
            let existing_pixel_index =
                ((y_position + h) * graphics::SCREEN_WIDTH as u16 + (x_position + w)) as usize;
            let existing_pixel = cpu.video[existing_pixel_index];
            if pixel != 0 {
                if existing_pixel == 0xFFFFFFFF {
                    cpu.registers[VF] = 1;
                }
                cpu.video[existing_pixel_index] ^= 0xFFFFFFFF;
            }
        }
    }
    cpu.update_display = true;
}

fn jump_if_key_is_pressed(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    if cpu.keys[vx] {
        cpu.pc += 2;
    }
}

fn jump_if_key_is_not_pressed(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    if !cpu.keys[vx] {
        cpu.pc += 2;
    }
}

fn get_delay_timer_and_set_to_vx(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    cpu.registers[vx] = cpu.delay_timer;
}

fn set_delay_timer_to_vx(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    cpu.delay_timer = vx as u8;
}

fn set_sound_timer_to_vx(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    cpu.sound_timer = vx as u8;
}

fn get_key_press_and_store(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    for (key, &pressed) in cpu.keys.iter().enumerate() {
        if pressed {
            cpu.registers[vx] = key as u8;
            return;
        }
    }
    cpu.pc -= 2;
}

fn add_vx_to_i_and_set_overflow(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    let sum = vx as u16 + cpu.index;
    if sum > 0xFFF {
        cpu.registers[VF] = 1;
    } else {
        cpu.registers[VF] = 0;
    }
    cpu.index = sum;
}

fn set_i_to_location_of_sprite_in_vx(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    let number = cpu.registers[vx] as u16;
    cpu.index = 0x50 + (5 * number);
}

fn store_binary_coded_decimal_representaion_of_vx(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    let mut value = cpu.registers[vx];
    cpu.memory[cpu.index as usize + 2] = value.rem_euclid(10);
    value = value.div_euclid(10);
    cpu.memory[cpu.index as usize + 1] = value.rem_euclid(10);
    value = value.div_euclid(10);
    cpu.memory[cpu.index as usize] = value.rem_euclid(10);
}

fn store_v0_to_vx_starting_at_address_i(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    for v_index in 0..=vx {
        cpu.memory[cpu.index as usize + v_index] = cpu.registers[v_index];
    }
}

fn fill_v0_to_vx_starting_at_address_i(cpu: &mut Chip8, opcode: OpCode) {
    let (vx, _) = get_vx_and_vy(&opcode);
    for v_index in 0..=vx {
        cpu.registers[v_index] = cpu.memory[cpu.index as usize + v_index];
    }
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
