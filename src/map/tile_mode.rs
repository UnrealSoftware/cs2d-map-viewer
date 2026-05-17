use strum::FromRepr;
use crate::map::tile_walkability::TileWalkability;

/// Defines how a tile behaves and is rendered
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, FromRepr)]
#[repr(u8)]
pub enum TileMode {
    // Normal floor without any sound/fx
    #[default]
    Normal = 0,

    // Walls and obstacles
    Wall = 1,
    Obstacle = 2,
    WallWithoutShadow = 3,
    ObstacleWithoutShadow = 4,
    WallAtFloor = 5,
    ObstacleAtFloor = 6,

    // Regular floor tiles with different sounds
    Dirt = 10,
    Snow = 11,
    Step = 12,
    Tile = 13,
    Wade = 14,
    Metal = 15,
    Wood = 16,

    // Deadly floor tiles
    DeadlyNormal = 50,
    DeadlyToxic = 51,
    DeadlyExplosion = 52,
    DeadlyAbyss = 53,
}

impl From<u8> for TileMode {
    fn from(v: u8) -> Self {
        Self::from_repr(v).unwrap_or_default()
    }
}

impl TileMode {
    #[inline]
    pub fn get_render_level(&self) -> u8 {
        match self {
            TileMode::Wall | TileMode::WallWithoutShadow => 3,
            TileMode::Obstacle | TileMode::ObstacleWithoutShadow => 2,
            TileMode::Wade => 0,
            _ => 1
        }
    }

    #[inline]
    pub fn get_walkability(&self) -> TileWalkability {
        match self {
            TileMode::Wall | TileMode::WallWithoutShadow | TileMode::WallAtFloor => TileWalkability::Wall,
            TileMode::Obstacle | TileMode::ObstacleWithoutShadow | TileMode::ObstacleAtFloor => TileWalkability::Obstacle,
            TileMode::DeadlyNormal | TileMode::DeadlyToxic | TileMode::DeadlyExplosion | TileMode::DeadlyAbyss => TileWalkability::Deadly,
            _ => TileWalkability::Walkable
        }
    }
}