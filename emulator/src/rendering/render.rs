use crate::rendering::frame::Frame;
use crate::rendering::palette::SYSTEM_PALETTE;
use crate::rendering::view_port::ViewPort;
use nes::ppu::palette::{get_bg_palette, sprite_palette};
use nes::ppu::ppu::PPU;
use std::ops::Range;

pub fn render(ppu: &PPU, frame: &mut Frame) {
    let (main_name_table, second_name_table) = ppu.get_name_table_ranges();
    let scroll_x = ppu.get_x_scroll() as usize;
    let scroll_y = ppu.get_y_scroll() as usize;

    render_name_table(
        ppu,
        frame,
        main_name_table,
        ViewPort::new(scroll_x, scroll_y, 256, 240),
        -(scroll_x as isize),
        -(scroll_y as isize),
    );
    if scroll_x > 0 {
        render_name_table(
            ppu,
            frame,
            second_name_table,
            ViewPort::new(0, 0, scroll_x, 240),
            (256 - scroll_x) as isize,
            0,
        );
    } else if scroll_y > 0 {
        render_name_table(
            ppu,
            frame,
            second_name_table,
            ViewPort::new(0, 0, 256, scroll_y),
            0,
            (240 - scroll_y) as isize,
        );
    }

    for i in (0..256).step_by(4).rev() {
        let tile_idx = ppu.read_oamdata(i + 1) as usize;
        let tile_x = ppu.read_oamdata(i + 3) as usize;
        let tile_y = ppu.read_oamdata(i) as usize;

        let flip_vertical = ppu.read_oamdata(i + 2) >> 7 & 1 == 1;
        let flip_horizontal = ppu.read_oamdata(i + 2) >> 6 & 1 == 1;
        let palette_idx = ppu.read_oamdata(i + 2) & 0b11;
        let sprite_palette = sprite_palette(ppu, palette_idx);

        let sprite_tile = ppu.read_sprite_tile(tile_idx);

        for y in 0..=7 {
            let mut upper = sprite_tile[y];
            let mut lower = sprite_tile[y + 8];

            'c: for x in (0..=7).rev() {
                let value = (1 & lower) << 1 | (1 & upper);
                upper >>= 1;
                lower >>= 1;
                let rgb = match value {
                    0 => continue 'c,
                    1 => SYSTEM_PALETTE[sprite_palette[1] as usize],
                    2 => SYSTEM_PALETTE[sprite_palette[2] as usize],
                    3 => SYSTEM_PALETTE[sprite_palette[3] as usize],
                    _ => panic!("Impossible value for tile pixel."),
                };
                match (flip_horizontal, flip_vertical) {
                    (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                    (true, false) => frame.set_pixel(tile_x + 7 - x, tile_y + y, rgb),
                    (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                    (true, true) => frame.set_pixel(tile_x + 7 - x, tile_y + 7 - y, rgb),
                };
            }
        }
    }
}

pub fn render_name_table(
    ppu: &PPU,
    frame: &mut Frame,
    name_table_range: Range<usize>,
    view_port: ViewPort,
    shift_x: isize,
    shift_y: isize,
) {
    for i in 0..0x03C0usize {
        let tile_x = i % 32;
        let tile_y = i / 32;
        let tile = ppu.read_tile(i, &name_table_range);
        let palette = get_bg_palette(ppu, tile_x, tile_y);

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let rgb = match (1 & lower) << 1 | (1 & upper) {
                    0 => SYSTEM_PALETTE[palette[0] as usize],
                    1 => SYSTEM_PALETTE[palette[1] as usize],
                    2 => SYSTEM_PALETTE[palette[2] as usize],
                    3 => SYSTEM_PALETTE[palette[3] as usize],
                    _ => panic!("Impossible value for tile pixel."),
                };
                upper >>= 1;
                lower >>= 1;

                let pixel_x = tile_x * 8 + x;
                let pixel_y = tile_y * 8 + y;

                if pixel_x >= view_port.x1
                    && pixel_x < view_port.x2
                    && pixel_y >= view_port.y1
                    && pixel_y < view_port.y2
                {
                    frame.set_pixel(
                        (shift_x + pixel_x as isize) as usize,
                        (shift_y + pixel_y as isize) as usize,
                        rgb,
                    );
                }
            }
        }
    }
}
