// PPUADDR - VRAM address ($2006 write)
// https://www.nesdev.org/wiki/PPU_registers#PPUADDR
//
// 1st write  2nd write
// 15 bit  8  7  bit  0
// ---- ----  ---- ----
// ..AA AAAA  AAAA AAAA
// || ||||  |||| ||||
// ++-++++--++++-++++- VRAM address
pub struct AddressRegister {
    value: u16,
    hi_ptr: bool,
}

impl AddressRegister {
    pub fn new() -> AddressRegister {
        AddressRegister {
            value: 0,
            hi_ptr: true,
        }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn update(&mut self, data: u8) {
        let mut value_bytes: [u8; 2] = self.value.to_be_bytes();
        if self.hi_ptr {
            value_bytes[0] = data;
        } else {
            value_bytes[1] = data;
        }
        self.set(u16::from_be_bytes(value_bytes));
        self.hi_ptr = !self.hi_ptr;
    }

    pub fn increment(&mut self, inc: u8) {
        self.set(self.value.wrapping_add(inc as u16));
    }

    fn set(&mut self, addr: u16) {
        self.value = addr & 0b0011_1111_1111_1111;
    }
}
