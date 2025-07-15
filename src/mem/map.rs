pub struct MemoryMap {
    layout: [u8; 0xFFFF],
}

pub trait IOOperation<T> {
    fn read(&self, address: u16) -> T;

    fn write(&mut self, address: u16, value: T);
}

impl MemoryMap {
    pub fn new() -> MemoryMap {
        MemoryMap {
            layout: [0; 0xFFFF],
        }
    }

    pub fn copy_to(&mut self, address: u16, data: &[u8]) {
        self.layout[address as usize..(address as usize + data.len())].clone_from_slice(data);
    }
}

impl IOOperation<u8> for MemoryMap {
    fn read(&self, address: u16) -> u8 {
        self.layout[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.layout[address as usize] = value;
    }
}

impl IOOperation<u16> for MemoryMap {
    fn read(&self, address: u16) -> u16 {
        u16::from_le_bytes([
            self.layout[address as usize],
            self.layout[address as usize + 1],
        ])
    }

    fn write(&mut self, address: u16, value: u16) {
        let value_le_bytes: [u8; 2] = value.to_le_bytes();
        self.layout[address as usize] = value_le_bytes[0];
        self.layout[address as usize + 1] = value_le_bytes[1];
    }
}
