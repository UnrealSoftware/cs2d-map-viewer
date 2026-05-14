use macroquad::prelude::*;

/// Map background properties and state
#[derive(Debug, Default)]
pub struct MapBackground {
    pub path: String,
    pub texture: Option<Texture2D>,

    pub is_transparent: bool,
    pub position: Vec2,

    pub scroll_speed: Vec2,
    pub scroll_tile_size: u16,
    pub scroll_like_tiles: bool,

    pub path_old: String,
    pub texture_old: Option<Texture2D>,
    pub scroll_speed_old: Vec2,
    pub scroll_tile_size_old: u16,
    pub scroll_like_tiles_old: bool,

    pub tile_texture: Option<Texture2D>,
    pub color: Color,
}

impl MapBackground {
    pub fn draw(&mut self, delta: f32) {
        if !self.tile_texture.is_some() {
            clear_background(self.color);
            return;
        }

        if self.scroll_like_tiles {
            // Scrolling coupled with map tiles

        } else {
            // Scrolling unrelated map

        }
    }
}