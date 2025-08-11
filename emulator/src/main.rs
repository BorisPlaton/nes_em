use emulator::rendering::frame::Frame;
use emulator::rendering::render::render;
use nes::cpu::bus::CPUBus;
use nes::cpu::cpu::CPU;
use nes::ppu::ppu::PPU;
use nes::rom::rom::Rom;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::fs::OpenOptions;

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

    let bytes: Vec<u8> = std::fs::read("./roms/123.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();
    let mut frame = Frame::new();
    let bus = CPUBus::new(rom, |ppu: &PPU| {
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
                _ => { /* do nothing */ }
            }
        }
    });
    let mut cpu = CPU::new(bus);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();

    file.set_len(0).unwrap();

    cpu.reset_interrupt();
    cpu.run(|_| {}).unwrap();
    // cpu.run(|cpu| println!("{}", trace(cpu))).unwrap();
}
