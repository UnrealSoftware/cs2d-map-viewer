/// A color in RGB (red, green, blue) format, each component 8 bit (0 - 255)
/// Unlike the Color struct, this one uses u8 and has no alpha component.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub frame: u8,
    pub modifier: u8,
}