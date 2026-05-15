use crate::util::rgb::Rgb;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TileModifiers {
    pub frame: u8,
    pub overlay: u8,
    pub rgb: Rgb
}