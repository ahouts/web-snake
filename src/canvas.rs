use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;
use graphics_data::GraphicsData;
use graphics_data::PixelData;
use js_utils::get_date;
use chrono::{DateTime, FixedOffset};
use time::Duration;

pub struct Canvas {
    canvas_dom_element: CanvasElement,
    ctx: CanvasRenderingContext2d,
    background_color: String,
    duration_between_frames: Duration,
    last_frame: DateTime<FixedOffset>
}

impl Canvas {
    pub fn new(e: CanvasElement, frame_rate: u32) -> Result<Self, String> {
        let ctx: CanvasRenderingContext2d = match e.get_context() {
            Ok(ctx) => ctx,
            Err(e) => {
                return Err(format!("error getting canvas element rendering context: {}", e));
            }
        };
        Ok(Canvas {
            canvas_dom_element: e,
            ctx,
            background_color: String::from("white"),
            duration_between_frames: Duration::milliseconds(((1.0 / frame_rate as f64) * 1000.0) as i64),
            last_frame: get_date()
        })
    }

    pub fn render(&mut self, data: &GraphicsData) -> i64 {
        let now = get_date();
        let mut time_diff: Duration = now - self.last_frame;
        if time_diff >= self.duration_between_frames {
            self.last_frame = now;
            self.clear_screen();
            let pixel_width = self.canvas_dom_element.width() as f64 / data.width as f64;
            let pixel_height = self.canvas_dom_element.height() as f64 / data.height as f64;
            for pixel in data.pixels.iter() {
                self.draw_pixel(pixel, pixel_width, pixel_height);
            }
        }
        time_diff = get_date() - self.last_frame;
        time_diff = self.duration_between_frames - time_diff;
        if time_diff.num_milliseconds() < 0 {
            time_diff = Duration::milliseconds(0);
        }
        time_diff.num_milliseconds()
    }

    fn draw_pixel(&self, pixel: &PixelData, pixel_width: f64, pixel_height: f64) {
        match *pixel {
            PixelData::Pixel{x, y, ref color} => {
                self.ctx.set_fill_style_color(color.as_ref());
                self.ctx.fill_rect(pixel_width * x as f64, pixel_height * y as f64, pixel_width, pixel_height);
            },
            PixelData::SubPixel {x, y, c1, c2, ref color} => {
                self.ctx.set_fill_style_color(color.as_ref());
                self.ctx.fill_rect((c1.0 + x as f64) * pixel_width, (c1.1 + y as f64) * pixel_height, c2.0 * pixel_width, c2.1 * pixel_height);
            },
        }
    }

    fn clear_screen(&self) {
        self.ctx.set_fill_style_color(&self.background_color);
        self.ctx.fill_rect(0.0,
                           0.0,
                           self.canvas_dom_element.width() as f64,
                           self.canvas_dom_element.height() as f64);
    }
}
