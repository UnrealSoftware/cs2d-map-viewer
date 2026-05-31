use macroquad::prelude::*;
use crate::assets::assets::Assets;

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
    pub fn draw(&mut self, delta: f32, rect: Rect, assets: &Assets) {
        clear_background(self.color);

        if self.texture.is_none() { return; }

        let tex = self.texture.as_ref().unwrap();
        let w = tex.width();
        let h = tex.height();

        let uv_scale_x = rect.w / w;
        let uv_scale_y = rect.h / h;

        let uv_offset_x = (rect.x + self.position.x) / w;
        let uv_offset_y = (rect.y + self.position.y) / h;

        assets.materials.tile.set_uniform("uv_scale", (uv_scale_x, uv_scale_y));
        assets.materials.tile.set_uniform("uv_offset", (uv_offset_x, uv_offset_y));

        gl_use_material(&assets.materials.tile);

        draw_texture_ex(
            tex,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(rect.w, rect.h)),
                ..Default::default()
            },
        );

        gl_use_default_material();

        if self.scroll_speed.x != 0 || self.scroll_speed.y != 0 {
            self.position += Vec2::new(
                self.scroll_speed.x as f32 * delta,
                self.scroll_speed.y as f32 * delta,
            );
        }
    }
}