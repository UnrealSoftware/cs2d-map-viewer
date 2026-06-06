use crate::map::tile_blend::TileBlend;
use crate::map::tile_fx::TileFxManager;
use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::map::header::MapHeader;
use crate::map::background::MapBackground;
use crate::map::entity::Entity;
use crate::map::tile::Tile;
use crate::map::tile_modifiers::TileModifiers;
use crate::map::entity_type::EntityType;
use crate::map::tile_mode::TileMode;
use crate::map::tile_walkability::TileWalkability;
use crate::util::recti::RectI;
use crate::util::texture_sheet::TextureSheet;
use crate::TILE_SIZE;
use crate::ui::ui_icon::UiIcon;

const RAD90: f32 = std::f32::consts::FRAC_PI_2;
const RAD180: f32 = std::f32::consts::PI;

#[derive(Debug, Default)]
pub struct Map {
    pub path: String,
    pub header: MapHeader,
    pub background: MapBackground,

    pub size: U16Vec2,

    pub tiles: Vec<Tile>,
    pub modifiers: Vec<TileModifiers>,
    pub shadows: Vec<u8>,
    pub entity_areas: Vec<u8>,

    pub entities: Vec<Entity>,

    pub tile_texture_filename: String,
    pub tile_texture: Option<TextureSheet>,
    pub tile_modes: Vec<TileMode>,
    pub tile_heights: Vec<u16>,
    pub tile_3d_modifiers: Vec<u8>,
    pub tile_fx: TileFxManager,
    pub tile_blend: Vec<TileBlend>,
}

impl Map {
    pub fn draw(&mut self, rect: Rect, assets: &Assets, level: u8) {

        let tex = self.tile_texture.as_ref().unwrap();
        let size = vec2(TILE_SIZE, TILE_SIZE);
        let (start_x, start_y, end_x, end_y) = self.get_update_bounds(rect);

        gl_use_material(&assets.materials.mask_magenta);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = y * self.size.x as usize + x;
                let tile = &self.tiles[idx];

                if tile.mode.get_render_level() != level {
                    continue;
                }
                
                let mut rot = 0.0;
                let mut color = WHITE;
                let modifier = tile.modifier;

                if modifier > 0 {
                    rot = get_tile_rotation(modifier);
                    color = self.get_tile_color(modifier, idx);

                    // Blended Tile
                    if modifier >= 64 {
                        let blend_idx = self.modifiers[idx].blend as usize;
                        if blend_idx < self.tile_blend.len() {
                            let blend = &self.tile_blend[blend_idx];
                            draw_texture_ex(
                                &blend.texture,
                                x as f32 * TILE_SIZE,
                                y as f32 * TILE_SIZE,
                                color,
                                DrawTextureParams {
                                    dest_size: Some(size),
                                    rotation: rot,
                                    ..Default::default()
                                },
                            );
                            continue;
                        }
                    }
                }

                let fx_mapping = self.tile_fx.mapping[tile.frame as usize];
                if fx_mapping == usize::MAX {
                    // Regular tile
                    tex.draw_ex(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE,
                                tile.frame as u16, color, rot, size);
                } else {
                    // FX tile
                    let effect = &self.tile_fx.effects[fx_mapping];
                    if effect.frames.len() == 0 {
                        let draw_frame = effect.current_frame as u16;
                        if draw_frame < tex.frame_count {
                            tex.draw_ex(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE,
                                        draw_frame, color, rot, size);
                        }
                    } else {
                        draw_texture_ex(
                            &effect.frames[effect.current_frame],
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            color,
                            DrawTextureParams {
                                dest_size: Some(size),
                                rotation: rot,
                                ..Default::default()
                            },
                        )
                    }
                }
            }
        }

        assets.materials.use_default();
    }

    pub fn draw_shadows(&mut self, rect: Rect, assets: &Assets) {
        let (start_x, start_y, end_x, end_y) = self.get_update_bounds(rect);

        gl_use_material(&assets.materials.grayscale_to_alpha);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = y * self.size.x as usize + x;
                let shadow_frame = self.shadows[idx];
                if shadow_frame == 255 { continue; }

                assets.shadow_map.draw(
                    x as f32 * TILE_SIZE, y as f32 * TILE_SIZE,
                    shadow_frame as u16,
                    Color::new(0.0, 0.0, 0.0, 0.3)
                );
            }
        }

        assets.materials.use_default();
    }

    pub fn draw_entities(&mut self, delta:f32, rect: Rect, assets: &Assets, level: u8, deco: bool) {
        for entity in &mut self.entities {
            entity.draw(delta, rect, assets, level, deco);
        }
    }

    pub fn draw_entity_info(&mut self, rect: Rect, assets: &Assets) {
        gl_use_material(&assets.materials.lum_to_alpha_white);

        let (start_x, start_y, end_x, end_y) = self.get_update_bounds(rect);

        for entity in &mut self.entities {
            if entity.position.x < start_x as i32 ||
                entity.position.y < start_y as i32 ||
                entity.position.x > end_x as i32 ||
                entity.position.y > end_y as i32 {
                continue;
            }

            entity.draw_info(assets);
        }

        assets.materials.use_default();
    }

    pub fn draw_grid(&mut self, rect: Rect, assets: &Assets) {
        let (start_x, start_y, end_x, end_y) = self.get_update_bounds(rect);
        let col = WHITE;

        gl_use_material(&assets.materials.invert);

        let start_x_px = start_x as f32 * TILE_SIZE;
        let start_y_px = start_y as f32 * TILE_SIZE;

        let close_x = end_x == self.size.x as usize;
        let close_y = end_y == self.size.y as usize;

        let grid_w = (end_x - start_x) as f32 * TILE_SIZE + if close_x { 1.0 } else { 0.0 };
        let grid_h = (end_y - start_y) as f32 * TILE_SIZE + if close_y { 1.0 } else { 0.0 };

        let x_limit = if close_x { end_x + 1 } else { end_x };
        let y_limit = if close_y { end_y + 1 } else { end_y };

        for y in start_y..y_limit {
            draw_rectangle(
                start_x_px,
                y as f32 * TILE_SIZE,
                grid_w,
                1.0,
                col
            );
        }

        for x in start_x..x_limit {
            draw_rectangle(
                x as f32 * TILE_SIZE,
                start_y_px,
                1.0,
                grid_h,
                col
            );
        }

        assets.materials.use_default();
    }

    pub fn draw_tile_overlays(&mut self, rect: Rect, assets: &Assets) {
        let (start_x, start_y, end_x, end_y) = self.get_update_bounds(rect);

        gl_use_material(&assets.materials.light_blend);

        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = y * self.size.x as usize + x;
                let tile = &self.tiles[idx];
                let tx = x as f32 * TILE_SIZE;
                let ty = y as f32 * TILE_SIZE;

                match tile.mode.get_walkability() {
                    TileWalkability::Wall => draw_rectangle(tx, ty, TILE_SIZE, TILE_SIZE, Color::from_rgba(128, 0, 0, 255)),
                    TileWalkability::Obstacle => draw_rectangle(tx, ty, TILE_SIZE, TILE_SIZE, Color::from_rgba(128, 64, 0, 255)),
                    TileWalkability::Deadly => {
                        let red = Color::from_rgba(255, 0, 0, 255);
                        draw_line(tx + 5.0, ty + 5.0, tx + 27.0, ty + 27.0, 5.0, red);
                        draw_line(tx + 27.0, ty + 5.0, tx + 5.0, ty + 27.0, 5.0, red);
                    }
                    _ => {}
                }

                let modifier = tile.modifier;
                if modifier > 0 {
                    // Rotation
                    if modifier & (1 + 2) > 0 {
                        let mut rot_frame: u16 = 0;
                        if modifier & 1 == 1 {
                            rot_frame = 1;
                            if modifier & 2 == 2 {
                                rot_frame = 3;
                            }
                        } else {
                            rot_frame = 2;
                        }
                        assets.gui_icons.draw(
                            tx, ty,
                            UiIcon::ArrowUp as u16 + rot_frame,
                            Color::from_rgba(0, 255,0, 255));
                    }

                    // Brightness
                    if modifier & (1 + 2) > 3 {
                        if ((modifier & (4 + 8 + 16 + 32)) >> 2) > 0 {
                            assets.gui_icons.draw(
                                tx, ty + 16.0,
                                UiIcon::Entity as u16,
                                Color::from_rgba(255, 0,0, 255));
                        }
                    }

                    if modifier & (64 + 128) == 64 {
                        // Blending
                        let blend_frame = &self.modifiers[idx].frame % 8;
                        let icon_frame = match blend_frame {
                            0 => UiIcon::ArrowUp,
                            1 => UiIcon::ArrowUpRight,
                            2 => UiIcon::ArrowRight,
                            3 => UiIcon::ArrowDownRight,
                            4 => UiIcon::ArrowDown,
                            5 => UiIcon::ArrowDownLeft,
                            6 => UiIcon::ArrowLeft,
                            7 => UiIcon::ArrowUpLeft,
                            _ => UiIcon::X
                        } as u16;
                        assets.gui_icons.draw(
                            tx + 8.0, ty + 8.0,
                            icon_frame,
                            Color::from_rgba(255, 128,0, 255));
                    } else if modifier & (64 + 128) == 128 {
                        // Color
                        assets.gui_icons.draw(
                            tx + 19.0, ty,
                            UiIcon::Fill as u16,
                            Color::from_rgba(255, 0,0, 255));
                    }
                }
            }
        }

        assets.materials.use_default();
    }

    pub fn map_update(&mut self, modes: bool, shadows: bool, entities: bool, area : Option<RectI>) {
        let a: RectI = if area.is_none() {
            RectI::new(0, 0, self.size.x as i32, self.size.y as i32)
        } else {
            area.unwrap()
        };

        // Update modes
        let tile_modes = self.tile_modes.len() as u8;
        if modes {
            for x in a.x..a.x + a.width {
                for y in a.y..a.y + a. height {
                    let idx = (y * self.size.x as i32 + x) as usize;
                    let frame = self.tiles[idx].frame;
                    let mode = if frame < tile_modes {
                        self.tile_modes[frame as usize]
                    } else {
                        TileMode::default()
                    };
                    self.tiles[idx].mode = mode;
                }
            }
        }

        // Update shadows
        if shadows {
            const UNDEFINED: u8 = 255;
            const NORMAL: TileMode = TileMode::Normal;
            const WALL: TileMode = TileMode::Wall;
            const OBSTACLE: TileMode = TileMode::Obstacle;

            for x in 0..self.size.x {
                for y in 0..self.size.y {
                    let idx = (y * self.size.x + x) as usize;
                    self.shadows[idx] = UNDEFINED;

                    if x <= 0 || y <= 0 { continue; }

                    let mode = self.tiles[idx].mode;
                    if mode == WALL || mode == OBSTACLE || mode == NORMAL { continue; }

                    // x shadow
                    let idx_left = (y * self.size.x + x - 1) as usize;
                    if self.tiles[idx_left].mode == WALL { self.shadows[idx] = 6 }
                    if self.tiles[idx_left].mode == OBSTACLE { self.shadows[idx] = 7 }

                    // y shadow
                    let idx_top = ((y - 1) * self.size.x + x) as usize;
                    if self.tiles[idx_top].mode == WALL {
                        if self.shadows[idx] == UNDEFINED { self.shadows[idx] = 0 }
                        if self.shadows[idx] == 6 { self.shadows[idx] = 10 }
                        if self.shadows[idx] == 7 { self.shadows[idx] = 11 }
                    } else if self.tiles[idx_top].mode == OBSTACLE {
                        if self.shadows[idx] == UNDEFINED { self.shadows[idx] = 1 }
                        if self.shadows[idx] == 6 { self.shadows[idx] = 12 }
                        if self.shadows[idx] == 7 { self.shadows[idx] = 13 }
                    }

                    let idx_topleft = ((y - 1) * self.size.x + x - 1) as usize;

                    // shadow edges
                    if self.shadows[idx] == UNDEFINED {
                        if self.tiles[idx_topleft].mode == WALL { self.shadows[idx] = 4 }
                        if self.tiles[idx_topleft].mode == OBSTACLE { self.shadows[idx] = 5 }
                    }

                    // round off x edges
                    if self.shadows[idx] == 0 || self.shadows[idx] == 1 {
                        let mode = self.tiles[idx_topleft].mode;
                        if mode != WALL && mode != OBSTACLE {
                            self.shadows[idx] += 2;
                        }
                    }

                    // round off y edges
                    if self.shadows[idx] == 6 || self.shadows[idx] == 7 {
                        let mode = self.tiles[idx_topleft].mode;
                        if mode != WALL && mode != OBSTACLE {
                            self.shadows[idx] += 2;
                        }
                    }

                    // x edge mixed
                    if self.shadows[idx] == 0 {
                        if self.tiles[idx_topleft].mode == OBSTACLE { self.shadows[idx] = 16 }
                    } else if self.shadows[idx] == 1 {
                        if self.tiles[idx_topleft].mode == WALL { self.shadows[idx] = 17 }
                    }

                    // y edge mixed
                    if self.shadows[idx] == 6 {
                        if self.tiles[idx_topleft].mode == OBSTACLE { self.shadows[idx] = 14 }
                    } else if self.shadows[idx] == 7 {
                        if self.tiles[idx_topleft].mode == WALL { self.shadows[idx] = 15 }
                    }
                }
            }
        }

        // Update entities
        if entities {
            for i in 0..self.entity_areas.len() {
                self.entity_areas[i] = 0;
            }
            let mut teleporters = 0;

            for entity in &self.entities {
                match entity.entity_type {
                    EntityType::InfoTeamGate => Self::set_entity_area(
                        &self.size, &mut self.entity_areas,
                        entity.position.x, entity.position.y,
                        entity.position.x + entity.ints[0], entity.position.y + entity.ints[1]),
                    EntityType::EnvHurt => Self::set_entity_area(
                        &self.size, &mut self.entity_areas,
                        entity.position.x, entity.position.y,
                        entity.position.x + entity.ints[2], entity.position.y + entity.ints[3]),
                    _ => {
                        let area_size = entity.entity_type.get_area();
                        if area_size == 0 {
                            if self.is_in_bounds(entity.position) {
                                let idx = (entity.position.y * self.size.x as i32 + entity.position.x) as usize;
                                self.entity_areas[idx] = 1;
                            }
                        } else {
                            Self::set_entity_area(
                                &self.size, &mut self.entity_areas,
                                entity.position.x - area_size, entity.position.y - area_size,
                                entity.position.x + area_size, entity.position.y + area_size);
                        }
                    }
                }
                match entity.entity_type {
                    EntityType::EnvBreakable => if self.is_in_bounds(entity.position) {
                            let idx = (entity.position.y * self.size.x as i32 + entity.position.x) as usize;
                            self.tiles[idx].mode = match entity.ints[6] {
                                1 => TileMode::Wall,
                                2 => TileMode::Normal,
                                3 => TileMode::Obstacle,
                                _ => TileMode::Wall
                            }
                        }
                    EntityType::FuncTeleport => { teleporters += 1; }
                    EntityType::FuncDynamicWall =>
                        if entity.state == 0 && self.is_in_bounds(entity.position) {
                            let idx = (entity.position.y * self.size.x as i32 + entity.position.x) as usize;
                            self.tiles[idx].mode = match entity.ints[1] {
                                0 => TileMode::Wall,
                                1 => TileMode::Obstacle,
                                2 => TileMode::WallWithoutShadow,
                                3 => TileMode::ObstacleWithoutShadow,
                                4 => if entity.ints[0] >= 0 && (entity.ints[0] as usize) < self.tile_modes.len() {
                                        self.tile_modes[entity.ints[0] as usize]
                                    } else {
                                        TileMode::Wall
                                    }
                                _ => TileMode::Wall
                            }
                        }
                    _ => {}
                }
            }

            // Teleporter Exits
            if teleporters > 0 {
                for entity in &self.entities {
                    if entity.entity_type == EntityType::FuncTeleport && self.is_in_bounds(entity.position) {
                        let idx = (entity.position.y * self.size.x as i32 + entity.position.x) as usize;
                        self.entity_areas[idx] = 2;
                    }
                }
            }

            // Dynamic Objects
            //todo
        }
    }

    pub fn is_in_bounds(&self, position: IVec2) -> bool {
        return position.x >= 0 &&
            position.y >= 0 &&
            position.x < self.size.x as i32 &&
            position.y < self.size.y as i32;
    }

    pub fn get_entity(&self, index: usize) -> Option<&Entity> {
        if index < self.entities.len() {
            return Some(&self.entities[index]);
        }
        None
    }

    pub fn get_entity_at_position(&self, position: IVec2) -> Option<&Entity> {
        for entity in &self.entities {
            if entity.position == position {
                return Some(&entity);
            }
        }
        None
    }

    pub fn get_entities_with_name<'a>(&'a self, name: &str, result: &mut Vec<&'a Entity>) {
        result.clear();
        result.extend(self.entities.iter().filter(|e| e.name == name));
    }

    fn set_entity_area(size: &U16Vec2, areas: &mut Vec<u8>, mut start_x:i32, mut start_y:i32, mut end_x:i32, mut end_y:i32) {
        if start_x < 0  { start_x = 0 }
        if start_y < 0  { start_y = 0 }
        if end_x > size.x as i32 { end_x = size.x as i32 }
        if end_y > size.y as i32 { end_y = size.y as i32 }
        for x in start_x..end_x {
            for y in start_y..end_y {
                let idx = (y as u16 * size.x + x as u16) as usize;
                areas[idx] = 1;
            }
        }
    }

    #[inline]
    pub fn get_update_bounds(&self, rect: Rect) -> (usize, usize, usize, usize) {
        let start_x = (rect.x.max(0.0) / TILE_SIZE).floor() as usize;
        let start_y = (rect.y.max(0.0) / TILE_SIZE).floor() as usize;

        let end_x = ((rect.right().max(0.0) / TILE_SIZE).ceil() as usize).min(self.size.x as usize);
        let end_y = ((rect.bottom().max(0.0) / TILE_SIZE).ceil() as usize).min(self.size.y as usize);

        (start_x, start_y, end_x, end_y)
    }

    #[inline]
    pub fn get_tile_color(&self, modifier: u8, idx: usize) -> Color {
        let mut color: Color = WHITE;

        // Color
        if (modifier & 192) == 128 {
            color = self.modifiers[idx].rgb.to_color();
        }

        // Brightness
        if modifier > 3 {
            let mut br: f32 = ((modifier & (4 + 8 + 16 + 32)) >> 2) as f32;
            if br > 0.0 {
                br /= 10.0;
                color = Color::new(
                    color.r * br,
                    color.g * br,
                    color.b * br,
                    1.0);
            }
        }

        color
    }
}

#[inline]
fn get_tile_rotation(modifier: u8) -> f32 {
    let mut rot = 0.0;
    if (modifier & 1) == 1 {
        rot += RAD90;
    }
    if (modifier & 2) == 2 {
        rot += RAD180;
    }
    rot
}