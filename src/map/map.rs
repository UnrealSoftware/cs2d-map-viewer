use macroquad::prelude::*;
use crate::assets::Assets;
use crate::map::header::MapHeader;
use crate::map::background::MapBackground;
use crate::map::entity::Entity;
use crate::map::tile::Tile;
use crate::map::tile_modifiers::TileModifiers;
use crate::util::recti::RectI;
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

    pub tile_texture: Option<Texture2D>,
    pub tiles_per_row: u8,

    pub tile_modes: Vec<u8>,
    pub tile_heights: Vec<u16>,
    pub tile_3d_modifiers: Vec<u8>,
}

impl Map {
    pub fn draw(&mut self, assets: &Assets, level: i8) {
        const RAD90: f32 = std::f32::consts::FRAC_PI_2;
        const RAD180: f32 = std::f32::consts::PI;

        let tex = self.tile_texture.as_ref().unwrap();

        for y in 0..self.size.y {
            for x in 0..self.size.x {

                let idx = (y * self.size.x + x) as usize;
                let tile = self.tiles[idx];
                let tile_index = tile.frame;
                let tile_x = tile_index % self.tiles_per_row;
                let tile_y = tile_index / self.tiles_per_row;

                let mut rot = 0.0;
                let mut color = WHITE;
                let modifier = tile.modifier;

                if (modifier > 0) {
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

                draw_texture_ex(
                    &tex,
                    x as f32 * TILE_SIZE, y as f32 * TILE_SIZE,
                    color,
                    DrawTextureParams {
                        source: Option::from(Rect {
                            x: tile_x as f32 * TILE_SIZE,
                            y: tile_y as f32 * TILE_SIZE,
                            w: TILE_SIZE,
                            h: TILE_SIZE
                        }),
                        rotation: rot,
                        ..Default::default()
                    }
                );
            }
        }
    }

    pub fn draw_shadows(&mut self, assets: &Assets) {
        gl_use_material(&assets.materials.grayscale_to_alpha);
        let tiles_per_row = (assets.shadow_texture.width() / 32.0) as u8;

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let idx = (y * self.size.x + x) as usize;
                let shadow_index = self.shadows[idx];

                if shadow_index == 255 { continue; }

                let shadow_x = shadow_index % tiles_per_row;
                let shadow_y = shadow_index / tiles_per_row;

                draw_texture_ex(
                    &assets.shadow_texture,
                    x as f32 * TILE_SIZE, y as f32 * TILE_SIZE,
                    Color::new(0.0, 0.0, 0.0, 0.3),
                    DrawTextureParams {
                        source: Option::from(Rect {
                            x: shadow_x as f32 * TILE_SIZE,
                            y: shadow_y as f32 * TILE_SIZE,
                            w: TILE_SIZE,
                            h: TILE_SIZE
                        }),
                        ..Default::default()
                    }
                );
            }
        }

        gl_use_default_material();
    }

    pub fn map_update(&mut self, modes: bool, shadows: bool, entities: bool, area : Option<RectI>) {
        let a: RectI = if area.is_none() {
            RectI::new(0, 0, self.size.x as i32, self.size.y as i32)
        } else {
            area.unwrap()
        };

        // Update modes
        if (modes) {
            for x in 0..self.size.x {
                for y in 0..self.size.y {
                    let idx = (y * self.size.x + x) as usize;
                    let frame = self.tiles[idx].frame;
                    let mode = self.tile_modes[frame as usize];
                    self.tiles[idx].mode = mode;
                }
            }
        }

        // Update shadows
        if (shadows) {
            for x in 0..self.size.x {
                for y in 0..self.size.y {
                    let idx = (y * self.size.x + x) as usize;
                    self.shadows[idx] = 255;
                    if x > 0 && y > 0 {
                        let mode = self.tiles[idx].mode;
                        if mode == 1 || mode == 2 || mode == 0 {
                            // no shadow
                        } else {
                            // x shadow
                            let idx_left = (y * self.size.x + x - 1) as usize;
                            if self.tiles[idx_left].mode == 1 { self.shadows[idx] = 6 }
                            if self.tiles[idx_left].mode == 2 { self.shadows[idx] = 7 }

                            // y shadow
                            let idx_top = ((y - 1) * self.size.x + x) as usize;
                            if self.tiles[idx_top].mode == 1 {
                                if self.shadows[idx] == 255 { self.shadows[idx] = 0 }
                                if self.shadows[idx] == 6 { self.shadows[idx] = 10 }
                                if self.shadows[idx] == 7 { self.shadows[idx] = 11 }
                            } else if self.tiles[idx_top].mode == 2 {
                                if self.shadows[idx] == 255 { self.shadows[idx] = 1 }
                                if self.shadows[idx] == 6 { self.shadows[idx] = 12 }
                                if self.shadows[idx] == 7 { self.shadows[idx] = 13 }
                            }

                            let idx_topleft = ((y - 1) * self.size.x + x - 1) as usize;

                            // shadow edges
                            if self.shadows[idx] == 255 {
                                if self.tiles[idx_topleft].mode == 1 { self.shadows[idx] = 4 }
                                if self.tiles[idx_topleft].mode == 2 { self.shadows[idx] = 5 }
                            }

                            // round off x edges
                            if self.shadows[idx] == 0 || self.shadows[idx] == 1 {
                                let mode = self.tiles[idx_topleft].mode;
                                if mode != 1 && mode != 2 {
                                    self.shadows[idx] += 2;
                                }
                            }

                            // round off y edges
                            if self.shadows[idx] == 6 || self.shadows[idx] == 7 {
                                let mode = self.tiles[idx_topleft].mode;
                                if mode != 1 && mode != 2 {
                                    self.shadows[idx] += 2;
                                }
                            }

                            // x edge mixed
                            if self.shadows[idx] == 0 {
                                if self.tiles[idx_topleft].mode == 2 { self.shadows[idx] = 16 }
                            } else if self.shadows[idx] == 1 {
                                if self.tiles[idx_topleft].mode == 1 { self.shadows[idx] = 17 }
                            }

                            // y edge mixed
                            if self.shadows[idx] == 6 {
                                if self.tiles[idx_topleft].mode == 2 { self.shadows[idx] = 14 }
                            } else if self.shadows[idx] == 7 {
                                if self.tiles[idx_topleft].mode == 1 { self.shadows[idx] = 15 }
                            }
                        }
                    }
                }
            }
        }


        // Update entities
        if (entities) {
            //todo
        }
    }
}