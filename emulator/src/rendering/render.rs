use crate::rendering::frame::Frame;
use crate::rendering::palette::SYSTEM_PALETTE;
use nes::ppu::palette::{get_bg_palette, sprite_palette};
use nes::ppu::ppu::PPU;

pub fn render(ppu: &PPU, frame: &mut Frame) {
    for i in 0..0x03c0usize {
        let tile_x = i % 32;
        let tile_y = i / 32;
        let tile = ppu.read_tile(i);
        let palette = get_bg_palette(ppu, tile_x, tile_y);

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                frame.set_pixel(
                    tile_x * 8 + x,
                    tile_y * 8 + y,
                    match (1 & lower) << 1 | (1 & upper) {
                        0 => SYSTEM_PALETTE[palette[0] as usize],
                        1 => SYSTEM_PALETTE[palette[1] as usize],
                        2 => SYSTEM_PALETTE[palette[2] as usize],
                        3 => SYSTEM_PALETTE[palette[3] as usize],
                        _ => panic!("Impossible value for tile pixel."),
                    },
                );
                upper >>= 1;
                lower >>= 1;
            }
        }
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
