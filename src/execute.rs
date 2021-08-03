use crate::machine::Machine;
use rand::{Rng};
use std::sync::mpsc::{Receiver};
use crate::KeyEvent;
use crate::draw_event::DrawEvent;
use sdl2::event::EventSender;

pub fn execute(machine: &mut Machine, key_receiver: &Receiver<KeyEvent>, event_sender: &EventSender) {
    while let Some(key_event) = key_receiver.try_recv().ok() {
        process_key_event(machine, &key_event);
    }

    let op = ((machine.memory[machine.pc as usize] as u16) << 8) | (machine.memory[(machine.pc + 1u16) as usize] as u16);
    machine.pc += 2u16;

    match op & 0xF000 {
        0x0000u16 => {
            match op & 0x00FFu16 {
                // 00E0 - CLS
                0x00E0u16 => {
                    let mut updated = false;
                    let mut display = machine.display.lock().unwrap();
                    for i in 0..display.len() {
                        updated = updated || display[i];
                        display[i] = false
                    }
                    if updated {
                        // draw_sender.send(DrawEvent {});
                        event_sender.push_custom_event(DrawEvent {}).unwrap();
                    }
                }

                // 00EE - RET
                0x00EEu16 => {
                    machine.pc = machine.stack[machine.sp as usize];
                    machine.sp -= 1u16;
                }

                _ => panic!("Unknown opcode: {:x}", op)
            }
        }

        // 1nnn - JP addr
        0x1000u16 => {
            // Jump to nnn.
            machine.pc = op & 0x0FFFu16
        }

        // 2nnn - CALL addr
        0x2000u16 => {
            // Call nnn.
            machine.sp += 1u16;
            machine.stack[machine.sp as usize] = machine.pc;
            machine.pc = op & 0x0FFFu16
        }

        // 3xkk - SE Vx, byte
        0x3000u16 => {
            // Skip next instruction if Vx = kk.
            let x = get_x(op);
            let kk = get_kk(op);
            if machine.v[x] == kk {
                machine.pc += 2u16;
            }
        }

        // 4xkk - SNE Vx, byte
        0x4000u16 => {
            // Skip next instruction if Vx != kk.
            let x = get_x(op);
            let kk = get_kk(op);
            if machine.v[x] != kk {
                machine.pc += 2u16
            }
        }

        0x5000u16 => {
            match op & 0x000Fu16 {
                // 5xy0 - SE Vx, Vy
                0x0000u16 => {
                    // Skip next instruction if Vx = Vy.
                    let x = get_x(op);
                    let y = get_y(op);
                    if machine.v[x] == machine.v[y] {
                        machine.pc += 2u16
                    }
                }

                _ => panic!("Unknown opcode: {:x}", op)
            }
        }

        // 6xkk - LD Vx, byte
        0x6000u16 => {
            // Set Vx = kk.
            let x = get_x(op);
            let kk = get_kk(op);
            machine.v[x] = kk
        }

        // 7xkk - ADD Vx, byte
        0x7000u16 => {
            // Set Vx = Vx + kk.
            let x = get_x(op);
            let kk = get_kk(op);
            machine.v[x] = machine.v[x].wrapping_add(kk);
        }

        0x8000u16 => {
            match op & 0x000Fu16 {
                // 8xy0 - LD Vx, Vy
                0x0000u16 => {
                    // Set Vx = Vy.
                    let x = get_x(op);
                    let y = get_y(op);
                    machine.v[x] = machine.v[y]
                }

                // 8xy1 - OR Vx, Vy
                0x0001u16 => {
                    // Set Vx = Vx OR Vy.
                    let x = get_x(op);
                    let y = get_y(op);
                    machine.v[x] = machine.v[x] | machine.v[y];
                }

                // 8xy2 - AND Vx, Vy
                0x0002u16 => {
                    // Set Vx = Vx AND Vy.
                    let x = get_x(op);
                    let y = get_y(op);
                    machine.v[x] = machine.v[x] | machine.v[y];
                }

                // 8xy3 - XOR Vx, Vy
                0x0003u16 => {
                    // Set Vx = Vx XOR Vy.
                    let x = get_x(op);
                    let y = get_y(op);
                    machine.v[x] = machine.v[x] ^ machine.v[y];
                }

                // 8xy4 - ADD Vx, Vy
                0x0004u16 => {
                    // Set Vx = Vx + Vy, set VF = carry.
                    let x = get_x(op);
                    let y = get_y(op);
                    let result = (machine.v[x] as u16) + (machine.v[y] as u16);
                    machine.v[x] = result as u8;
                    machine.v[0xF] = (result >> 8) as u8;
                }

                // 8xy5 - SUB Vx, Vy
                0x0005u16 => {
                    // Set Vx = Vx - Vy, set VF = NOT borrow.
                    let x = get_x(op);
                    let y = get_y(op);
                    machine.v[0xF] = if machine.v[x] > machine.v[y] {
                        1u8
                    } else {
                        0u8
                    };
                    machine.v[x] = machine.v[x].wrapping_sub(machine.v[y]);
                }

                // 8xy6 - SHR Vx {, Vy}
                0x0006u16 => {
                    // Set Vx = Vx SHR 1.
                    let x = get_x(op);
                    machine.v[0xF] = if (machine.v[x] & 0b1u8) == 0b1u8 {
                        1u8
                    } else {
                        0u8
                    };
                    machine.v[x] >>= 1;
                }

                // 8xy7 - SUBN Vx, Vy
                0x0007u16 => {
                    // Set Vx = Vy - Vx, set VF = NOT borrow.
                    let x = get_x(op);
                    let y = get_y(op);
                    machine.v[0xF] = if machine.v[y] > machine.v[x] {
                        1u8
                    } else {
                        0u8
                    };
                    machine.v[x] = machine.v[y].wrapping_sub(machine.v[x])
                }

                // 8xyE - SHL Vx {, Vy}
                0x000Eu16 => {
                    // Set Vx = Vx SHL 1.
                    let x = get_x(op);
                    machine.v[0xF] = if (machine.v[x] & 0b1000_0000u8) == 0b1000_0000u8 {
                        1u8
                    } else {
                        0u8
                    };
                    machine.v[x] <<= 1
                }

                _ => panic!("Unknown opcode: {:x}", op)
            }
        }

        // 9xy0 - SNE Vx, Vy
        0x9000u16 => {
            // Skip next instruction if Vx != Vy.
            let x = get_x(op);
            let y = get_y(op);
            if machine.v[x] != machine.v[y] {
                machine.pc += 2u16
            }
        }

        // Annn - LD I, addr
        0xA000u16 => {
            // Set I = nnn.
            machine.i = op & 0x0FFFu16
        }

        // Bnnn - JP V0, addr
        0xB000u16 => {
            // Jump to location nnn + V0.
            machine.pc = (op & 0x0FFFu16) + (machine.v[0] as u16)
        }

        // Cxkk - RND Vx, byte
        0xC000u16 => {
            // Set Vx = random byte AND kk.
            let x = get_x(op);
            let kk = get_kk(op);
            let rb = rand::thread_rng().gen_range(0u8..=255u8);
            machine.v[x] = rb & kk
        }

        // Dxyn - DRW Vx, Vy, nibble
        0xD000u16 => {
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            let x = get_x(op);
            let y = get_y(op);
            let n = get_n(op) as u16;
            let vx = machine.v[x] as u16;
            let vy = machine.v[y] as u16;
            let mut display_updated = false;
            let mut collision = false;
            let mut display = machine.display.lock().unwrap();
            for y_offset in 0u16..n {
                let sprite_byte = machine.memory[machine.i as usize + y_offset as usize];
                for x_offset in 0u16..8u16 {
                    let update = ((sprite_byte >> (7 - x_offset)) & 0b1u8) == 0b1u8;
                    let display_position = ((((vx + x_offset) % 64u16) * 32u16) + ((vy + y_offset) % 32u16)) as usize;
                    let existing_pixel = display[display_position];
                    let display_pixel = existing_pixel ^ update;
                    display[display_position] = display_pixel;
                    display_updated |= existing_pixel != display_pixel;
                    collision |= existing_pixel && !display_pixel
                }
            }
            if display_updated {
                // draw_sender.send(DrawEvent {});
                event_sender.push_custom_event(DrawEvent {}).unwrap();
            }
            machine.v[0xF] = if collision {
                1u8
            } else {
                0u8
            };
        }

        0xE000u16 => {
            match op & 0x00FFu16 {
                // Ex9E - SKP Vx
                0x009Eu16 => {
                    // Skip next instruction if key with the value of Vx is pressed.
                    let x = get_x(op);
                    if machine.keys[machine.v[x] as usize] {
                        machine.pc += 2u16
                    }
                }

                // ExA1 - SKNP Vx
                0x00A1u16 => {
                    // Skip next instruction if key with the value of Vx is not pressed.
                    let x = get_x(op);
                    if !machine.keys[machine.v[x] as usize] {
                        machine.pc += 2u16;
                    }
                }

                _ => panic!("Unknown opcode: {:x}", op)
            }
        }

        0xF000u16 => {
            match op & 0x00FFu16 {
                // Fx07 - LD Vx, DT
                0x0007u16 => {
                    // Set Vx = delay timer value.
                    let x = get_x(op);
                    machine.v[x] = *machine.dt.lock().unwrap();
                }

                // Fx0A - LD Vx, K
                0x000Au16 => {
                    // Wait for a key press, store the value of the key in Vx.
                    let x = get_x(op);
                    for i in 0..machine.keys.len() {
                        if machine.keys[i] {
                            machine.v[x] = i as u8;
                            break;
                        }
                    }
                    while let Some(key_event) = key_receiver.recv().ok() {
                        process_key_event(machine, &key_event);
                        if key_event.pressed {
                            machine.v[x] = key_event.key;
                            break;
                        }
                    }
                }

                // Fx15 - LD DT, Vx
                0x0015u16 => {
                    // Set delay timer = Vx.
                    let x = get_x(op);
                    *machine.dt.lock().unwrap() = machine.v[x];
                }

                // Fx18 - LD ST, Vx
                0x0018u16 => {
                    // Set sound timer = Vx.
                    let x = get_x(op);
                    *machine.st.lock().unwrap() = machine.v[x];
                }

                // Fx1E - ADD I, Vx
                0x001Eu16 => {
                    // Set I = I + Vx.
                    let x = get_x(op);
                    machine.i += machine.v[x] as u16;
                }

                // Fx29 - LD F, Vx
                0x0029u16 => {
                    // Set I = location of sprite for digit Vx.
                    let x = get_x(op);
                    machine.i = machine.sprite_digits[machine.v[x] as usize];
                }

                // Fx33 - LD B, Vx
                0x0033u16 => {
                    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                    let x = get_x(op);
                    let hundreds = (machine.v[x] / 100u8) % 10u8;
                    let tens = (machine.v[x] / 10u8) % 10u8;
                    let ones = (machine.v[x]) % 10u8;
                    machine.memory[machine.i as usize] = hundreds;
                    machine.memory[machine.i as usize + 1usize] = tens;
                    machine.memory[machine.i as usize + 2usize] = ones;
                }

                // Fx55 - LD [I], Vx
                0x0055u16 => {
                    // Store registers V0 through Vx in memory starting at location I.
                    let x = get_x(op);
                    for i in 0usize..=x {
                        machine.memory[machine.i as usize + i] = machine.v[i]
                    }
                }

                // Fx65 - LD Vx, [I]
                0x0065u16 => {
                    // Read registers V0 through Vx from memory starting at location I.
                    let x = get_x(op);
                    for i in 0usize..=x {
                        machine.v[i] = machine.memory[machine.i as usize + i]
                    }
                }

                _ => panic!("Unknown opcode: {:x}", op)
            }
        }

        _ => panic!("Unknown opcode: {:x}", op)
    }
}

fn get_x(op: u16) -> usize {
    return ((op & 0x0F00u16) >> 8) as usize;
}

fn get_y(op: u16) -> usize {
    return ((op & 0x00F0u16) >> 4) as usize;
}

fn get_n(op: u16) -> u8 {
    return (op & 0x000Fu16) as u8;
}

fn get_kk(op: u16) -> u8 {
    return op as u8;
}

fn process_key_event(machine: &mut Machine, key_event: &KeyEvent) {
    machine.keys[key_event.key as usize] = key_event.pressed;
}
