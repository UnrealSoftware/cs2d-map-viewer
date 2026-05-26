use macroquad::material::gl_use_material;
use macroquad::prelude::{vec2, Color, Rect, Vec2};
use crate::assets::assets::Assets;
use crate::map::entity_type::EntityType;
use crate::map::map::Map;
use crate::{MAP_WIDTH, TILE_SIZE};

pub const DECAL_LEVEL_FLOOR: u8 = 0;
pub const DECAL_LEVEL_OBSTACLE: u8 = 1;
pub const DECAL_LEVEL_WALL: u8 = 2;

/// A decal on the ground, spawned via an Env_Decal entity.
/// This is basically like a particle, but it's static and doesn't fade out.
/// There are also runtime decals which are displayed using the particle system.
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Decal {
    pub level: u8,
    pub frame: u8,
    pub color: Color,
    pub position: Vec2,
    pub rotation: f32,
}

#[derive(Debug, Default)]
pub struct DecalManager {
    pub decals: Vec<Decal>,

    last_rect: Rect,
    active_decals: [Vec<Decal>; 3],
}

impl DecalManager {

    pub fn cache_decal_entities(&mut self, map: &Map) {
        self.decals.clear();

        for entity in &map.entities {
            if entity.entity_type != EntityType::EnvDecal {
                continue;
            }

            if !map.is_in_bounds(entity.position) {
                continue;
            }

            let idx = (entity.position.y * MAP_WIDTH + entity.position.x) as usize;
            let level = match map.tiles[idx].mode.get_render_level() {
                2 => DECAL_LEVEL_OBSTACLE,
                3 => DECAL_LEVEL_WALL,
                _ => DECAL_LEVEL_FLOOR
            };

            let frame = entity.ints[0] as u8;

            let r = entity.ints[1] as u8;
            let g = entity.ints[2] as u8;
            let b = entity.ints[3] as u8;
            let a = (entity.strings[0].parse().unwrap_or(1.0) * 255.0) as u8;
            let color = Color::from_rgba(r, g, b, a);

            let position = vec2(entity.position.x as f32 * TILE_SIZE,
                                entity.position.y as f32 * TILE_SIZE);

            let rotation = (entity.ints[4] as f32).to_radians();

            let decal = Decal {
                level,
                frame,
                color,
                position,
                rotation
            };

            self.decals.push(decal);
        }
    }

    pub fn update_visible_rect(&mut self, rect: Rect) {
        if rect == self.last_rect {
            return;
        }

        self.last_rect = rect;
        self.update();
    }

    pub fn update(&mut self) {
        for list in &mut self.active_decals {
            list.clear();
        }

        let start_x = self.last_rect.x - TILE_SIZE;
        let start_y = self.last_rect.y - TILE_SIZE;
        let end_x = self.last_rect.right() + TILE_SIZE;
        let end_y = self.last_rect.bottom() + TILE_SIZE;

        for decal in &mut self.decals {
            if decal.position.x < start_x || decal.position.x > end_x || decal.position.y < start_y || decal.position.y > end_y {
                continue;
            }
            self.active_decals[decal.level as usize].push(*decal);
        }
    }

    pub fn draw(&mut self, assets: &Assets, level: u8) {
        gl_use_material(&assets.materials.lum_to_alpha_white);

        let list = &self.active_decals[level as usize];
        let scale = vec2(TILE_SIZE, TILE_SIZE);
        for decal in list {
            assets.decals.draw_ex(decal.position.x, decal.position.y,
                        decal.frame as u16, decal.color, decal.rotation, scale);
        }

        assets.materials.use_default();
    }
}