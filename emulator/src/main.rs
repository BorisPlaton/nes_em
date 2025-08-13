use emulator::rendering::frame::Frame;
use emulator::rendering::render::render;
use nes::bus::Bus;
use nes::controller::controller::Controller;
use nes::controller::register::JoypadRegister;
use nes::cpu::cpu::CPU;
use nes::ppu::ppu::PPU;
use nes::rom::rom::Rom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::collections::HashMap;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    let mut key_map = HashMap::new();
    key_map.insert(Keycode::Down, JoypadRegister::DOWN);
    key_map.insert(Keycode::Up, JoypadRegister::UP);
    key_map.insert(Keycode::Right, JoypadRegister::RIGHT);
    key_map.insert(Keycode::Left, JoypadRegister::LEFT);
    key_map.insert(Keycode::E, JoypadRegister::SELECT);
    key_map.insert(Keycode::Return, JoypadRegister::START);
    key_map.insert(Keycode::A, JoypadRegister::BUTTON_A);
    key_map.insert(Keycode::B, JoypadRegister::BUTTON_B);

    let bytes: Vec<u8> = std::fs::read("./roms/123.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let mut frame = Frame::new();
    let bus = Bus::new(rom, |ppu: &PPU, contoller: &mut Controller| {
        render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        contoller.set_button_status(key.clone(), true);
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        contoller.set_button_status(key.clone(), false);
                    }
                }

                _ => { /* do nothing */ }
            }
        }
    });
    let mut cpu = CPU::new(bus);
    cpu.reset_interrupt();
    cpu.run(|_| {}).unwrap();

    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("log.txt")
    //     .unwrap();
    // file.set_len(0).unwrap();
    // cpu.run(|cpu| println!("{}", trace(cpu))).unwrap();
}
