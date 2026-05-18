use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::assets::asset_id::AssetId;
use crate::map::entity_type::EntityType;
use crate::TILE_SIZE;

#[derive(Debug, Clone, Default)]
pub struct Entity {
    // Main Data
    pub entity_type: EntityType,
    pub position: IVec2,
    pub state: u8,
    pub name: String,
    pub trigger: String,

    // Values
    pub ints: [i32; 10],
    pub strings: [String; 10],

    // Asset
    pub asset_id: Option<AssetId>,

    pub rotation: f32,
    pub audio_id: u32,
    pub timer: f32,
    pub ai: i32,

    // todo light engine
}

impl Entity {
    pub fn new(entity_type: EntityType, position: IVec2, name: String, trigger: String, ints: [i32; 10], strings: [String; 10]) -> Self {
        Self {
            entity_type,
            position,
            state: 0,
            name,
            trigger,
            ints,
            strings,
            ..Default::default()
        }
    }

    pub async fn setup(&mut self, assets: &mut Assets) {
        match self.entity_type {
            EntityType::EnvSprite => {
                let path = self.strings[0].as_str();
                let asset = assets.load_texture(path).await;
                self.asset_id = Some(asset.id);
            }
            EntityType::EnvSound => {
                let path = self.strings[0].as_str();
                assets.load_sound(path).await;
                let asset = assets.load_texture(path).await;
                self.asset_id = Some(asset.id);
            }
            EntityType::EnvDecal => {

            }
            EntityType::EnvImage => {

            }
            _ => {}
        }
    }

    pub fn draw(&mut self, delta: f32, rect: Rect, assets: &Assets) {
        match self.entity_type {
            EntityType::EnvSprite => {
                if /* self.state == 0 ||*/ self.strings[0].len() == 0 || self.asset_id.is_none() {
                    return
                }

                let size_x = self.ints[0] as f32;
                let size_y = self.ints[1] as f32;
                let offset_x = self.ints[2] as f32;
                let offset_y = self.ints[3] as f32;

                if self.strings[3] == "0" || self.strings[3] == "" {
                    self.rotation = (self.ints[4] as f32).to_radians();
                } else {
                    let rot_speed:f32 = self.strings[3].parse().unwrap_or(0.0);
                    self.rotation -= rot_speed.to_radians() * delta;
                }

                let rot_degrees = -self.ints[4] as f32;

                let x = self.position.x as f32 * TILE_SIZE + offset_x;
                let y = self.position.y as f32 * TILE_SIZE + offset_y;

                if self.rotation == 0.0 {
                    if x + size_x < rect.x || y + size_y < rect.y || x > rect.right() || y > rect.bottom() {
                        return;
                    }
                } else {
                    let size = size_x.max(size_y);
                    if x + size < rect.x || y + size < rect.y || x - size > rect.right() || y - size > rect.bottom() {
                        return;
                    }
                }

                let idx: usize = self.asset_id.unwrap().into();
                let asset = &assets.assets[idx];
                let tex = asset.texture2d.as_ref().unwrap();

                let r = self.ints[5] as u8;
                let g = self.ints[6] as u8;
                let b = self.ints[7] as u8;
                let a = (self.strings[1].parse().unwrap_or(1.0) * 255.0) as u8;
                let col = Color::from_rgba(r, g, b, a);

                let mut custom_mat = false;
                match self.ints[9] {
                    3 => { gl_use_material(&assets.materials.light_blend); custom_mat = true; }
                    4 => { gl_use_material(&assets.materials.shade_blend); custom_mat = true; }
                    _ => {}
                }

                draw_texture_ex(
                    &tex,
                    x,
                    y,
                    col,
                    DrawTextureParams {
                        dest_size: Some(vec2(size_x, size_y)),
                        rotation: rot_degrees.to_radians(),
                        ..Default::default()
                    },
                );

                if custom_mat {
                    gl_use_default_material();
                }
            }
            EntityType::EnvImage => {
                if /*self.state == 0 ||*/ self.strings[0].len() == 0 || self.asset_id.is_none() {
                    return
                }

                let idx: usize = self.asset_id.unwrap().into();
                let asset = &assets.assets[idx];
                let tex = asset.texture2d.as_ref().unwrap();

                let x = self.position.x as f32 * TILE_SIZE;
                let y = self.position.y as f32 * TILE_SIZE;

                draw_texture(&tex, x, y, WHITE);

            }
            _ => {}
        }
    }
}