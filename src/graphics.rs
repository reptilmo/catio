use catphys::Vec2;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::surface::Surface;
use sdl2::video::WindowContext;
use std::cell::Cell;
use std::path::Path;

pub struct Graphics {
    canvas: WindowCanvas,
}

pub struct Render {
    pub texture_idx: Option<usize>,
    pub color: Color,
    pub fill: bool,
}

#[allow(unused)]
impl Graphics {
    pub fn new(c: WindowCanvas) -> Self {
        Self {
            canvas: c,
        }
    }

    pub fn texture_creator(&mut self) -> TextureCreator<WindowContext> {
        self.canvas.texture_creator()
    }

    pub fn copy_from_surface(&mut self, surface: &Surface) {
        // TODO: There is a memory leak here somewhere.
        let texture_creator = self.canvas.texture_creator();
        let result = texture_creator.create_texture_from_surface(surface);
        let texture = match result {
            Ok(tex) => tex,
            Err(e) => {
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
        self.canvas.set_draw_color(Color::RGB(50, 50, 50));
        self.canvas.clear();
    }

    pub fn end_frame(&mut self) {
        self.canvas.present();
    }

    pub fn set_draw_color(&mut self, c: Color) {
        self.canvas.set_draw_color(c);
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

    // https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut x0 = x0;
        let mut y0 = y0;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let mut error = dx + dy;

        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        loop {
            self.canvas.draw_point(Point::new(x0, y0)).unwrap();
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                error += dy;
                x0 += sx;
            }

            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_rect(&mut self, upper_left: (i32, i32), lower_right: (i32, i32)) {
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

    pub fn draw_box(&mut self, origin: Vec2, width: f32, height: f32, rotation: f32) {
        let mut v0 = Vec2::new(width * -0.5, height * -0.5).rotate(rotation);
        let mut v1 = Vec2::new(width * 0.5, height * -0.5).rotate(rotation);
        let mut v2 = Vec2::new(width * 0.5, height * 0.5).rotate(rotation);
        let mut v3 = Vec2::new(width * -0.5, height * 0.5).rotate(rotation);

        v0 += origin;
        v1 += origin;
        v2 += origin;
        v3 += origin;

        // TODO: Draw filled box!
        // NOTE: Canvas already has functions for drawing primitives.
        self.draw_line(v0.x as i32, v0.y as i32, v1.x as i32, v1.y as i32);
        self.draw_line(v1.x as i32, v1.y as i32, v2.x as i32, v2.y as i32);
        self.draw_line(v2.x as i32, v2.y as i32, v3.x as i32, v3.y as i32);
        self.draw_line(v3.x as i32, v3.y as i32, v0.x as i32, v0.y as i32);
    }

    pub fn draw_texture(&mut self, texture: &Texture, idx: usize, x: i16, y: i16, width: i16, height: i16) {
        self.canvas
            .copy(
                texture,
                None,
                Some(Rect::new(x.into(), y.into(), width as u32, height as u32)),
            )
            .unwrap();
    }
}
