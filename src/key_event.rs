use std::sync::mpsc::Sender;
use sdl2::keyboard::Keycode;

pub struct KeyEvent {
    pub key: u8,
    pub pressed: bool
}

pub fn handle_key_press(key_sender: &Sender<KeyEvent>, key_code: Keycode, pressed: bool) {
    match key_code {
        Keycode::Num1 => key_sender.send(KeyEvent { key: 0x1u8, pressed }).unwrap(),
        Keycode::Num2 => key_sender.send(KeyEvent { key: 0x2u8, pressed }).unwrap(),
        Keycode::Num3 => key_sender.send(KeyEvent { key: 0x3u8, pressed }).unwrap(),
        Keycode::Num4 => key_sender.send(KeyEvent { key: 0xCu8, pressed }).unwrap(),
        Keycode::Q => key_sender.send(KeyEvent { key: 0x4u8, pressed }).unwrap(),
        Keycode::W => key_sender.send(KeyEvent { key: 0x5u8, pressed }).unwrap(),
        Keycode::E => key_sender.send(KeyEvent { key: 0x6u8, pressed }).unwrap(),
        Keycode::R => key_sender.send(KeyEvent { key: 0xDu8, pressed }).unwrap(),
        Keycode::A => key_sender.send(KeyEvent { key: 0x7u8, pressed }).unwrap(),
        Keycode::S => key_sender.send(KeyEvent { key: 0x8u8, pressed }).unwrap(),
        Keycode::D => key_sender.send(KeyEvent { key: 0x9u8, pressed }).unwrap(),
        Keycode::F => key_sender.send(KeyEvent { key: 0xEu8, pressed }).unwrap(),
        Keycode::Z => key_sender.send(KeyEvent { key: 0xAu8, pressed }).unwrap(),
        Keycode::X => key_sender.send(KeyEvent { key: 0x0u8, pressed }).unwrap(),
        Keycode::C => key_sender.send(KeyEvent { key: 0xBu8, pressed }).unwrap(),
        Keycode::V => key_sender.send(KeyEvent { key: 0xFu8, pressed }).unwrap(),
        _ => {}
    }
    return
}