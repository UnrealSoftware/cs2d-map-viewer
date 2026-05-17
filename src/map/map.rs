use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::map::header::MapHeader;
use crate::map::background::MapBackground;
use crate::map::entity::Entity;
use crate::map::tile::Tile;
use crate::map::tile_modifiers::TileModifiers;
use crate::map::entity_type::EntityType;
use crate::map::tile_mode::TileMode;
use crate::util::recti::RectI;
use crate::util::texture_sheet::TextureSheet;
use crate::TILE_SIZE;

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

    pub tile_texture: Option<TextureSheet>,
    pub tile_modes: Vec<TileMode>,
    pub tile_heights: Vec<u16>,
    pub tile_3d_modifiers: Vec<u8>,
}

impl Map {
    pub fn draw(&mut self, assets: &Assets, level: i8) {
        const RAD90: f32 = std::f32::consts::FRAC_PI_2;
        const RAD180: f32 = std::f32::consts::PI;

        let tex = self.tile_texture.as_ref().unwrap();
        let size = vec2(TILE_SIZE, TILE_SIZE);

        for y in 0..self.size.y {
            for x in 0..self.size.x {

                let idx = (y * self.size.x + x) as usize;
                let tile = &self.tiles[idx];

                let mut rot = 0.0;
                let mut color = WHITE;
                let modifier = tile.modifier;

                if modifier > 0 {
                    // Rotation: Bit 1 & 2
                    if (modifier & 1) == 1 {
                        rot += RAD90;
                    }
                    if (modifier & 2) == 2 {
                        rot += RAD180;
                    }

                    // Color
                    if (modifier & 192) == 128 {
                        color = self.modifiers[idx].rgb.to_color();
                    }

                    // Brightness
                    if modifier > 3 {
                        let mut br: f32 = ((modifier & 60) >> 2) as f32;
                        if br > 0.0 {
                            br /= 10.0;
                            color = Color::new(
                                color.r * br,
                                color.g * br,
                                color.b * br,
                                1.0);
                        }
                    }
                }

                tex.draw_ex(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE,
                         tile.frame as u16, color, rot, size);
            }
        }
    }

    pub fn draw_shadows(&mut self, assets: &Assets) {
        gl_use_material(&assets.materials.grayscale_to_alpha);

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let idx = (y * self.size.x + x) as usize;
                let shadow_frame = self.shadows[idx];
                if shadow_frame == 255 { continue; }

                assets.shadow_sheet.draw(
                    x as f32 * TILE_SIZE, y as f32 * TILE_SIZE,
                    shadow_frame as u16,
                    Color::new(0.0, 0.0, 0.0, 0.3)
                );
            }
        }

        gl_use_default_material();
    }

    pub fn draw_entities(&self, assets: &Assets) {
        for entity in &self.entities {
            entity.draw(assets);
        }
    }

    pub fn map_update(&mut self, modes: bool, shadows: bool, entities: bool, area : Option<RectI>) {
        let a: RectI = if area.is_none() {
            RectI::new(0, 0, self.size.x as i32, self.size.y as i32)
        } else {
            area.unwrap()
        };

        // Update modes
        if modes {
            for x in a.x..a.x + a.width {
                for y in a.y..a.y + a. height {
                    let idx = (y * self.size.x as i32 + x) as usize;
                    let frame = self.tiles[idx].frame;
                    let mode = self.tile_modes[frame as usize];
                    self.tiles[idx].mode = mode;
                }
            }
        }

        // Update shadows
        if shadows {
            const NORMAL: TileMode = TileMode::Normal;
            const WALL: TileMode = TileMode::Wall;
            const OBSTACLE: TileMode = TileMode::Obstacle;
            for x in 0..self.size.x {
                for y in 0..self.size.y {
                    let idx = (y * self.size.x + x) as usize;
                    self.shadows[idx] = 255;
                    if x > 0 && y > 0 {
                        let mode = self.tiles[idx].mode;
                        if mode == WALL || mode == OBSTACLE || mode == NORMAL {
                            // no shadow
                        } else {
                            // x shadow
                            let idx_left = (y * self.size.x + x - 1) as usize;
                            if self.tiles[idx_left].mode == WALL { self.shadows[idx] = 6 }
                            if self.tiles[idx_left].mode == OBSTACLE { self.shadows[idx] = 7 }

                            // y shadow
                            let idx_top = ((y - 1) * self.size.x + x) as usize;
                            if self.tiles[idx_top].mode == WALL {
                                if self.shadows[idx] == 255 { self.shadows[idx] = 0 }
                                if self.shadows[idx] == 6 { self.shadows[idx] = 10 }
                                if self.shadows[idx] == 7 { self.shadows[idx] = 11 }
                            } else if self.tiles[idx_top].mode == OBSTACLE {
                                if self.shadows[idx] == 255 { self.shadows[idx] = 1 }
                                if self.shadows[idx] == 6 { self.shadows[idx] = 12 }
                                if self.shadows[idx] == 7 { self.shadows[idx] = 13 }
                            }

                            let idx_topleft = ((y - 1) * self.size.x + x - 1) as usize;

                            // shadow edges
                            if self.shadows[idx] == 255 {
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
}