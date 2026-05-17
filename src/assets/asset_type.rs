use strum::FromRepr;

#[derive(Debug, Clone, Default, FromRepr)]
#[repr(u8)]
pub enum AssetType {
    #[default]
    Undefined = 0,
    Texture2D = 1,
    Sound = 2,
}

impl From<u8> for AssetType {
    fn from(v: u8) -> Self {
        Self::from_repr(v).unwrap_or_default()
    }
}