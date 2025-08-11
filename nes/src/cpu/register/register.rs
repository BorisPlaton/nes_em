#[derive(Clone)]
pub struct Register<T>
where
    T: Copy,
{
    value: T,
}

impl<T> Register<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Register { value }
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }

    pub fn get(&self) -> T {
        self.value
    }
}

// TODO: Write macro to remove impl repeating

impl Register<u8> {
    pub fn inc(&mut self) -> u8 {
        self.value = self.value.wrapping_add(1);
        self.value
    }

    pub fn dec(&mut self) {
        self.value = self.value.wrapping_sub(1);
    }

    pub fn add(&mut self, value: u8) -> (u8, bool) {
        self.value.overflowing_add(value)
    }

    pub fn sub(&mut self, value: u8) -> u8 {
        self.value.wrapping_sub(value)
    }
}

impl Register<u16> {
    pub fn inc(&mut self) -> u16 {
        self.value = self.value.wrapping_add(1);
        self.value
    }

    pub fn dec(&mut self) {
        self.value = self.value.wrapping_sub(1);
    }

    pub fn add(&mut self, value: u16) -> u16 {
        self.value.wrapping_add(value)
    }

    pub fn add_signed(&mut self, value: i16) -> u16 {
        self.value = self.value.wrapping_add_signed(value);
        self.value
    }
}
