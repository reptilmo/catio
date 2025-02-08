use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use sdl2::VideoSubsystem;

pub struct Graphics {
    window: Window,
    canvas: WindowCanvas,
}

impl Graphics {
    pub fn create(video: &VideoSubsystem, width: u32, height: u32, _fullscreen: bool) -> Self {
        // TODO: Fullscreen.
        let wnd = video
            .window("", width, height)
            .position_centered()
            .build()
            .unwrap();

        let cnv = wnd.clone().into_canvas().build().unwrap();

        Self {
            window: wnd,
            canvas: cnv,
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
