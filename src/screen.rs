pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl RGBA {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        RGBA { r, g, b, a }
    }
}

pub trait Screen {
    fn write_pixel(&mut self, x: u32, y: u32, color: &RGBA);
    fn line(&mut self, x1: i32, x2: i32, y1: i32, y2: i32, color: RGBA) {
        unimplemented!();
    }
}

pub struct U32BufferScreen {
    buffer: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

impl U32BufferScreen {
    pub fn new(width: u32, height: u32) -> Self {
        U32BufferScreen {
            buffer: vec![0; (width * height) as usize],
            width,
            height,
        }
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}

impl Screen for U32BufferScreen {
    fn write_pixel(&mut self, x: u32, y: u32, color: &RGBA) {
        if x >= self.width || y >= self.height {
            panic!("Tried to draw outside bounds");
        }
        let (r, g, b) = (color.r as u32, color.g as u32, color.b as u32);
        self.buffer[(y * self.width + x) as usize] = (r << 16) | (g << 8) | b;
    }
}