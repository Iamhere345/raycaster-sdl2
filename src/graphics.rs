use std::fmt;
use std::ops::{Mul, Div};
use sdl2::pixels::Color;
use sdl2::{keyboard::Keycode, render::Canvas};
use sdl2::video::Window;

pub const CANVAS_WIDTH: i32 = 600;
pub const CANVAS_HEIGHT: i32 = 600;

pub const SCREEN_WIDTH: usize = CANVAS_WIDTH as usize;
pub const SCREEN_HEIGHT: usize = CANVAS_HEIGHT as usize;

pub type PixelColour = [u8; 3];
pub type PixelBuf = Vec<PixelColour>; // depricated in favour of Screen
pub type Screen = Canvas<Window>;
pub type InputKeycode = Keycode;

#[derive(Clone, Copy, Debug)]
pub struct CanvasColour {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl CanvasColour {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b
        }
    }

    const fn as_pixels(self) -> PixelColour {
        [self.r, self.g, self.b]
    }

    pub fn from_u32(from: u32) -> Self {
        Self {
            // the last byte in unused (but could be used for alpha)
            r: ((from >> 0) & 255) as u8,
            g: ((from >> 8) & 255) as u8,
            b: ((from >> 16) & 255) as u8
        }
    }

    pub const RED: CanvasColour = CanvasColour::new(255, 0, 0);
    pub const GREEN: CanvasColour = CanvasColour::new(0, 255, 0);
    pub const BLUE: CanvasColour = CanvasColour::new(0, 0, 255);
    pub const WHITE: CanvasColour = CanvasColour::new(255, 255, 255);
    pub const TEAL: CanvasColour = CanvasColour::new(2, 247, 247);

}

impl fmt::Display for CanvasColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.r, self.g, self.b)
    }
}

impl Div<u8> for CanvasColour {

    type Output = Self;

    fn div(self, div: u8) -> Self::Output {
        Self {
            r: self.r / div,
            g: self.g / div,
            b: self.b / div,
        }
    }

}

impl Mul<f32> for CanvasColour {

    type Output = Self;

    fn mul(self, mul: f32) -> Self::Output {
        Self {
            r: (self.r as f32 * mul) as u8,
            g: (self.g as f32 * mul) as u8,
            b: (self.b as f32 * mul) as u8,
        }
    }

}

impl From<(u8, u8, u8)> for CanvasColour {
    fn from(colour: (u8, u8, u8)) -> Self {
        Self::new(colour.0, colour.1, colour.2)
    }
}

pub fn draw_line(canvas: &mut Canvas<Window>, x: i32, start: i32, end: i32, colour: CanvasColour) {
    
    canvas.set_draw_color(Color::RGB(colour.r, colour.g, colour.b));
    canvas.draw_line((x, start), (x, end)).unwrap();

}

pub fn draw_pixel(canvas: &mut Canvas<Window>, x: i32, y: i32, colour: CanvasColour) {

    if x >= SCREEN_WIDTH as i32 || x <= SCREEN_WIDTH as i32 {
        //panic!("draw exceeded screen bounds (x: {x}, {SCREEN_WIDTH}");
    }
    if y >= SCREEN_HEIGHT as i32 || y <= SCREEN_HEIGHT as i32 {
        //panic!("draw exceeded screen bounds (y: {y}, {SCREEN_HEIGHT}");
    }

    canvas.set_draw_color(Color::RGB(colour.r, colour.g, colour.b));
    canvas.draw_point((x, y)).unwrap();

}