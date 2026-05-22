use crate::map::tile_mode::TileMode;

/// Base tile information.
/// Used for rendering and collision checks.
/// Additional data is only fetched for tiles with special modifiers.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Tile {
    /// frame index in the tile set
    pub frame: u8,

    /// modifier flags (rotation etc.)
    pub modifier: u8,

    /// cached tile mode for fast lookup, equal to `map.tile_modes[map.tiles[x].frame]`
    pub mode: TileMode,
}