use strum::FromRepr;

/// Defines how a tile behaves and is rendered
#[derive(Debug, Clone, Default, FromRepr)]
#[repr(u8)]
enum TileMode {
    // Normal floor without any sound/fx
    #[default]
    Normal = 0,

    // Walls and obstacles
    Wall = 1,
    Obstacle = 2,
    WallWithoutShadow = 3,
    ObstacleWithoutShadow = 4,
    WallAtFloorLevel = 5,

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