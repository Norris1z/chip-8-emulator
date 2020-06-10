use crate::chip8::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub fn capture(events: &mut EventPump, cpu: &mut Chip8) -> bool {
    let polled_event: Option<Event> = events.poll_event();
    match polled_event {
        Some(event) => match event {
            Event::Quit { .. } => {
                return true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Kp1),
                ..
            } => {
                cpu.keys[0x1] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Kp1),
                ..
            } => {
                cpu.keys[0x1] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Kp2),
                ..
            } => {
                cpu.keys[0x2] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Kp2),
                ..
            } => {
                cpu.keys[0x2] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Kp3),
                ..
            } => {
                cpu.keys[0x3] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Kp3),
                ..
            } => {
                cpu.keys[0x3] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Kp4),
                ..
            } => {
                cpu.keys[0xC] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Kp4),
                ..
            } => {
                cpu.keys[0xC] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => {
                cpu.keys[0x4] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Q),
                ..
            } => {
                cpu.keys[0x4] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.keys[0x5] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.keys[0x5] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => {
                cpu.keys[0x6] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::E),
                ..
            } => {
                cpu.keys[0x6] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                cpu.keys[0xD] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => {
                cpu.keys[0xD] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.keys[0x7] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.keys[0x7] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.keys[0x8] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.keys[0x8] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.keys[0x9] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.keys[0x9] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                cpu.keys[0xE] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::F),
                ..
            } => {
                cpu.keys[0xE] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => {
                cpu.keys[0xA] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Z),
                ..
            } => {
                cpu.keys[0xA] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => {
                cpu.keys[0x0] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::X),
                ..
            } => {
                cpu.keys[0x0] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => {
                cpu.keys[0xB] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => {
                cpu.keys[0xB] = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => {
                cpu.keys[0xF] = true;
            }
            Event::KeyUp {
                keycode: Some(Keycode::V),
                ..
            } => {
                cpu.keys[0xF] = false;
            }
            _ => {}
        },
        None => {
            return false;
        }
    }
    false
}
