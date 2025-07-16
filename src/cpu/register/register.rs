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
        let addition_result = self.value.overflowing_add(value);
        self.value = addition_result.0;
        addition_result
    }

    pub fn sub(&mut self, value: u8, store: bool) -> u8 {
        let result = self.value.wrapping_sub(value);
        if store {
            self.value = result;
        }
        result
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
        self.value = self.value.wrapping_add(value);
        self.value
    }

    pub fn add_signed(&mut self, value: i16) {
        self.value = self.value.wrapping_add_signed(value);
    }
}
