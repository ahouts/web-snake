
// possible graphics data types
pub enum PixelData {
    // a full pixel at position (x, y)
    Pixel {
        x: u32,
        y: u32,
        color: String
    },
    // a partial pixel at position (x, y)
    //
    // c1 and c2 represent the relative position in the
    // pixel for the top left corner and bottom right corner
    //
    // for c1(m, n):
    // 0 <= m <= 1 :: where 0 is the left side of the
    // pixel and 1 is the right side of the pixel
    // 0 <= n <= 1 :: where 0 is the top of the pixel
    // and 1 is the bottom of the pixel
    //
    // for example, c1(0.25, 0), c2(1, 0.5) would represent
    // the following subpixel
    //   +-----------+
    //   |  |  |  |  |
    //   |XX|XX|  |  |
    //   |XX|XX|  |  |
    //   |XX|XX|  |  |
    //   +-----------+
    SubPixel {
        x: u32,
        y: u32,
        c1: (f64, f64),
        c2: (f64, f64),
        color: String
    }
}

// collection of graphics data to reunder
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