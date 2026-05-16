use macroquad::prelude::*;
use crate::materials::Materials;
use crate::util::texture_sheet::TextureSheet;

pub struct Assets {
    pub shadow_sheet: TextureSheet,
    pub materials: Materials
}

impl Assets {
    pub async fn load() -> Self {
        let shadow_tex = load_texture("assets/shadowmap.png")
            .await
            .expect("failed to load shadow map texture");
        let shadow_sheet = TextureSheet::new(shadow_tex, IVec2::new(32, 32));

        let materials = Materials::load().await;

        Self {
            shadow_sheet,
            materials
        }
    }
}