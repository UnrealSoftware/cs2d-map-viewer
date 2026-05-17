#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(u8)]
pub enum TileWalkability {
    #[default]
    Wall = 0,

    Walkable = 1,
    Obstacle = 2,
    Deadly = 3,
}

impl TileWalkability {
    #[inline(always)]
    pub const fn is_passable(&self) -> bool {
        matches!(self, Self::Walkable | Self::Deadly)
    }

    #[inline(always)]
    pub const fn blocks_vision(&self) -> bool {
        matches!(self, Self::Wall)
    }
}