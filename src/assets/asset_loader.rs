use macroquad::prelude::*;
use std::collections::HashMap;
use std::io::Read;

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

    pub async fn load_zip(&mut self, url: &str) -> Result<(), String> {
        let zip_bytes = load_file(url)
            .await
            .map_err(|e| format!("Failed to load zip: {:?}", e))?;

        let cursor = std::io::Cursor::new(zip_bytes);
        let mut archive = zip::ZipArchive::new(cursor)
            .map_err(|e| format!("Invalid zip archive: {}", e))?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| format!("Failed to read zip index: {}", e))?;

            if file.is_file() {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)
                    .map_err(|e| format!("Failed to read file bytes: {}", e))?;

                self.zip_cache.insert(file.name().to_string(), buffer);

                //info!("unpacked zip file: {}", file.name());
            }
        }

        Ok(())
    }

    /// Load file from loaded zip data or file system
    pub async fn load_file(&self, path: &str) -> Result<Vec<u8>, String> {
        if let Some(bytes) = self.get_bytes_from_zip(path) {
            return Ok(bytes.to_vec());
        }

        load_file(path)
            .await
            .map_err(|e| format!("Failed to load data from path: {:?}", e))
    }

    /// Load texture from loaded zip data or file system
    pub async fn load_texture(&self, path: &str) -> Result<Texture2D, String> {
        if let Some(bytes) = self.get_bytes_from_zip(path) {
            return Ok(Texture2D::from_file_with_format(&bytes, None));
        }

        load_texture(path)
            .await
            .map_err(|e| format!("Failed to load texture from path: {:?}", e))
    }

    fn get_bytes_from_zip(&self, path: &str) -> Option<&[u8]> {
        self.zip_cache.get(path).map(|v| v.as_slice())
    }
}