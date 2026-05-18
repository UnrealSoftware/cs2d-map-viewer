use macroquad::prelude::*;

/// A color in RGB (red, green, blue) format, each component 8 bit (0 - 255)
/// Unlike the Color struct, this one uses u8 and has no alpha component.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_color(self) -> Color {
        Color::from_rgba(self.r, self.g, self.b, 255)
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            r: (color.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            g: (color.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            b: (color.b.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
    }
}