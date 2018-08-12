
pub enum PixelData {
    Pixel {
        x: u32,
        y: u32,
        color: String
    },
    SubPixel {
        x: u32,
        y: u32,
        c1: (f64, f64),
        c2: (f64, f64),
        color: String
    }
}

pub struct GraphicsData {
    pub pixels: Vec<PixelData>,
    pub width: u32,
    pub height: u32
}

impl GraphicsData {
    pub fn new(width: u32, height: u32) -> Self {
        GraphicsData {
            pixels: Vec::new(),
            width,
            height
        }
    }

    pub fn add_pixel(&mut self, x: u32, y: u32, color: String) {
        self.pixels.push(PixelData::Pixel {
            x,
            y,
            color,
        })
    }

    pub fn add_sub_pixel(&mut self, x: u32, y: u32, c1: (f64, f64), c2: (f64, f64), color: String) {
        self.pixels.push( PixelData::SubPixel {
            x,
            y,
            c1,
            c2,
            color,
        })
    }
}