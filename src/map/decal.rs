use macroquad::prelude::{Color, Vec2};

/// A decal on the ground
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Decal {
    pub level: u8,
    pub frame: u8,
    pub color: Color,
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}