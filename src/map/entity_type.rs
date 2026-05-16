use strum::FromRepr;

/// Types for [crate::map::entity::Entity].
/// [EntityType::Undefined] if failed to resolve (e.g., loading issues).
///
/// Grouped by function:
/// Info = defines the position/value of something, often relevant for game play
/// Env = environmental, either visually but sometimes also game play
/// Gen = generator, e.g., particles
/// Func = functional, performs an action when triggered
/// Trigger = triggers other entities under certain conditions
#[derive(Debug, Clone, Default, FromRepr)]
#[repr(u8)]
pub enum EntityType {
    // Info
    InfoT = 0,
    InfoCT = 1,
    InfoVip = 2,
    InfoHostage = 3,
    InfoRescuePoint = 4,
    InfoBombSpot = 5,
    InfoEscapePoint = 6,
    InfoTarget = 7,
    InfoAnimation = 8,
    InfoStorm = 9,
    InfoTileFx = 10,
    InfoNoBuying = 11,
    InfoNoWeapons = 12,
    InfoNowFow = 13,
    InfoQuake = 14,
    InfoCtf = 15,
    InfoOldRender = 16,
    InfoDom = 17,
    InfoNoBuilding = 18,
    InfoBotNode = 19,
    InfoTeamGate = 20,
    InfoNoWeather = 80,
    InfoRadarIcon = 81,

    // Env
    EnvIem = 21,
    EnvSprite= 22,
    EnvSound = 23,
    EnvDecal = 24,
    EnvBreakable = 25,
    EnvExplode = 26,
    EnvHurt = 27,
    EnvImage = 28,
    EnvObject = 29,
    EnvBuilding = 30,
    EnvNpc = 31,
    EnvRoom = 32,
    EnvLight = 33,
    EnvLightStripe = 34,
    EnvCube3d = 35,

    // Gen
    GenParticles = 50,
    GenSprites = 51,
    GenWeather = 52,
    GenFx = 53,

    // Func
    FuncTeleport = 70,
    FuncDynamicWall = 71,
    FuncMessage= 72,
    FuncGameAction = 73,

    // Trigger
    TriggerStart = 90,
    TriggerMove = 91,
    TriggerHit = 92,
    TriggerUse = 93,
    TriggerDelay = 94,
    TriggerOnce = 95,
    TriggerIf = 96,

    #[default]
    Undefined = 255
}

impl From<u8> for EntityType {
    fn from(v: u8) -> Self {
        Self::from_repr(v).unwrap_or_default()
    }
}