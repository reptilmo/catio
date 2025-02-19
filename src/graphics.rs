use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;

pub struct Graphics {
    canvas: WindowCanvas,
}

impl Graphics {
    pub fn new(canvas: WindowCanvas) -> Self {
        Self { canvas }
    }

    pub fn copy_from_surface(&mut self, surface: &Surface) {
        // TODO: There is a memory leak here somewhere.
        let texture_creator = self.canvas.texture_creator();
        let result = texture_creator.create_texture_from_surface(surface);
        let texture = match result {
            Ok(tex) => tex, Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        let dst = Rect::new(0, 0, 120, 24);
        let result = self.canvas.copy(&texture, None, Some(dst));
        if let Err(e) = result {
            eprintln!("{}", e);
        }
    }

    pub fn begin_frame(&mut self) {
        self.canvas.set_draw_color(Color::RGB(40, 40, 40));
        self.canvas.clear();
    }

    pub fn end_frame(&mut self) {
        self.canvas.present();
    }

    pub fn set_draw_color(&mut self, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
    }

    pub fn draw_horizontal_line(&mut self, x_min: i32, x_max: i32, y: i32) {
        for x in x_min..x_max {
            self.canvas.draw_point(Point::new(x, y)).unwrap();
        }
    }

    pub fn draw_vertical_line(&mut self, x: i32, y_min: i32, y_max: i32) {
        for y in y_min..y_max {
            self.canvas.draw_point(Point::new(x, y)).unwrap();
        }
    }

    pub fn draw_box(&mut self, upper_left: (i32, i32), lower_right: (i32, i32)) {
        let (x0, y0) = upper_left;
        let (x1, y1) = lower_right;

        for y in y0..y1 {
            self.draw_horizontal_line(x0, x1, y);
        }
    }

    pub fn draw_circle(&mut self, origin: (i32, i32), radius: i32) {
        let (ox, oy) = origin;
        let r2 = radius * radius;

        for x in -radius..radius {
            let y: i32 = (f32::sqrt((r2 - x * x) as f32) + 0.5) as i32;
            self.draw_vertical_line(ox + x, oy - y, oy + y);
        }
    }
}
