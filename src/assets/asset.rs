use macroquad::prelude::Texture2D;
use crate::assets::asset_id::AssetId;
use crate::assets::asset_type::AssetType;

#[derive(Debug, Default, Clone)]
pub struct Asset {
    pub id:AssetId,

    /// file path with modifiers
    pub file:String,

    /// real path without modifiers
    pub real_path:String,

    pub asset_type:AssetType,
    pub mode:u8,

    pub texture2d:Option<Texture2D>,
    pub sound:Option<Vec<u8>>,

    pub data_compressed:Vec<u8>,
    pub size_uncompressed:usize
}

impl Asset {
    pub fn from_texture2d(id: AssetId, file: String, mode: u8, tex2d: Texture2D) -> Self {
        let real_path = file.clone();
        let asset_type = AssetType::Texture2D;
        let texture2d = Some(tex2d);
        Self {
            id,
            file,
            real_path,
            asset_type,
            mode,
            texture2d,
            ..Default::default()
        }
    }

    pub fn from_sound(id: AssetId, file: String, sound_data: Vec<u8>)  -> Self {
        let real_path = file.clone();
        let asset_type = AssetType::Texture2D;
        let mode = 0;
        let sound = Some(sound_data);
        Self {
            id,
            file,
            real_path,
            asset_type,
            mode,
            sound,
            ..Default::default()
        }
    }
}