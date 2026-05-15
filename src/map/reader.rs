use std::io;
use std::io::{Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};
use macroquad::file::load_file;
use macroquad::math::U16Vec2;
use crate::map::map::Map;
use crate::map::tile::Tile;
use crate::map::tile_modifiers::TileModifiers;
use crate::util::io::read_string;
use crate::util::rgb::Rgb;

pub async fn read_map_file(path: &str, map: &mut Map ) -> io::Result<()> {
    let bytes = load_file(path).await.unwrap();
    let mut reader = Cursor::new(bytes);
    read_map_bytes(&mut reader, path, map)
}

/// Reads and parses the binary map format from any `Read` source (like a File)
/// Specs https://www.unrealsoftware.de/files_pub/cs2d_spec_map_format.txt
pub fn read_map_bytes<R: Read>(mut reader: R, path: &str, map: &mut Map) -> io::Result<()> {
    type E = LittleEndian;

    // --- (1) HEADER

    // Header
    let header = read_string(&mut reader)?;
    if !header.starts_with("Unreal Software's CS2D Map File") &&
        !header.starts_with("Unreal Software's Counter-Strike 2D Map File") {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid map header string"));
    }

    // 10 bytes for map settings / info
    let scroll_map_like_tiles = reader.read_u8()?;
    let use_tile_modifiers = reader.read_u8()? == 1;
    let save_tile_heights = reader.read_u8()?;
    let use_64_pixel_tiles = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;
    _ = reader.read_u8()?;

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
    let background_filename = read_string(&mut reader)?;
    let background_scroll_x = reader.read_i32::<E>()?;
    let background_scroll_y = reader.read_i32::<E>()?;
    let background_color_r = reader.read_u8()?;
    let background_color_g = reader.read_u8()?;
    let background_color_b = reader.read_u8()?;

    // Header Test
    let header_test = read_string(&mut reader)?;
    if (header_test != "ed.erawtfoslaernu") {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid post map header string"));
    }

    let tile_it_count = tile_count + 1;

    // --- (2) TILE MODES
    for i in 0..tile_it_count {
        let tile_mode = reader.read_u8();
    }

    // --- (3) TILE HEIGHTS
    if (save_tile_heights == 1) {
        for i in 0..tile_it_count {
            let tile_height = reader.read_i32::<E>()?;
        }
    } else if (save_tile_heights == 2) {
        for i in 0..tile_it_count {
            let tile_height = reader.read_u16::<E>()?;
            let tile_modifier = reader.read_u8()?;
        }
    }

    // --- (4) MAP
    map.size = U16Vec2::new(width as u16, height as u16);
    map.tiles = vec![Tile::default(); (width * height) as usize];
    map.modifiers = vec![TileModifiers::default(); (width * height) as usize];

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
                        map.modifiers[idx].overlay = overlay;
                    }
                }
            }
        }
    }

    // --- (5) ENTITIES
    let entity_count = reader.read_i32::<E>()?;
    for _ in 0..entity_count {
        let entity_name = read_string(&mut reader)?;
        let entity_type = reader.read_u8()?;
        let entity_x = reader.read_i32::<E>()?;
        let entity_y = reader.read_i32::<E>()?;
        let entity_triggers = read_string(&mut reader)?;

        for i in 0..10 {
            let entity_int = reader.read_i32::<E>()?;
            let entity_str = read_string(&mut reader)?;
        }
    }

    // --- (6) END OF FILE

    map.path = path.to_owned();


    Ok(())

    /*
    // 3. Read Tile Information
    let total_tiles = (width * height) as usize;
    let mut tiles = Vec::with_capacity(total_tiles);

    for _ in 0..total_tiles {
        // Adjust byte sizes according to the .txt file specification
        let tile_id = reader.read_u8();
        let modifier = reader.read_u8();

        tiles.push(Tile { tile_id, modifier });
    }

    // 4. Read Entities
    let entity_count = reader.read_u32::<LittleEndian>()?;
    let mut entities = Vec::with_capacity(entity_count as usize);

    for _ in 0..entity_count {
        let entity_type = reader.read_u8()?;
        let x = reader.read_u32::<LittleEndian>()?;
        let y = reader.read_u32::<LittleEndian>()?;

        // Often entities in CS2D have a fixed or variable amount of attribute parameters (like trigger IDs, target IDs, etc.)
        let attr_count = reader.read_u8()?;
        let mut attributes = Vec::with_capacity(attr_count as usize);
        for _ in 0..attr_count {
            attributes.push(reader.read_i32::<LittleEndian>()?);
        }

        // If entities contain strings (like Lua script names or target names),
        // you would read the string length, then the string bytes here.

        entities.push(Entity::new(entity_type, x, y, attributes));
    }
    */
}