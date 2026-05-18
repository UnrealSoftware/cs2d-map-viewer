use std::io;
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use macroquad::prelude::*;
use crate::assets::assets::Assets;
use crate::map::map::Map;
use crate::map::tile::Tile;
use crate::map::entity::Entity;
use crate::map::entity_type::EntityType;
use crate::map::tile_mode::TileMode;
use crate::map::tile_modifiers::TileModifiers;
use crate::paths::{PATH_BACKGROUNDS, PATH_TILES};
use crate::TILE_SIZE;
use crate::util::io::read_string;
use crate::util::rgb::Rgb;
use crate::util::texture_sheet::TextureSheet;

pub async fn read_map_file(path: &str, map: &mut Map, assets: &mut Assets) -> io::Result<()> {
    let bytes = assets.loader.load_file(path).await.unwrap();
    let mut reader = Cursor::new(bytes);
    read_map_bytes(&mut reader, path, map, assets).await
}

/// Reads and parses the binary map format from any `Read` source (like a File)
/// Specs https://www.unrealsoftware.de/files_pub/cs2d_spec_map_format.txt
pub async fn read_map_bytes<R: Read>(mut reader: R, path: &str, map: &mut Map, assets: &mut Assets) -> io::Result<()> {
    type E = LittleEndian;

    // --- (1) HEADER

    // Header
    let header = read_string(&mut reader)?;
    if !header.starts_with("Unreal Software's CS2D Map File") &&
        !header.starts_with("Unreal Software's Counter-Strike 2D Map File") {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid map header string"));
    }

    // 10 bytes for map settings / info
    let scroll_background_like_tiles = reader.read_u8()?;
    let use_tile_modifiers = reader.read_u8()? > 0;
    let save_tile_heights = reader.read_u8()?;
    let use_64_pixel_tiles = reader.read_u8()? > 0;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    map.header.has_modifiers = use_tile_modifiers;
    map.header.use_64_pixel_tiles = use_64_pixel_tiles;
    map.background.scroll_like_tiles = scroll_background_like_tiles > 0;

    // 10 ints for map settings / info
    let uptime_ms = reader.read_i32::<E>()?;
    let usgn_id = reader.read_i32::<E>()?;
    let daylight_time = (reader).read_i32::<E>()?;
    _ = reader.read_i32::<E>()?;
    _ = reader.read_i32::<E>()?;
    _ = reader.read_i32::<E>()?;
    _ = reader.read_i32::<E>()?;
    _ = reader.read_i32::<E>()?;
    _ = reader.read_i32::<E>()?;
    _ = reader.read_i32::<E>()?;

    // 10 strings for map settings / info
    let author_name = read_string(&mut reader)?;
    let tool_name = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;
    _ = read_string(&mut reader)?;

    // More map settings
    let control_string = read_string(&mut reader)?;
    let tileset_filename = read_string(&mut reader)?;
    let tile_count = reader.read_u8()?;
    let width = reader.read_i32::<E>()? + 1;
    let height = reader.read_i32::<E>()? + 1;
    let bg_filename = read_string(&mut reader)?;
    let bg_scroll_x = reader.read_i32::<E>()?;
    let bg_scroll_y = reader.read_i32::<E>()?;
    let bg_color_r = reader.read_u8()?;
    let bg_color_g = reader.read_u8()?;
    let bg_color_b = reader.read_u8()?;

    let mut tile_path = String::from(PATH_TILES);
    tile_path.push_str(&tileset_filename);
    let tex = assets.loader.load_texture(&tile_path).await.unwrap();
    map.tile_texture = Option::from(TextureSheet::new(tex, vec2(TILE_SIZE, TILE_SIZE)));

    if !bg_filename.is_empty() {
        let mut bg_path = String::from(PATH_BACKGROUNDS);
        bg_path.push_str(&bg_filename);
        map.background.texture = Some(assets.loader.load_texture(&bg_path).await.unwrap());
    }
    map.background.scroll_speed = IVec2::new(bg_scroll_x, bg_scroll_y);
    map.background.color = Color::from_rgba(bg_color_r, bg_color_g, bg_color_b, 255);

    // Header Test
    let header_test = read_string(&mut reader)?;
    if header_test != "ed.erawtfoslaernu" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid post map header string"));
    }

    let tile_it_count = tile_count + 1;

    // --- (2) TILE MODES
    map.tile_modes = vec![TileMode::default(); tile_it_count as usize];
    map.tile_heights = vec![0; tile_it_count as usize];
    map.tile_3d_modifiers = vec![0; tile_it_count as usize];
    for i in 0..tile_it_count {
        let tile_mode = reader.read_u8()?;
        map.tile_modes[i as usize] = tile_mode.into();
    }

    // --- (3) TILE HEIGHTS
    if save_tile_heights == 1 {
        for i in 0..tile_it_count {
            let tile_height = reader.read_i32::<E>()?;
            map.tile_heights[i as usize] = tile_height as u16;
        }
    } else if save_tile_heights == 2 {
        for i in 0..tile_it_count {
            let tile_height = reader.read_u16::<E>()?;
            let tile_modifier = reader.read_u8()?;
            map.tile_heights[i as usize] = tile_height;
            map.tile_3d_modifiers[i as usize] = tile_modifier;
        }
    }

    // --- (4) MAP
    let size = (width * height) as usize;
    map.size = U16Vec2::new(width as u16, height as u16);
    map.tiles = vec![Tile::default(); size];
    map.modifiers = vec![TileModifiers::default(); size];
    map.shadows = vec![0; size];
    map.entity_areas = vec![0; size];

    for x in 0..width {
        for y in 0..height {
            let tile_frame = reader.read_u8()?;
            map.tiles[(y * width + x) as usize] = Tile {
                frame: tile_frame,
                ..Default::default()
            };
        }
    }

    if use_tile_modifiers {
        for x in 0..width {
            for y in 0..height {
                let tile_modifier = reader.read_u8()?;
                let idx = (y * width + x) as usize;
                map.tiles[idx].modifier = tile_modifier;

                let has64 = tile_modifier & 64 != 0;
                let has128 = tile_modifier & 128 != 0;
                if has64 || has128 {
                    if has64 && has128 {
                        _ = read_string(&mut reader)?;
                    } else if has64 && !has128 {
                        let frame = reader.read_u8()?;
                        map.modifiers[idx].frame = frame;
                    } else {
                        let r = reader.read_u8()?;
                        let g = reader.read_u8()?;
                        let b = reader.read_u8()?;
                        let overlay = reader.read_u8()?;
                        map.modifiers[idx].rgb = Rgb::new(r, g, b);
                        map.modifiers[idx].frame = overlay;
                    }
                }
            }
        }
    }

    // --- (5) ENTITIES
    let entity_count = reader.read_i32::<E>()?;
    map.entities = vec![Entity::default(); entity_count as usize];
    for i in 0..entity_count {
        let entity_name = read_string(&mut reader)?;
        let entity_type = reader.read_u8()?;
        let entity_x = reader.read_i32::<E>()?;
        let entity_y = reader.read_i32::<E>()?;
        let entity_triggers = read_string(&mut reader)?;

        let mut ints: [i32; 10] = Default::default();
        let mut strings: [String; 10] = Default::default();
        for i in 0..10 {
            let entity_int = reader.read_i32::<E>()?;
            let entity_str = read_string(&mut reader)?;
            ints[i] = entity_int;
            strings[i] = entity_str;
        }

        let mut entity = Entity::new(
            EntityType::from(entity_type),
            IVec2::new(entity_x, entity_y),
            entity_name,
            entity_triggers,
            ints,
            strings
        );

        entity.setup(assets).await;
        map.entities[i as usize] = entity;
    }

    // --- (6) END OF FILE

    map.path = path.to_owned();

    map.map_update(true, true, true, None);

    Ok(())
}