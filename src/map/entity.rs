use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::assets::asset_id::AssetId;
use crate::map::entity_type::EntityType;
use crate::TILE_SIZE;
use crate::ui::ui_icon::UiIcon;

const STATE_ACTIVE: u8 = 0;
const STATE_INACTIVE : u8 = 1;

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
                let asset_id = assets.load_texture(path).await;
                self.asset_id = asset_id;
            }
            EntityType::EnvSound => {
                let path = self.strings[0].as_str();
                let asset_id = assets.load_sound(path).await;
                self.asset_id = asset_id;
            }
            EntityType::EnvImage => {
                let path = self.strings[0].as_str();
                let asset_id = assets.load_texture(path).await;
                self.asset_id = asset_id;
            }
            _ => {}
        }
    }

    pub fn draw(&mut self, delta: f32, rect: Rect, assets: &Assets, level: u8, deco: bool) {
        match self.entity_type {
            EntityType::EnvSprite => {
                if self.state != STATE_ACTIVE || self.asset_id.is_none() {
                    return
                }

                if level == 0 {
                    if self.strings[4].len() == 0 { return; }
                } else if level == 2 {
                    if self.strings[4].len() > 0 { return; }
                } else {
                    return;
                }

                if self.strings[8].len() > 0 && !deco { return; }

                // todo: skip nightvision
                //if self.strings[5].len() > 0 { return; }

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
                    2 => { gl_use_material(&assets.materials.premultiplied_cutoff); custom_mat = true; }
                    3 => { gl_use_material(&assets.materials.light_blend); custom_mat = true; }
                    4 => { gl_use_material(&assets.materials.shade_blend); custom_mat = true; }
                    6 => { gl_use_material(&assets.materials.lum_to_alpha_white); custom_mat = true; }
                    _ => {
                        if self.strings[2] == "4" && a >= 255 {
                            gl_use_material(&assets.materials.mask_black);
                            custom_mat = true;
                        }
                    }
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
                    assets.materials.use_default();
                }
            }
            EntityType::EnvImage => {
                if self.state != STATE_ACTIVE || self.asset_id.is_none() {
                    return
                }

                if level == 0 {
                    if self.ints[2] != 0 { return; }
                } else if level == 2  {
                    if self.ints[2] != 1 { return; }
                } else {
                    return;
                }

                let x = self.position.x as f32 * TILE_SIZE;
                let y = self.position.y as f32 * TILE_SIZE;

                let idx: usize = self.asset_id.unwrap().into();
                let asset = &assets.assets[idx];
                let tex = asset.texture2d.as_ref().unwrap();

                if x + tex.width() < rect.x || y + tex.height() < rect.y || x > rect.right() || y > rect.bottom() {
                    return;
                }

                draw_texture(&tex, x, y, WHITE);

            }
            EntityType::FuncDynamicWall => {
                //TODO
            }
            EntityType::InfoNowFow => {
                //TODO
            }
            EntityType::EnvObject => {
                if self.state != STATE_ACTIVE { return; }

                let x = self.position.x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                let y = self.position.y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

                const BUFFER: f32 = 150.0;
                if x + BUFFER < rect.x || y + BUFFER < rect.y || x - BUFFER > rect.right() || y - BUFFER > rect.bottom() {
                    return;
                }

                match self.ints[0] {
                    // 0 - Palm Tree
                    0 => { if level == 2 { self.draw_palm_tree(x, y, assets) }}
                    // 1 - Tree
                    1 => { if level == 2 { self.draw_leaf_tree(x, y, assets) }}
                    // Unknown type
                    _ => {}
                }
            }
            EntityType::TriggerUse => {
                //TODO
            }
            EntityType::InfoQuake => {
                //TODO
            }
            EntityType::InfoCtf => {
                //TODO
            }
            EntityType::InfoDom => {
                //TODO
            }
            EntityType::EnvItem => {
                //TODO
            }
            EntityType::EnvSound => {
                //TODO
            }
            EntityType::EnvBreakable => {
                //TODO
            }
            EntityType::EnvHurt => {
                //TODO
            }
            EntityType::EnvLight => {
                //TODO
            }
            EntityType::GenParticles => {
                //TODO
            }
            EntityType::GenSprites => {
                //TODO
            }
            EntityType::GenWeather => {
                //TODO
            }
            EntityType::GenFx => {
                //TODO
            }
            EntityType::TriggerDelay => {
                //TODO
            }
            EntityType::EnvCube3d => {
                //TODO
            }
            _ => {}
        }
    }

    pub fn draw_info(&mut self, assets: &Assets) {
        let size = &assets.gui_icons.frame_size;
        let x = self.position.x as f32 * TILE_SIZE + (TILE_SIZE - size.x) * 0.5;
        let y = self.position.y as f32 * TILE_SIZE + (TILE_SIZE - size.y) * 0.5;
        let color = self.entity_type.get_color();
        assets.gui_icons.draw(x, y, UiIcon::Entity as u16, color);
        /*
        // too slow for many entities
        draw_text(
            self.entity_type.get_short_name(),
            x + 17.0,
            y + 20.0,
            12.0,
            color
        );
         */
    }

    pub fn draw_palm_tree(&mut self, x: f32, y: f32, assets: &Assets) {
        let base_rot = self.ints[4] as f32;
        let scale = self.strings[0].parse().unwrap_or(1.0) * 0.8;
        let size = assets.palm_leaf.size() * scale;

        let r = self.ints[1] as u8;
        let g = self.ints[2] as u8;
        let b = self.ints[3] as u8;
        let a = (self.strings[1].parse().unwrap_or(1.0) * 255.0) as u8;
        let col = Color::from_rgba(r, g, b, a);

        const PIV_X: f32 = 15.0;
        const PIV_Y: f32 = 2.0;

        let get_anim_degrees = |i: i32, t: f32| -> f32 {
            let anim_time = t * 100.0 + (i * 20) as f32;
            let trig_val = if i % 3 == 0 {
                anim_time.to_radians().sin()
            } else {
                anim_time.to_radians().cos()
            };
            trig_val * 3.0
        };

        let t = get_time() + x as f64 - y as f64 * 2.1;

        let angle: f32 = 45.0;

        let draw_x = x - PIV_X * scale;
        let draw_y = y - PIV_Y * scale;

        let shadow_center_x = x + angle.to_radians().cos() * 4.0;
        let shadow_center_y = y + angle.to_radians().sin() * 4.0;

        let shadow_draw_x = shadow_center_x - PIV_X * scale;
        let shadow_draw_y = shadow_center_y - PIV_Y * scale;

        for i in 1..=8 {
            let rot = base_rot + (i * 45) as f32 + get_anim_degrees(i, t as f32);
            draw_texture_ex(
                &assets.palm_leaf,
                shadow_draw_x, shadow_draw_y,
                Color::new(0.0, 0.0, 0.0, 0.3),
                DrawTextureParams {
                    dest_size: Some(size),
                    rotation: rot.to_radians(),
                    pivot: Some(vec2(shadow_center_x, shadow_center_y)),
                    ..Default::default()
                },
            );
        }

        for i in 1..=8 {
            let rot = base_rot + (i * 45) as f32 + get_anim_degrees(i, t as f32);
            draw_texture_ex(
                &assets.palm_leaf,
                draw_x, draw_y,
                col,
                DrawTextureParams {
                    dest_size: Some(size),
                    rotation: rot.to_radians(),
                    pivot: Some(vec2(x, y)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn draw_leaf_tree(&mut self, x: f32, y: f32, assets: &Assets) {
        let scale = self.strings[0].parse().unwrap_or(1.0);
        let alpha = self.strings[1].parse().unwrap_or(1.0);
        let base_rot = self.ints[4] as f32;

        let dest_size = assets.tree_leafs.size() * scale;

        let col = Color::from_rgba(
            self.ints[1] as u8,
            self.ints[2] as u8,
            self.ints[3] as u8,
            (alpha * 255.0) as u8,
        );
        let shadow_color = Color::new(0.0, 0.0, 0.0, alpha);

        let t = (get_time() + x as f64 - y as f64 * 2.1) as f32;

        let shadow_angle_rad = 45.0f32.to_radians();
        let xs = shadow_angle_rad.cos() * 4.0;
        let ys = shadow_angle_rad.sin() * 4.0;

        let mut leaf_transforms: [(f32, f32, f32); 7] = [(0.0, 0.0, 0.0); 7];

        for i in 1..=7 {
            let i_f = i as f32;
            let time_base = t * 100.0 + i_f * 50.0;
            let rot_base = t + i_f * 50.0;

            let (trig_val, r_trig_val) = if i % 3 == 0 {
                (time_base.to_radians().sin(), rot_base.to_radians().sin())
            } else {
                (time_base.to_radians().cos(), rot_base.to_radians().cos())
            };

            let rot_rad = (base_rot + (i_f * 51.0) + trig_val * 2.0).to_radians();
            let r_rad = (base_rot + r_trig_val * 5.0).to_radians();

            let x_offset = rot_rad.sin() * (30.0 * scale);
            let y_offset = -(rot_rad.cos() * (30.0 * scale));

            leaf_transforms[i - 1] = (x_offset, y_offset, r_rad);
        }

        for &(x_offset, y_offset, rotation) in &leaf_transforms {
            let ex = x + x_offset;
            let ey = y + y_offset;

            draw_texture_ex(
                &assets.tree_leafs,
                ex + xs - dest_size.x / 2.0,
                ey + ys - dest_size.y / 2.0,
                shadow_color,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    rotation,
                    ..Default::default()
                },
            );
        }

        for &(x_offset, y_offset, rotation) in &leaf_transforms {
            let ex = x + x_offset;
            let ey = y + y_offset;

            draw_texture_ex(
                &assets.tree_leafs,
                ex - dest_size.x / 2.0,
                ey - dest_size.y / 2.0,
                col,
                DrawTextureParams {
                    dest_size: Some(dest_size),
                    rotation,
                    ..Default::default()
                },
            );
        }

        let r_rad_center = (base_rot + (t * 100.0).to_radians().sin() * 5.0).to_radians();
        let dest_size_center = dest_size * 1.2;

        draw_texture_ex(
            &assets.tree_leafs,
            x - dest_size_center.x / 2.0,
            y - dest_size_center.y / 2.0,
            col,
            DrawTextureParams {
                dest_size: Some(dest_size_center),
                rotation: r_rad_center,
                ..Default::default()
            },
        );
    }
}