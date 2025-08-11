pub struct Frame {
    pub data: Vec<u8>,
}

impl Frame {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 240;

    pub fn new() -> Frame {
        Frame {
            data: vec![0; Self::WIDTH * Self::HEIGHT * 3],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let pixel_index = y * 3 * Frame::WIDTH + x * 3;
        if pixel_index + 2 < self.data.len() {
            self.data[pixel_index] = rgb.0;
            self.data[pixel_index + 1] = rgb.1;
            self.data[pixel_index + 2] = rgb.2;
        }
    }
}
