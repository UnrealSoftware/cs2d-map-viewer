use crate::util::rgb::Rgb;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TileModifiers {
    pub frame: u8,
    pub overlay: u8, //todo merge frame and overlay so this struct is 4 bytes
    pub rgb: Rgb
}