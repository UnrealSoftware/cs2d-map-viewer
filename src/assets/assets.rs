use std::collections::HashMap;
use macroquad::prelude::*;
use crate::assets::asset::Asset;
use crate::assets::asset_id::AssetId;
use crate::assets::asset_loader::AssetLoader;
use crate::materials::Materials;
use crate::util::texture_sheet::TextureSheet;

/// Holds assets and provides methods for asset management.
pub struct Assets {
    pub loader: AssetLoader,

    pub assets: Vec<Asset>,
    pub lookup: HashMap<String, AssetId>,

    pub shadow_map: TextureSheet,
    pub blend_map: TextureSheet,
    pub decals: TextureSheet,
    pub palm_leaf: Texture2D,
    pub tree_leafs: Texture2D,

    pub materials: Materials,
}

impl Assets {
    pub async fn init(base_zip_path: &str) -> Self {
        let mut loader = AssetLoader::new();
        if !base_zip_path.is_empty() {
            if let Err(e) = loader.load_zip(base_zip_path, false).await {
                error!("Failed to load zip '{}': {}", base_zip_path, e);
            }
        }

        let assets = Vec::new();
        let lookup = HashMap::new();

        let shadow_map = loader.load_sheet("gfx/shadowmap.bmp", vec2(32.0, 32.0)).await.unwrap();
        let blend_map = loader.load_sheet("gfx/blendmap.bmp", vec2(32.0, 32.0)).await.unwrap();
        let decals = loader.load_sheet("gfx/decals.bmp", vec2(32.0, 32.0)).await.unwrap();
        let palm_leaf = loader.load_texture("incbin/palmleaf.png").await.unwrap();
        let tree_leafs = loader.load_texture("incbin/treeleafs.png").await.unwrap();

        let materials = Materials::load().await;

        Self {
            loader,
            assets,
            lookup,
            shadow_map,
            blend_map,
            decals,
            palm_leaf,
            tree_leafs,
            materials
        }
    }

    pub fn clear(&mut self) {
        self.loader.clear();
        self.assets.clear();
        self.lookup.clear();
    }

    pub async fn load_texture(&mut self, path: &str) -> Option<AssetId> {
        let result = self.lookup.get(path);
        if result.is_some() {
            return result.copied();
        }
        let tex = self.loader.load_texture(path).await;
        if tex.is_err() {
            error!("Failed to load texture '{}': {}", path, tex.err().unwrap());
            return None;
        }
        let id = self.assets.len().into();
        let tex_asset = Asset::from_texture2d(id, path.to_string(), 0, tex.unwrap());
        self.lookup.insert(path.to_string(), id);
        self.assets.push(tex_asset);
        Some(id)
    }

    pub async fn load_sound(&mut self, path: &str) -> Option<AssetId> {
        let result = self.lookup.get(path);
        if result.is_some() {
            return result.copied();
        }
        let bytes = self.loader.load_file(path).await;
        if bytes.is_err() {
            error!("Failed to load sound '{}': {}", path, bytes.err().unwrap());
            return None;
        }
        let id = self.assets.len().into();
        let sound_asset = Asset::from_sound(id, path.to_string(), bytes.unwrap());
        self.lookup.insert(path.to_string(), id);
        self.assets.push(sound_asset);
        Some(id)
    }
}