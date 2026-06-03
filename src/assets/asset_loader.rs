use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::Read;
use crate::util::texture_sheet::TextureSheet;

/// Loads and caches files in zip files.
/// Provides methods to load files from the zip files or the file system.
pub struct AssetLoader {
    zip_cache: HashMap<String, Vec<u8>>,
}

impl AssetLoader {
    pub fn new() -> Self {
        let zip_cache = HashMap::new();
        Self { zip_cache }
    }

    pub fn clear(&mut self) {
        self.zip_cache.clear();
    }

    pub async fn load_zip(&mut self, path: &str, remap: bool) -> Result<Vec<String>, String> {
        info!("unpacking zip {}...", path);

        let mut final_path = path.to_string();
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(query_pos) = final_path.find("?") {
            final_path.truncate(query_pos);
        }
        
        let mut loaded_files = Vec::new();
        let zip_bytes = load_file(&final_path)
            .await
            .map_err(|e| format!("failed to load zip: {:?}", e))?;

        let cursor = std::io::Cursor::new(&zip_bytes);
        let mut archive = zip::ZipArchive::new(cursor).map_err(|e| {
            let max_len = zip_bytes.len().min(1024 * 10);
            let preview = String::from_utf8_lossy(&zip_bytes[..max_len]);
            format!("zip parsing failed: {} | content: {:?}", e, preview)
        })?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| format!("failed to read zip index: {}", e))?;

            if !file.is_file() { continue; }

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|e| format!("failed to read file bytes: {}", e))?;

            let original_name = file.name();
            let remapped_name = if remap {
                if let Some(idx) = original_name.find("/gfx/") {
                    original_name[idx + 1..].to_string()
                } else if let Some(idx) = original_name.find("/sfx/") {
                    original_name[idx + 1..].to_string()
                } else {
                    original_name.to_string()
                }
            } else {
                original_name.to_string()
            };

            self.zip_cache.insert(remapped_name.clone().to_ascii_lowercase(), buffer);
            loaded_files.push(remapped_name.clone());
            if remap && remapped_name != original_name {
                info!("unpacked from zip: {} (mapped from {})", remapped_name, original_name);
            } else {
                info!("unpacked from zip: {}", remapped_name);
            }
        }

        Ok(loaded_files)
    }

    pub async fn load_sheet(&self, path: &str, size: Vec2) -> Result<TextureSheet, String> {
        let tex = self.load_texture(path)
            .await
            .map_err(|e| format!("Failed to load sheet from path: {:?}", e));
        Ok(TextureSheet::new(tex?, size))
    }

    /// Load file from loaded zip data or file system
    pub async fn load_file(&self, path: &str) -> Result<Vec<u8>, String> {
        info!("loading file {}", path);
        if let Some(bytes) = self.get_bytes_from_zip(path) {
            return Ok(bytes.to_vec());
        }

        load_file(path)
            .await
            .map_err(|e| format!("Failed to load data from path: {:?}", e))
    }

    /// Load texture from loaded zip data or file system
    pub async fn load_texture(&self, path: &str) -> Result<Texture2D, String> {
        info!("loading texture {}", path);
        if let Some(bytes) = self.get_bytes_from_zip(path) {
            match Image::from_file_with_format(&bytes, None) {
                Ok(image) => {
                    return Ok(Texture2D::from_image(&image));
                }
                Err(err) => {
                    error!("Failed to decode image from zip {:?}: {}", path, err);
                }
            }
        }

        load_texture(path)
            .await
            .map_err(|e| format!("failed to load texture from path: {:?}", e))
    }

    fn get_bytes_from_zip(&self, path: &str) -> Option<&[u8]> {
        self.zip_cache.get(&path.to_ascii_lowercase()).map(|v| v.as_slice())
    }
}