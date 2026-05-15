use std::io::{self, Read};
use macroquad::prelude::*;
use crate::map::header::MapHeader;
use crate::map::background::MapBackground;
use crate::map::entity::Entity;
use crate::map::tile::Tile;
use crate::map::tile_modifiers::TileModifiers;
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
    pub entity: Vec<u8>,

    pub entities: Vec<Entity>,

    pub tile_texture: Option<Texture2D>
}

impl Map {
    pub fn draw(&mut self, level: i8) {
        const rad_90: f32 = std::f32::consts::FRAC_PI_2;
        const rad_180: f32 = std::f32::consts::PI;

        let tex = self.tile_texture.as_ref().unwrap();
        let tiles_per_row = (tex.width() / TILE_SIZE) as u8;

        for y in 0..self.size.y {
            for x in 0..self.size.x {

                let idx = (y * self.size.x + x) as usize;
                let tile = self.tiles[idx];
                let tile_index = tile.frame;
                let tile_x = tile_index % tiles_per_row;
                let tile_y = tile_index / tiles_per_row;

                let mut xo = 0;
                let mut  yo = 0;
                let mut rot = 0.0;
                let mut color = WHITE;
                let modifier = tile.modifier;

                if (modifier > 0) {
                    // Rotation: Bit 1 & 2
                    if (modifier & 1) == 1 {
                        rot += rad_90;
                    }
                    if (modifier & 2) == 2 {
                        rot += rad_180;
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
}