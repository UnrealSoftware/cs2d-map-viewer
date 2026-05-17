use macroquad::prelude::Texture2D;
use crate::assets::asset_type::AssetType;

#[derive(Debug, Default)]
pub struct Asset {
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
    
}