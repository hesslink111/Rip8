mod machine;
mod execute;
mod key_event;
mod draw_event;

use crate::machine::Machine;
use crate::execute::execute;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{Read};
use std::cmp::min;
use std::sync::mpsc::channel;
use crate::key_event::{KeyEvent, handle_key_press};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use crate::draw_event::DrawEvent;
use sdl2::timer::{TimerCallback};

fn main() {
    let (key_sender, key_receiver) = channel();
    let mut machine = Machine::init();
    let display = machine.display.clone();
    let dt = machine.dt.clone();
    let st = machine.st.clone();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let event = sdl_context.event().unwrap();
    event.register_custom_event::<DrawEvent>().unwrap();
    let event_sender = event.event_sender();

    thread::spawn(move || {
        let mut rom = File::open("roms/Breakout (Brix hack) [David Winter, 1997].ch8").unwrap();
        let mut rom_data = Vec::new();
        rom.read_to_end(&mut rom_data).unwrap();

        for i in 0..(min(rom_data.len(), 0x1000-0x200)) {
            machine.memory[0x200 + i] = rom_data[i];
        }
        machine.pc = 0x200u16;

        loop {
            execute(&mut machine, &key_receiver, &event_sender);
            thread::sleep(Duration::from_millis(2));
        }
    });

    let window = video_subsystem
        .window("Rip8", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let timer = sdl_context.timer().unwrap();
    let _t = timer.add_timer(1000_000 / 60_000, TimerCallback::from(Box::new(|| {
        let mut dt = dt.lock().unwrap();
        let mut st = st.lock().unwrap();
        *dt = dt.checked_sub(1).unwrap_or(0);
        *st = st.checked_sub(1).unwrap_or(0);
        return 1000_000 / 60_000;
    })));

    for event in event_pump.wait_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => break,
            Event::KeyDown { keycode: Some(key_code), .. } => handle_key_press(&key_sender, key_code, true),
            Event::KeyUp { keycode: Some(key_code), .. } => handle_key_press(&key_sender, key_code, false),
            Event::User { .. } => {
                canvas.set_draw_color(Color::BLACK);
                canvas.clear();
                let (width, height) = canvas.output_size().unwrap();
                let pixel_width = width / 64;
                let pixel_height = height / 32;
                let display = display.lock().unwrap();
                for y in 0..32 {
                    for x in 0..64 {
                        let color = if display[x * 32 + y] {
                            Color::WHITE
                        } else {
                            Color::BLACK
                        };
                        canvas.set_draw_color(color);
                        canvas.fill_rect(Rect::new(
                            x as i32 * pixel_width as i32,
                            y as i32 * pixel_height as i32,
                            pixel_width,
                            pixel_height
                        )).unwrap();
                    }
                }
                canvas.present();
            }
            _ => {}
        }
    }
}
