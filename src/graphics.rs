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
    r: u8,
    g: u8,
    b: u8
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

    pub const RED: CanvasColour = CanvasColour::new(255, 0, 0);
    pub const GREEN: CanvasColour = CanvasColour::new(0, 255, 0);
    pub const BLUE: CanvasColour = CanvasColour::new(0, 0, 255);
    pub const WHITE: CanvasColour = CanvasColour::new(255, 255, 255);
    pub const TEAL: CanvasColour = CanvasColour::new(2, 247, 247);

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