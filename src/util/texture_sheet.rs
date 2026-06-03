use macroquad::prelude::*;

/// Wraps texture sheet logic around a texture.
/// A texture sheet has X frames with equal size.
/// The frame order in the sheet is top-left to bottom-right.
/// This struct provides utility methods to draw those more easily.
#[derive(Debug, Clone, PartialEq)]
pub struct TextureSheet {
    pub texture:Texture2D,
    pub frame_size:Vec2,
    pub frames_per_row:u16,
    pub frame_count:u16,
}

impl TextureSheet {
    pub fn new(texture:Texture2D, frame_size:Vec2) -> Self {
        let size = texture.size();
        let frames_per_row = (size.x / frame_size.x) as u16;
        let frames_per_column = (size.y / frame_size.y) as u16;
        let frame_count = frames_per_row * frames_per_column;
        Self {
            texture,
            frame_size,
            frames_per_row,
            frame_count,
        }
    }

    pub fn draw(&self, x:f32, y:f32, frame:u16, color:Color) {
        draw_texture_ex(
            &self.texture,
            x, y,
            color,
            DrawTextureParams {
                source: Some(self.get_frame_rect(frame)),
                ..Default::default()
            }
        );
    }

    pub fn draw_ex(&self, x:f32, y:f32, frame:u16, color:Color, rotation:f32, size:Vec2) {
        draw_texture_ex(
            &self.texture,
            x, y,
            color,
            DrawTextureParams {
                dest_size: Option::from(size),
                source: Some(self.get_frame_rect(frame)),
                rotation,
                ..Default::default()
            }
        );
    }

    pub fn extract_frame_texture(&self, frame:u16) -> Texture2D {
        let image = self.extract_frame_image(frame);
        Texture2D::from_image(&image)
    }

    pub fn extract_frame_image(&self, frame:u16) -> Image {
        let image = self.texture.get_texture_data();
        let source_rect = self.get_frame_rect(frame);
        if source_rect.x >= 0.0
            && source_rect.y >= 0.0
            && (source_rect.x + source_rect.w) <= image.width() as f32
            && (source_rect.y + source_rect.h) <= image.height() as f32
        {
            image.sub_image(source_rect)
        } else {
            // fallback: first frame
            image.sub_image(self.get_frame_rect(0))
        }
    }

    #[inline(always)]
    fn get_frame_rect(&self, mut frame: u16) -> Rect {
        if frame >= self.frame_count {
            frame = self.frame_count - 1;
        }

        let frame_x = (frame % self.frames_per_row) as f32;
        let frame_y = (frame / self.frames_per_row) as f32;

        Rect {
            x: frame_x * self.frame_size.x,
            y: frame_y * self.frame_size.y,
            w: self.frame_size.x,
            h: self.frame_size.y,
        }
    }
}