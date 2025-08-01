// PPUDATA - VRAM data ($2007 read/write)
// https://www.nesdev.org/wiki/PPU_registers#PPUDATA_-_VRAM_data_($2007_read/write)
//
// 7654 3210 bit
// ---- ----
// DDDD DDDD
// |||| ||||
// ++++-++++- VRAM data
pub struct DataRegister {
    write_value: u8,
    read_buffer: u8,
}

impl DataRegister {
    pub fn new() -> Self {
        DataRegister {
            write_value: 0,
            read_buffer: 0,
        }
    }

    pub fn get_write_value(&self) -> u8 {
        self.write_value
    }

    pub fn read(&mut self, buffer_value: u8) -> u8 {
        let result = self.read_buffer;
        self.read_buffer = buffer_value;
        result
    }
}
