/// A color in RGB (red, green, blue) format, each component 8 bit (0 - 255)
/// Unlike the Color struct, this one uses u8 and has no alpha component.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    /// frame index in the tile set
    pub frame: u8,

    /// modifier flags (rotation etc.)
    pub modifier: u8,

    /// cached tile mode for fast lookup, equal to `map.tile_modes[map.tiles[x].frame]`
    pub mode: u8,
}