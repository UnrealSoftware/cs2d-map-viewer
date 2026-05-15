use macroquad::prelude::*;
use crate::materials::Materials;

pub struct Assets {
    pub shadow_texture: Texture2D,
    pub materials: Materials
}

impl Assets {
    pub async fn load() -> Self {
        let shadow_texture = load_texture("assets/shadowmap.png")
            .await
            .expect("failed to load shadow map texture");

        let materials = Materials::load().await;

        Self {
            shadow_texture,
            materials
        }
    }
}