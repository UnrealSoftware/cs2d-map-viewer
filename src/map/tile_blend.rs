use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::map::map::Map;

#[derive(Debug, PartialEq, Clone)]
pub struct TileBlend {
    blend_frame: u8,
    tile1: u8,
    tile2: u8,
    pub texture: Texture2D
}

pub fn tile_blend_init(map: &mut Map, assets: &Assets) {
    map.tile_blend.clear();

    for y in 0..map.size.y as usize {
        for x in 0..map.size.x as usize {
            tile_blend_update_tile(x, y, map, assets);
        }
    }
}

pub fn tile_blend_update_tile(x: usize, y: usize, map: &mut Map, assets: &Assets) {
    let idx = y * map.size.x as usize + x;
    let tile = &map.tiles[idx];

    if (tile.modifier & (64 + 128)) != 64 { return; }

    let blend_frame = map.modifiers[idx].frame;
    let current_tile_frame = tile.frame;
    let mut neighbor_tile_frame = 0;

    let offset:IVec2 = match blend_frame % 8 {
        0 => ivec2(0, -1),  // Top
        1 => ivec2(1, -1),  // Top Right
        2 => ivec2(1, 0),   // Right
        3 => ivec2(1, 1),   // Bottom Right
        4 => ivec2(0, 1),   // Bottom
        5 => ivec2(-1, 1),  // Bottom Left
        6 => ivec2(-1, 0),  // Left
        7 => ivec2(-1, -1), // Top Left
        _ => ivec2(0, 0),
    };

    let neighbor_pos = ivec2(x as i32, y as i32) + offset;

    if map.is_in_bounds(neighbor_pos) {
        let neighbor_idx = neighbor_pos.y as usize * map.size.x as usize + neighbor_pos.x as usize;
        neighbor_tile_frame = map.tiles[neighbor_idx].frame;
    }

    if current_tile_frame != neighbor_tile_frame {
        map.modifiers[idx].blend = tile_blend_add(blend_frame, current_tile_frame, neighbor_tile_frame, map, assets);
    } else {
        // Same tiles -> Blending pointless, strip the modifier flag (64)
        map.tiles[idx].modifier &= !64;
    }
}

pub fn tile_blend_add(blend: u8, tile1: u8, tile2: u8, map: &mut Map, assets: &Assets) -> u8 {
    // Find existing blend texture
    for i in 0..map.tile_blend.len() {
        let tb = &map.tile_blend[i];
        if tb.blend_frame == blend && tb.tile1 == tile1 && tb.tile2 == tile2 {
            return i as u8;
        }
    }

    // Create new blend texture
    let tileset = map.tile_texture.as_ref().unwrap();
    let img1 = tileset.extract_frame_image(tile1 as u16);
    let img2 = tileset.extract_frame_image(tile2 as u16);
    let blend_img = assets.blend_map.extract_frame_image(blend as u16);

    let w = img1.width as usize;
    let h = img1.height as usize;
    let multiplier = w / 32;

    let mut out_img = Image::gen_image_color(w as u16, h as u16, BLANK);

    for y in 0..h {
        for x in 0..w {
            let col1 = img1.get_pixel(x as u32, y as u32);
            let col2 = img2.get_pixel(x as u32, y as u32);
            let gradient_col = blend_img.get_pixel((x / multiplier) as u32, (y / multiplier) as u32);

            let f2 = gradient_col.r;
            let f1 = 1.0 - f2;

            let blended_color = Color::new(
                col1.r * f1 + col2.r * f2,
                col1.g * f1 + col2.g * f2,
                col1.b * f1 + col2.b * f2,
                col1.a * f1 + col2.a * f2,
            );

            out_img.set_pixel(x as u32, y as u32, blended_color);
        }
    }

    let texture = Texture2D::from_image(&out_img);
    let b = TileBlend {
        blend_frame: blend,
        tile1,
        tile2,
        texture
    };

    map.tile_blend.push(b);
    if map.tile_blend.len() >= u8::MAX as usize {
        warn!("tile blend variations exceed limit of {}", u8::MAX);
    }

    (map.tile_blend.len() - 1) as u8
}