use macroquad::prelude::*;

/// Wraps texture sheet logic around a texture.
/// A texture sheet has X frames with equal size.
/// The frame order in the sheet is top-left to bottom-right.
/// This struct provides utility methods to draw those more easily.
#[derive(Debug, Clone, PartialEq)]
pub struct TextureSheet {
    pub texture:Texture2D,
    pub frame_size:IVec2,
    pub frames_per_row:u16,
    pub frame_count:u16,
}

impl TextureSheet {
    pub fn new(texture:Texture2D, frame_size:IVec2) -> Self {
        let size = texture.size();
        let frames_per_row = (size.x / frame_size.x as f32) as u16;
        let frames_per_column = (size.y / frame_size.y as f32) as u16;
        let frame_count = frames_per_row * frames_per_column;
        Self {
            texture,
            frame_size,
            frames_per_row,
            frame_count,
        }
    }

    pub fn draw(&self, x:f32, y:f32, frame:u16, color:Color, rotation:f32) {
        let frame_x = (frame % self.frames_per_row) as i32;
        let frame_y = (frame / self.frames_per_row) as i32;

        draw_texture_ex(
            &self.texture,
            x, y,
            color,
            DrawTextureParams {
                source: Option::from(Rect {
                    x: (frame_x * self.frame_size.x) as f32,
                    y: (frame_y * self.frame_size.y) as f32,
                    w: self.frame_size.x as f32,
                    h: self.frame_size.y as f32
                }),
                rotation,
                ..Default::default()
            }
        );
    }
}