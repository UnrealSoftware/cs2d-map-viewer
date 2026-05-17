#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetId(pub usize);

impl From<usize> for AssetId {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<AssetId> for usize {
    fn from(id: AssetId) -> Self {
        id.0
    }
}