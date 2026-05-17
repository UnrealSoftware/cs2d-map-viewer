use std::collections::HashMap;
use macroquad::prelude::*;
use crate::assets::asset::Asset;
use crate::assets::asset_loader::AssetLoader;
use crate::materials::Materials;
use crate::util::texture_sheet::TextureSheet;

/// Holds assets and provides methods for asset management.
pub struct Assets {
    pub loader: AssetLoader,

    pub assets: HashMap<String, Asset>,

    pub shadow_sheet: TextureSheet,
    pub materials: Materials
}

impl Assets {
    pub async fn init(base_zip_path: &str) -> Self {
        let mut loader = AssetLoader::new();
        if !base_zip_path.is_empty() {
            loader.load_zip(base_zip_path).await.
                unwrap_or_else(|e| error!("{}", e));
        }

        let assets = HashMap::new();

        let shadow_tex = loader.load_texture("shadowmap.png")
            .await
            .expect("failed to load shadow map texture");
        let shadow_sheet = TextureSheet::new(shadow_tex, vec2(32.0, 32.0));

        let materials = Materials::load().await;

        Self {
            loader,
            assets,
            shadow_sheet,
            materials
        }
    }

    pub fn clear(&mut self) {
        self.loader.clear();
        self.assets.clear();
    }
}