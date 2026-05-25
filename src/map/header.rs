#[derive(Debug, Default)]
pub struct MapHeader {
    pub has_modifiers: bool,
    pub save_tile_heights: u8,
    pub use_64_pixel_tiles: bool,

    pub uptime_ms: i32,
    pub usgn_id: i32,
    pub daylight_time: i32,

    pub author_name: String,
    pub tool_name: String,
}