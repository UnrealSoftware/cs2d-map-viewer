use macroquad::color::Color;
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
    InfoNoBuildings = 18,
    InfoBotNode = 19,
    InfoTeamGate = 20,
    InfoNoWeather = 80,
    InfoRadarIcon = 81,

    // Env
    EnvItem = 21,
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

impl EntityType {
    pub fn get_color(&self) -> Color {
        match self {
            EntityType::InfoT => Color::from_rgba(255, 0, 0, 255),
            EntityType::InfoCT => Color::from_rgba(0, 0, 255, 255),

            EntityType::EnvItem | EntityType::EnvSprite | EntityType::EnvSound |
            EntityType::EnvDecal |  EntityType::EnvBreakable | EntityType::EnvExplode |
            EntityType::EnvHurt | EntityType::EnvImage |  EntityType::EnvObject |
            EntityType::EnvBuilding | EntityType::EnvNpc | EntityType::EnvRoom |
            EntityType::EnvLight | EntityType::EnvLightStripe | EntityType::EnvCube3d
            => Color::from_rgba(128, 255, 0, 255),

            EntityType::GenParticles | EntityType::GenSprites | EntityType :: GenWeather |
            EntityType::GenFx
            => Color::from_rgba(255, 200, 0, 255),

            EntityType::FuncTeleport | EntityType::FuncDynamicWall | EntityType::FuncMessage |
            EntityType::FuncGameAction
            => Color::from_rgba(255, 128, 255, 255),

            EntityType::TriggerStart | EntityType::TriggerMove | EntityType::TriggerHit |
            EntityType::TriggerUse | EntityType::TriggerDelay | EntityType::TriggerOnce |
            EntityType::TriggerIf
            => Color::from_rgba(100, 200, 255, 255),

            _ => Color::from_rgba(255, 255, 0, 255)
        }
    }

    pub fn get_name(&self) -> &'static str {
        match self {
            EntityType::InfoT => "Info_T",
            EntityType::InfoCT => "Info_CT",
            EntityType::InfoVip => "Info_VIP",
            EntityType::InfoHostage => "Info_Hostage",
            EntityType::InfoRescuePoint => "Info_RescuePoint",
            EntityType::InfoBombSpot => "Info_BombSpot",
            EntityType::InfoEscapePoint => "Info_EscapePoint",
            EntityType::InfoTarget => "Info_Target",
            EntityType::InfoAnimation => "Info_Animation",
            EntityType::InfoStorm => "Info_Storm",
            EntityType::InfoTileFx => "Info_TileFX",
            EntityType::InfoNoBuying => "Info_NoBuying",
            EntityType::InfoNoWeapons => "Info_NoWeapons",
            EntityType::InfoNowFow => "Info_NoFoW",
            EntityType::InfoQuake => "Info_Quake",
            EntityType::InfoCtf => "Info_CTF_Flag",
            EntityType::InfoOldRender => "Info_OldRender",
            EntityType::InfoDom => "Info_Dom_Point",
            EntityType::InfoNoBuildings => "Info_NoBuildings",
            EntityType::InfoBotNode => "Info_BotNode",
            EntityType::InfoTeamGate => "Info_TeamGate",
            EntityType::InfoNoWeather => "Info_NoWeather",
            EntityType::InfoRadarIcon => "Info_RadarIcon",

            EntityType::EnvItem => "Env_Item",
            EntityType::EnvSprite => "Env_Sprite",
            EntityType::EnvSound => "Env_Sound",
            EntityType::EnvDecal => "Env_Decal",
            EntityType::EnvBreakable => "Env_Breakable",
            EntityType::EnvExplode => "Env_Explode",
            EntityType::EnvHurt => "Env_Hurt",
            EntityType::EnvImage => "Env_Image",
            EntityType::EnvObject => "Env_Object",
            EntityType::EnvBuilding => "Env_Building",
            EntityType::EnvNpc => "Env_NPC",
            EntityType::EnvRoom => "Env_Room",
            EntityType::EnvLight => "Env_Light",
            EntityType::EnvLightStripe => "Env_LightStripe",
            EntityType::EnvCube3d => "Env_Cube3D",

            EntityType::GenParticles => "Gen_Particles",
            EntityType::GenSprites => "Gen_Sprites",
            EntityType::GenWeather => "Gen_Weather",
            EntityType::GenFx => "Gen_FX",

            EntityType::FuncTeleport => "Func_Teleport",
            EntityType::FuncDynamicWall => "Func_DynWall",
            EntityType::FuncMessage => "Func_Message",
            EntityType::FuncGameAction => "Func_GameAction",

            EntityType::TriggerStart => "Trigger_Start",
            EntityType::TriggerMove => "Trigger_Move",
            EntityType::TriggerHit => "Trigger_Hit",
            EntityType::TriggerUse => "Trigger_Use",
            EntityType::TriggerDelay => "Trigger_Delay",
            EntityType::TriggerOnce => "Trigger_Once",
            EntityType::TriggerIf => "Trigger_If",

            _ => "unknown",
        }
    }

    pub fn get_short_name(&self) -> &'static str {
        match self {
            EntityType::InfoT => "T",
            EntityType::InfoCT => "CT",
            EntityType::InfoVip => "VIP",
            EntityType::InfoHostage => "H",
            EntityType::InfoRescuePoint => "Resq",
            EntityType::InfoBombSpot => "Bomb",
            EntityType::InfoEscapePoint => "Esc",
            EntityType::InfoTarget => "Trgt",
            EntityType::InfoAnimation => "Anim",
            EntityType::InfoStorm => "Storm",
            EntityType::InfoTileFx => "TileFX",
            EntityType::InfoNoBuying => "NoBuying",
            EntityType::InfoNoWeapons => "NoWpns",
            EntityType::InfoNowFow => "NoFoW",
            EntityType::InfoQuake => "Quake",
            EntityType::InfoCtf => "CTF",
            EntityType::InfoOldRender => "OldRender",
            EntityType::InfoDom => "Dom",
            EntityType::InfoNoBuildings => "NoBuild",
            EntityType::InfoBotNode => "BotN",
            EntityType::InfoTeamGate => "TGate",
            EntityType::InfoNoWeather => "NoWeather",
            EntityType::InfoRadarIcon => "RadIcn",

            EntityType::EnvItem => "Item",
            EntityType::EnvSprite => "Spr",
            EntityType::EnvSound => "Snd",
            EntityType::EnvDecal => "Dcl",
            EntityType::EnvBreakable => "Brkbl",
            EntityType::EnvExplode => "Expl",
            EntityType::EnvHurt => "Hurt",
            EntityType::EnvImage => "Img",
            EntityType::EnvObject => "Obj",
            EntityType::EnvBuilding => "Build",
            EntityType::EnvNpc => "NPC",
            EntityType::EnvRoom => "Room",
            EntityType::EnvLight => "Light",
            EntityType::EnvLightStripe => "LStripe",
            EntityType::EnvCube3d => "C3D",

            EntityType::GenParticles => "Particles",
            EntityType::GenSprites => "Sprites",
            EntityType::GenWeather => "Weather",
            EntityType::GenFx => "FX",

            EntityType::FuncTeleport => "Tel",
            EntityType::FuncDynamicWall => "DWall",
            EntityType::FuncMessage => "MSG",
            EntityType::FuncGameAction => "GAction",

            EntityType::TriggerStart => "Start",
            EntityType::TriggerMove => "Move",
            EntityType::TriggerHit => "Hit",
            EntityType::TriggerUse => "Use",
            EntityType::TriggerDelay => "Delay",
            EntityType::TriggerOnce => "Once",
            EntityType::TriggerIf => "If",

            _ => "unknown",
        }
    }

    #[inline]
    pub fn get_area(&self) -> u8 {
        match self {
            EntityType::InfoT | EntityType::InfoCT | EntityType::InfoBombSpot => 2,
            EntityType::InfoRescuePoint => 1,
            _ => 0
        }
    }
}