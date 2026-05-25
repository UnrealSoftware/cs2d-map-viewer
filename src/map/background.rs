use macroquad::prelude::*;

/// Map background properties and state
#[derive(Debug, Default)]
pub struct MapBackground {
    pub filename: String,
    pub texture: Option<Texture2D>,

    pub is_transparent: bool,
    pub position: Vec2,

    pub scroll_speed: IVec2,
    pub scroll_tile_size: u16,
    pub scroll_like_tiles: bool,

    pub filename_old: String,
    pub texture_old: Option<Texture2D>,
    pub scroll_speed_old: IVec2,
    pub scroll_tile_size_old: u16,
    pub scroll_like_tiles_old: bool,

    pub color: Color,
}

impl MapBackground {
    pub fn draw(&mut self, delta: f32, rect: Rect) {
        if !self.texture.is_some() {
            clear_background(self.color);
            return;
        }

        let tex = self.texture.as_ref().unwrap();
        let w = tex.width();
        let h = tex.height();

        let start_x = (rect.x / w).floor() as i32;
        let start_y = (rect.y / h).floor() as i32;
        let end_x = (rect.right() / w).floor() as i32 + 2;
        let end_y = (rect.bottom() / h).floor() as i32 + 2;

        if self.scroll_like_tiles {
            let ox = (rect.x % w).floor();
            let oy = (rect.y % h).floor();
            for y in start_y..end_y {
                for x in start_x..end_x {
                    draw_texture(&tex, ox + x as f32 * w, oy + y as f32 * h, WHITE);
                }
            }
        } else {
            for y in start_y..end_y {
                for x in start_x..end_x {
                    draw_texture(&tex, x as f32 * w, y as f32 * h, WHITE);
                }
            }
        }

        if self.scroll_speed.x != 0 || self.scroll_speed.y != 0 {
            self.position += Vec2::new(
                self.scroll_speed.x as f32 * delta,
                self.scroll_speed.y as f32 * delta
            );
        }
    }
}