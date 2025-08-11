use crate::ppu::ppu::PPU;

pub fn get_bg_palette(ppu: &PPU, tile_column: usize, tile_row: usize) -> [u8; 4] {
    let attr_byte = ppu.read_vram(0x03C0 + (tile_row / 4 * 8 + tile_column / 4)) as usize;

    let palette_idx = match (tile_column % 4 / 2, tile_row % 4 / 2) {
        (0, 0) => attr_byte & 0b11,
        (1, 0) => (attr_byte >> 2) & 0b11,
        (0, 1) => (attr_byte >> 4) & 0b11,
        (1, 1) => (attr_byte >> 6) & 0b11,
        (_, _) => panic!("Impossible value for palette index."),
    };

    let palette_start = 1 + palette_idx * 4;
    [
        ppu.read_palette_table(0),
        ppu.read_palette_table(palette_start),
        ppu.read_palette_table(palette_start + 1),
        ppu.read_palette_table(palette_start + 2),
    ]
}

pub fn sprite_palette(ppu: &PPU, palette_idx: u8) -> [u8; 4] {
    let start = 0x11 + (palette_idx * 4) as usize;
    [
        0,
        ppu.read_palette_table(start),
        ppu.read_palette_table(start + 1),
        ppu.read_palette_table(start + 2),
    ]
}
